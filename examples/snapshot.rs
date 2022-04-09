mod include;
extern crate rbspy;

use rbspy::recorder::snapshot;
use crate::include::path_to_ruby_binary;

fn main() {
    let mut process = std::process::Command::new(path_to_ruby_binary())
        .arg("ci/ruby-programs/infinite.rb")
        .spawn()
        .unwrap();
    let pid = process.id() as rbspy::Pid;

    match snapshot(pid, true) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Failed to get snapshot: {:?}", e),
    }

    process.kill().expect("couldn't clean up ruby process");
}
