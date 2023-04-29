use clap::Parser;
use std::process::Command;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    // Dispatcher server socket
    #[arg(short, long, default_value = "localhost:8888")]
    dispatcher_server: String,
    
    // Repository path
    #[arg(short, long)]
    repo: String,
}


fn main() {
    let args = Args::parse();

    println!("{}", args.dispatcher_server);
    dbg!(args.repo);

    let output = Command::new("ech").arg("Hello").output().expect("Fuggin failed lol");

    println!("{:?}", output.stdout);
}
