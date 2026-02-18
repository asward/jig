use clap::Parser;
use mouse_rs::Mouse;
use rand::prelude::*;
use std::thread;
use std::time::Duration;
#[derive(Parser)]
struct Args {
    /// Idle timeout in seconds
    #[arg(short = 'i', long, env = "JIG_INTERVAL", default_value = "60")]
    timeout: Option<u64>,

    /// Threshold for random movement in pixels
    #[arg(short = 't', long, env = "JIG_THRESHOLD", default_value = "1")]
    threshold: Option<i32>,
}

fn main() {
    let args = Args::parse();
    let timeout = Duration::from_secs(args.timeout.unwrap());
    let threshold = args.threshold.unwrap();
    match run(timeout, threshold) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
fn run(timeout: Duration, threshold: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();

    let mouse = Mouse::new();
    loop {
        let pos = mouse.get_position()?;

        let dx: i32 = rng.random_range(-threshold..=threshold);
        let dy: i32 = rng.random_range(-threshold..=threshold);

        mouse.move_to(pos.x + dx, pos.y + dy)?;

        let res = mouse.get_position()?;

        let x_ok = res.x == pos.x + dx;
        let y_ok = res.y == pos.y + dy;

        if x_ok != y_ok {
            if !x_ok {
                mouse.move_to(pos.x - dx, pos.y + dy)?;
            } else {
                mouse.move_to(pos.x + dx, pos.y - dy)?;
            }
        }

        thread::sleep(timeout);
    }
}
