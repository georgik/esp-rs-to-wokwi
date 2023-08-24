use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "SPI Parser", about = "Parse Rust source code for SPI pin mapping.")]
pub struct Opt {
    /// The path to the Rust source code file.
    #[structopt(short, long, parse(from_os_str))]
    pub file_path: std::path::PathBuf,
}