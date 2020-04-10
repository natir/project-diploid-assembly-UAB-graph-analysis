/* standard use */
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};

/* crate use */
use flate2;
use bzip2;
use xz2;
use enum_primitive::{
    enum_from_primitive, enum_from_primitive_impl, enum_from_primitive_impl_ty, FromPrimitive,
};
use failure::{Error, Fail};

enum_from_primitive! {
    #[repr(u64)]
    #[derive(Debug, PartialEq)]
    pub enum CompressionFormat {
        Gzip = 0x1F8B,
        Bzip = 0x425A,
        Lzma = 0x00FD_377A_585A,
        No,
    }
}

#[derive(Debug, Fail)]
pub enum OCFError {
    #[fail(display = "Feature disabled, enabled it during compilation")]
    FeatureDisabled,
}

pub fn get_readable_file(
    input_name: &str,
) -> Result<(Box<dyn io::Read>, CompressionFormat), Error> {
    let raw_input = get_readable(input_name);

    // check compression
    let compression = get_compression(raw_input);

    // return readable and compression status
    match compression {
        CompressionFormat::Gzip => Ok((
            Box::new(flate2::read::GzDecoder::new(get_readable(input_name))),
            CompressionFormat::Gzip,
        )),
        CompressionFormat::Bzip => Ok((
            Box::new(bzip2::read::BzDecoder::new(get_readable(input_name))),
            CompressionFormat::Bzip,
        )),
        CompressionFormat::Lzma => Ok((
            Box::new(xz2::read::XzDecoder::new(get_readable(input_name))),
            CompressionFormat::Lzma,
        )),
        CompressionFormat::No => Ok((Box::new(get_readable(input_name)), CompressionFormat::No)),
    }
}

pub fn get_readable(input_name: &str) -> Box<dyn io::Read> {
    match input_name {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(
            File::open(input_name)
                .unwrap_or_else(|_| panic!("Can't open input file {}", input_name)),
        )),
    }
}

fn get_compression(mut in_stream: Box<dyn io::Read>) -> CompressionFormat {
    let mut buf = vec![0u8; 5];

    in_stream
        .read_exact(&mut buf)
        .expect("Error durring reading first bit of file");

    let mut five_bit_val: u64 = 0;
    for (i, item) in buf.iter().enumerate().take(5) {
        five_bit_val |= (u64::from(*item)) << (8 * (4 - i));
    }
    if CompressionFormat::from_u64(five_bit_val) == Some(CompressionFormat::Lzma) {
        return CompressionFormat::Lzma;
    }

    let mut two_bit_val: u64 = 0;
    for (i, item) in buf.iter().enumerate().take(2) {
        two_bit_val |= (u64::from(*item)) << (8 * (1 - i));
    }

    match CompressionFormat::from_u64(two_bit_val) {
        e @ Some(CompressionFormat::Gzip) | e @ Some(CompressionFormat::Bzip) => e.unwrap(),
        _ => CompressionFormat::No,
    }
}

pub fn get_output(
    output_name: &str,
    format: CompressionFormat,
) -> Result<Box<dyn io::Write>, Error> {
    match format {
        CompressionFormat::Gzip => Ok(
            Box::new(
                flate2::write::GzEncoder::new(get_writable(output_name),
                                              flate2::Compression::best()
                )
            )
        ),
        CompressionFormat::Bzip => Ok(
            Box::new(
                bzip2::write::BzEncoder::new(get_writable(output_name),
                                             bzip2::Compression::Best
                )
            )
        ),
        CompressionFormat::Lzma => Ok(
            Box::new(
                xz2::write::XzEncoder::new(get_writable(output_name),
                                           9)
            )
        ),
        CompressionFormat::No => Ok(Box::new(get_writable(output_name))),
    }
}

pub fn choose_compression(
    input_compression: CompressionFormat,
    compression_set: bool,
    compression_value: &str,
) -> CompressionFormat {
    if !compression_set {
        return input_compression;
    }

    match compression_value {
        "gzip" => CompressionFormat::Gzip,
        "bzip2" => CompressionFormat::Bzip,
        "lzma" => CompressionFormat::Lzma,
        _ => CompressionFormat::No,
    }
}

fn get_writable(output_name: &str) -> Box<dyn io::Write> {
    match output_name {
        "-" => Box::new(BufWriter::new(io::stdout())),
        _ => Box::new(BufWriter::new(
            File::create(output_name)
                .unwrap_or_else(|_| panic!("Can't open output file {}", output_name)),
        )),
    }
}
