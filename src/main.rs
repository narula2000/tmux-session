use clap::Parser;
mod tmux;

const DEFAULT_WINDOW_AMOUNT: u8 = 2;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(short, long)]
    session_name: Option<String>,
    #[arg(short, long)]
    window_amount: Option<u8>,
}

fn get_session_name() -> String {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => match dir.to_str() {
            Some(dir_str) => dir_str.to_string(),
            None => {
                eprintln!("Error: Could not convert path to string");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match current_dir.split('/').last() {
        Some(session_name) => session_name.to_string(),
        None => {
            eprintln!("Error: Could not get session name");
            std::process::exit(1);
        }
    }
}
fn main() {
    let args = Cli::parse();

    let session_name = match args.session_name {
        Some(session_name) => session_name,
        None => get_session_name(),
    };

    let window_amount = match args.window_amount {
        Some(window_amount) => window_amount,
        None => DEFAULT_WINDOW_AMOUNT,
    };

    println!("Session name: {}", session_name);
    println!("Window amount: {}", window_amount);
    println!("tmux attach -t {}", session_name);

    tmux::create_session(&session_name, window_amount);
    tmux::goto_first_window(&session_name);
    std::process::exit(0);
}
