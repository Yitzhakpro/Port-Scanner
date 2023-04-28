use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PortScannerArgs {
    /// IP of the host
    #[arg(long)]
    pub ip: String,

    /// list of ports to scan separated by: ,
    #[arg(short, long, value_delimiter = ',')]
    pub ports: Vec<u16>,

    /// should scan udp protocol
    #[arg(long, default_value_t= false)]
    pub udp: bool,

    /// output file
    #[arg(short, long, default_value = "output.txt")]
    pub output: String,

    /// output more info of the scan
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
