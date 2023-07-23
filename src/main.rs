mod filesystem;

use clap::Parser;
use filesystem::set_brightness;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    increase: Option<u32>,
    #[arg(long)]
    decrease: Option<u32>,
    #[arg(long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let current_brightness = filesystem::current_brightness_level(&args.path).unwrap();
    if let Some(increase) = args.increase {
        set_brightness(current_brightness + increase, &args.path).unwrap();
    }

    if let Some(decrease) = args.decrease {
        set_brightness(current_brightness - decrease, &args.path).unwrap();
    }
}
