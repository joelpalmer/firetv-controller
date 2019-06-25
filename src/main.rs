extern crate subprocess;
use std::env;
use std::fmt::Error;
use subprocess::Exec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    Exec::cmd("adb").arg("connect").arg(ip).join();
    // ask the user what button they wish to virtually press
    loop {
        println!("What key would you like to press? (q to quit)");
        let mut key_name = String::new();

        // capture their answer as string or maybe keypress
        // string for now
        std::io::stdin()
            .read_line(&mut key_name)
            .expect("Can't read line");
        // give means to quit
        // kill adb server when they are done
        if let "q" = &*key_name.trim() {
            Exec::cmd("adb").arg("kill-server").join();
            break;
        }
        // pass along their wishes to adb.
        handle_command(&key_name);
        // rinse-repeat
    }
}

fn handle_command(key_name: &str) -> bool {
    match key_name.trim() {
        "up" => Exec::cmd("adb")
            .arg("shell")
            .arg("input")
            .arg("keyevent")
            .arg("19")
            .join()
            .is_ok(),
        "down" => Exec::cmd("adb")
            .arg("shell")
            .arg("input")
            .arg("keyevent")
            .arg("20")
            .join()
            .is_ok(),
        "left" => Exec::cmd("adb")
            .arg("shell")
            .arg("input")
            .arg("keyevent")
            .arg("21")
            .join()
            .is_ok(),
        "right" => Exec::cmd("adb")
            .arg("shell")
            .arg("input")
            .arg("keyevent")
            .arg("22")
            .join()
            .is_ok(),
        // todo: enter, back, home, menu, play/pause
        _ => false,
    }
}
