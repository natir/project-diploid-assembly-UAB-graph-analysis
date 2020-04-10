extern crate csv;
extern crate clap;
extern crate niffler;

use std::io::Write;

fn main() {
    let matches = clap::App::new("separate_by_chr")
	.arg(clap::Arg::with_name("assembly")
	     .short("a")
	     .long("assembly")
	     .required(true)
	     .takes_value(true)
	     .help("assembly")
	)
	.arg(clap::Arg::with_name("prefix")
	     .short("p")
	     .long("prefix")
	     .required(true)
	     .takes_value(true)
	     .help("prefix of file generate")
	)
	.arg(clap::Arg::with_name("min_percent")
	     .short("m")
	     .long("min-percent")
	     .default_value("0.7")
	     .takes_value(true)
	     .help("minimal percent to assign tig/read to a chromosome")
	)
	.get_matches();

    let (asm, _) = niffler::get_reader(
	Box::new(std::io::BufReader::new(
	    std::fs::File::open(matches.value_of("assembly").unwrap()).unwrap())
	)	 
    ).unwrap();

    let mut chr2file: std::collections::HashMap<String, bio::io::fasta::Writer<std::fs::File>> = std::collections::HashMap::new();
    let asm_reader = bio::io::fasta::Reader::new(asm);

    let prefix = matches.value_of("prefix").unwrap();
    for i in 1..26 {
	chr2file.insert(
	    format!("cluster{}", i),
	    bio::io::fasta::Writer::new(
		std::fs::File::create(
		    format!("{}cluster{}.fasta", prefix, i)
		).unwrap()
	    )
	);
    }
    
    for result in asm_reader.records() {
	let record = result.unwrap();

	let cluster = record.id().split('_').next().unwrap().to_string();

	chr2file.get_mut(&cluster).unwrap().write_record(&record);
	//writer.write_record(&record);
    }
}
