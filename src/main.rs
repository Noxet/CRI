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

    let Some((dhost, dport)) = args.dispatcher_server.split_once(":") else { panic!("Could not read server:port") };
    

    println!("host: {}, port: {}", dhost, dport);

    let output = Command::new("src/update_repo.sh").arg(&args.repo).output().expect("Fuggin failed lol");

    println!("{:?}", String::from_utf8(output.stdout));
}
