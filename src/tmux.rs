use std::process::Command;

const WINDOW_NAME: &str = "zsh";
const FIRST_WINDOW_NAME: &str = "nvim";

fn create_tmux_window() {
    let mut command = Command::new("tmux");
    command.arg("new-window");
    command.arg("-n");
    command.arg(WINDOW_NAME);
    command.arg("-d");

    match command.output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("New Window Error: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn create_session(session_name: &str, window_amount: u8) {
    let mut command = Command::new("tmux");
    command.arg("new-session");
    command.arg("-s");
    command.arg(session_name);
    command.arg("-n");
    command.arg(FIRST_WINDOW_NAME);
    command.arg("-d");

    match command.output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("New Session Error: {}", e);
            std::process::exit(1);
        }
    }

    for _ in 1..window_amount {
        create_tmux_window();
    }
}

pub fn goto_first_window(session_name: &str) {
    let mut command = Command::new("tmux");
    command.arg("select-window");
    command.arg("-t");
    command.arg(format!("{}:0", session_name));

    match command.output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Select Window Error: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
