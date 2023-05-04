use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::{thread, time};

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

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_req);
}
