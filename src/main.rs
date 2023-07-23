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
    brightness_path: String,
    #[arg(long)]
    max_brightness: Option<bool>,
    #[arg(long)]
    brightness: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let current_brightness =
        filesystem::current_brightness_level(&format!("{}/brightness", &args.brightness_path))
            .expect("Could not get the current brightness level of your device");

    if let Some(increase) = args.increase {
        set_brightness(current_brightness + increase, &args.brightness_path)
            .expect(&format!("Could not increase your brightness by {increase}"));
    }

    if let Some(decrease) = args.decrease {
        set_brightness(current_brightness - decrease, &args.brightness_path)
            .expect(&format!("Could not increase your brightness by {decrease}"));
    }

    if let Some(_) = args.max_brightness {
        println!(
            "Max brightness: {}",
            filesystem::max_brightness_level(&format!("{}/max_brightness", &args.brightness_path))
                .expect("Could not get the maximum brightness")
        );
    }
    if let Some(value) = args.brightness {
        set_brightness(value, &args.brightness_path)
            .expect(&format!("Could not set brightness level to {value}"));
    }
}
