
use std::process::Command;
use some_lib::*;
use some_lib_functions::*;

fn os_commands_example_1() {
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
                    .args(["/C", "echo aye there from windows!"])
                    .output()
                    .expect("Failed to execute command.")
    } else{
        Command::new("sh")
                    .args(["-c", "echo ahoy there from Linux!"])
                    .output()
                    .expect("Failed to execute command.")
    };
    println!("\n\n");
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte response.");
    println!("{}", cmd_output);
    println!("\n\n");
}

// Linux only
fn error_handling_example(dir: &str) {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    list_cmd.current_dir(dir).status().expect("Failed to execute list command.");

    println!("\n\n");
}

fn main() {
    os_commands_example_1();
    error_handling_example("src");
    //error_handling_example("lib");
    whats_up();
    nothing_much();
}                                                                                                               