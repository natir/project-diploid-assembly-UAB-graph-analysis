extern crate csv;
extern crate clap;
extern crate bzip2;
extern crate flate2;
extern crate xz2;
extern crate bio;

/* std use */
use std::io::BufRead;
use std::io::Write;

/* local use */
use phased_human;

fn main() {
    let matches = clap::App::new("filter_fastx")
        .arg(clap::Arg::with_name("map2asm")
             .short("m")
             .long("map2asm")
             .required(true)
             .takes_value(true)
             .help("path to mapping file")
        )
        .arg(clap::Arg::with_name("read2read")
             .short("r")
             .long("read2read")
             .required(true)
             .takes_value(true)
             .help("path to overlapping file")
        )
        .arg(clap::Arg::with_name("input")
             .short("i")
             .long("input")
             .required(true)
             .takes_value(true)
             .help("path where reads is read")
        )
        .arg(clap::Arg::with_name("output")
             .short("o")
             .long("output")
             .required(true)
             .takes_value(true)
             .help("path where select reads is write")
        )
        .arg(clap::Arg::with_name("assignation")
             .short("a")
             .long("assignation")
             .required(true)
             .takes_value(true)
             .help("path where assignation of reads is write")
        )
        .arg(clap::Arg::with_name("assemblies")
             .short("A")
             .long("assemblies")
             .required(true)
             .takes_value(true)
             .help("path where assemblies is read")
        )
        .arg(clap::Arg::with_name("distance")
             .short("d")
             .long("distance")
             .takes_value(true)
             .default_value("2500")
             .help("max distance to extremity")
        )
        .get_matches();

    let (map2asm, _) = phased_human::get_readable_file(matches.value_of("map2asm").unwrap()).unwrap();
    let (read2read, _) = phased_human::get_readable_file(matches.value_of("read2read").unwrap()).unwrap();
    let (input, compression) = phased_human::get_readable_file(matches.value_of("input").unwrap()).unwrap();
    let (assemblies, _) = phased_human::get_readable_file(matches.value_of("assemblies").unwrap()).unwrap();
    
    let mut output = std::io::BufWriter::new(phased_human::get_output(matches.value_of("output").unwrap(), compression).unwrap());
    let mut assignation = std::io::BufWriter::new(phased_human::get_output(matches.value_of("assignation").unwrap(), phased_human::CompressionFormat::No).unwrap());
    assignation.write_all(b"read,tig,tig_len\n");
    
    let dist: u64 = matches.value_of("distance").unwrap().parse::<u64>().unwrap();

    let (reads2tig_pos, tig2len) = read2tig_pos(map2asm);

    let asm_reader = bio::io::fasta::Reader::new(assemblies);
    for record in asm_reader.records() {
	let record = record.unwrap();
	write!(output, ">{} {}\n{}\n", record.id(), record.desc().unwrap_or(""), String::from_utf8_lossy(record.seq()));
    }

    let mut selected = select_read_by_dist(&reads2tig_pos, dist);
    selected = select_read_by_read_ovl(selected, read2read);
    
    let mut reads_reader = bio::io::fastq::Reader::new(input);
    for record in reads_reader.records() {
	let record = record.expect("Error in fasta parsing");
	if selected.contains(record.id()) {
	    if let Some((tig, _, _)) = reads2tig_pos.get(record.id()) {
		write!(assignation, "{},{},{}\n", record.id(), tig, *tig2len.get(tig).unwrap());
		write!(output, ">{}\n{}\n", record.id(), String::from_utf8_lossy(record.seq()));
	    }
	}
    }
}

fn read2tig_pos<R>(map2asm: R) -> (std::collections::HashMap<String, (String, u64, u64)>, std::collections::HashMap<String, u64>)
where R: std::io::Read
{
    let reader = std::io::BufReader::new(map2asm);
    let mut result = std::collections::HashMap::new();
    let mut tig2len = std::collections::HashMap::new();
    
    for l in reader.lines() {
        let line = l.unwrap();
        let row: Vec<&str> = line.split('\t').collect();

        let read_name = row[0].to_string();
        let read_len = row[1].parse::<u64>().unwrap();
        let tig_name = row[5].to_string();
        let tig_len = row[6].parse::<u64>().unwrap();
        let mut tig_beg = row[7].parse::<u64>().unwrap();
        let mut tig_end = row[8].parse::<u64>().unwrap();

	tig2len.entry(tig_name.clone()).or_insert(tig_len);
	
        if tig_beg > tig_end {
            std::mem::swap(&mut tig_beg, &mut tig_end);
        }
        
        let pos = tig_beg.min(tig_len - tig_end);

        if (tig_end - tig_beg) > (read_len as f64 * 0.7) as u64 {
	    let v = result.entry(read_name).or_insert((tig_name.clone(), pos, tig_end - tig_beg));
	    if v.2 < tig_end - tig_beg {
                *v = (tig_name, pos, tig_end - tig_beg); 
	    }
        }
    }

    return (result, tig2len);
}

fn select_read_by_dist(reads2tig_pos: &std::collections::HashMap<String, (String, u64, u64)>, dist: u64) -> std::collections::HashSet<String> {
    let mut selected_read = std::collections::HashSet::new();

    for (id, _) in reads2tig_pos {
	if let Some((tig, pos, _)) = reads2tig_pos.get(id) {
	    if pos > &dist {
		continue;
	    } else {
		selected_read.insert(id.clone());
	    }
	}
    }

    selected_read
}

fn select_read_by_read_ovl<R>(selected: std::collections::HashSet<String>, read2read: R) -> std::collections::HashSet<String>
where R: std::io::Read
{
    let reader = std::io::BufReader::new(read2read);
    let mut new_selected = std::collections::HashSet::new();
    
    for l in reader.lines() {
        let line = l.unwrap();
        let row: Vec<&str> = line.split('\t').collect();

        let query_name = row[0].to_string();
        let target_name = row[5].to_string();

	if selected.contains(&query_name) {
	    new_selected.insert(target_name.clone());
	}

	if selected.contains(&target_name) {
	    new_selected.insert(query_name.clone());
	}
    }

    selected.union(&new_selected).cloned().collect()
}
