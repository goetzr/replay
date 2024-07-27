use std::thread;
use std::time::Duration;
use clap::{Command, Arg};
use std::io;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
        process::exit(1);
    }
}
fn run() -> anyhow::Result<()> {
    static STOP: AtomicBool = AtomicBool::new(false);

    println!("[main] Sender");
    let args = parse_args();
    println!("[main] File path set to: {}", args.file_path);

    let th = thread::spawn(|| {
        let mut msg_num = 1;
        loop {
            if STOP.load(Relaxed) {
                break;
            }
            println!("[background] Sending message {msg_num}");
            msg_num += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    for line in io::stdin().lines() {
        let line = line?;
        match line.trim() {
            "help" => println!("[main] Commands: help, stop"),
            "stop" => {
                STOP.store(true, Relaxed);
                break;
            }
            cmd => println!("[main] Unhandled command: {cmd}"),
        };
    }

    th.join().unwrap();

    Ok(())
}

struct CliArgs {
    file_path: String,
}

impl CliArgs {
    fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

fn parse_args() -> CliArgs {
    let m = Command::new("sender")
        .author("Russ Goetz, russgoetz@gmail.com")
        .version("0.0.1")
        .about("Sends packets to the receiving application.")
        .arg(
            Arg::new("file_path")
                .short('f')
                .long("file")
                .value_name("FILE-PATH")
                .help("Full path to the file containing the packets to send.")
                .required(true)
        ).get_matches();

    CliArgs::new(m.get_one::<String>("file_path").unwrap().clone())
}