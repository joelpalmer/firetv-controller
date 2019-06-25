extern crate subprocess;
use std::env;
use std::io::{BufRead, BufReader};
use subprocess::Exec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    let x = Exec::cmd("adb").arg("connect").arg(ip).join();
}
