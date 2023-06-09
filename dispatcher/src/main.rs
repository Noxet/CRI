use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use regex::Regex;

use utils::ThreadPool;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Dispatcher server socket
    #[arg(short, long, default_value = "localhost:8888")]
    server: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.server);

    let listener = TcpListener::bind(args.server).unwrap();
    let pool = ThreadPool::new(16);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}

struct Runner {
    host: String,
    port: String,
}

struct Server {
    stream: TcpStream,
    runners: Vec<Runner>,
    commits: Vec<String>,
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut req = String::new();
    //buf_reader.read_line(&mut req);
    buf_reader.take(1024).read_line(&mut req).expect("Could not read data");
    println!("Request: {:#?}", req);

    /*
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
*/

    let re = Regex::new(r"(\w+):?(.+)*").unwrap();
    let cap = re.captures(req.as_str()).unwrap();
    println!("cap: {:#?}", cap.get(1));

    let cmd = cap.get(1).map_or("", |m| m.as_str());
    println!("cmd: {}", cmd);

    if cmd.eq("status") {
        println!("Got status");
        stream.write_all("OK".as_bytes()).unwrap();
    } else if cmd.eq("register") {
        println!("Got register");
        // TODO: add runner
        stream.write_all("OK".as_bytes()).unwrap();
    }
}
