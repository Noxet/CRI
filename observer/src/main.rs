use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
use std::net::TcpStream;
use std::process::Command;
use std::{thread, time};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Dispatcher server socket
    #[arg(short, long, default_value = "localhost:8888")]
    dispatcher_server: String,

    // Repository path
    #[arg(short, long)]
    repo: String,
}

fn get_commit_id() -> Result<String, io::Error> {
    let mut f = match File::open(".commit_id") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut commit_id = String::new();

    match f.read_to_string(&mut commit_id) {
        Ok(_) => Ok(commit_id),
        Err(e) => Err(e),
    }
}

fn main() {
    let args = Args::parse();

    println!("{}", args.dispatcher_server);

    let Some((dhost, dport)) = args.dispatcher_server.split_once(':') else { panic!("Could not read server:port") };
    dbg!(dhost, dport);

    loop {
        let output = Command::new("src/update_repo.sh")
            .arg(&args.repo)
            .output()
            .expect("Could not run 'update_repo.sh'");
        println!("{:?}", String::from_utf8(output.stdout));

        let commit_id = match get_commit_id() {
            Ok(cid) => cid,
            Err(_) => {
                println!("Failed to get commit id, trying later...");
                // TODO: add sleep here
                thread::sleep(time::Duration::from_secs(5));
                continue;
            }
        };

        // TODO: maybe loop this if something fails, as we want to try with the current commit
        // before leaving it
        //for _ in 1..=3 {
        let mut resp = String::new();
        println!("commit id: {}", commit_id);
        if let Ok(mut stream) = TcpStream::connect(&args.dispatcher_server) {
            stream
                .write_all(b"status\n")
                .expect("Could not send status command to server");
            stream
                .read_to_string(&mut resp)
                .expect("Could not read status response from server");
            print!("server resp: {}", resp);
            if resp == "ok" {
                stream
                    .write_all(format!("dispatch:{}", commit_id).as_bytes())
                    .expect("Could not send commit ID to dispatch");
            }
        } else {
            println!("Could not connect to server, trying again later...");
            thread::sleep(time::Duration::from_secs(60));
            continue;
        }

        //}
    }
}
