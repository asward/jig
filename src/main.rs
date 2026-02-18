use clap::Parser;
use mouse_rs::Mouse;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    /// Idle timeout in seconds
    #[arg(short = 'i', long, env = "JIG_INTERVAL", default_value = "60")]
    timeout: Option<u64>,
}

fn main() {
    let args = Args::parse();
    let timeout = Duration::from_secs(args.timeout.unwrap());
    let mouse = Mouse::new();
    loop {
        match mouse.get_position() {
            Ok(position) => _ = mouse.move_to(position.x + 10, position.y + 10),
            Err(e) => {
                eprintln!("Error getting mouse position: {}", e);
                continue;
            }
        }
        thread::sleep(timeout);
    }
}
