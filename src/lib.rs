use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX: u16 = 65535; // max port number

pub struct Arguments {
    ipaddr: IpAddr,
    num_threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                ipaddr,
                num_threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!(
                    "Usage: 
    -n to select how many threads you want 
    -h or --help to show help information."
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("too many arguments");
            } else if flag.contains("-n") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IP address; must be IPv4 or IPv6"),
                };
                let num_threads = match args[2].parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Arguments {
                    ipaddr,
                    num_threads,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

pub fn run(args: &Arguments) {
    let num_threads = args.num_threads;
    let addr = args.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    drop(tx);
    for port in rx {
        println!("{} is open! 🐶", port);
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    // port 0 carries special significance
    // reference: https://www.lifewire.com/port-0-in-tcp-and-udp-818145#:~:text=Port%200%20is%20a%20wildcard,number%20zero%20up%20to%2065535.

    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if port >= MAX - num_threads {
            break;
        }

        port += num_threads;
    }
}
