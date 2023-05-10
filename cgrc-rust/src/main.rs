use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long = "list-locations")]
    pub list_locations: bool
}

fn main() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }

    let args = Cli::parse();
    if args.list_locations {
        println!("List locations");
        return;
    }

    println!("Hello, world!");
}
