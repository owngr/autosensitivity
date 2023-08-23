use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None, arg_required_else_help = true)]
struct Args {
    /// Save config
    #[arg(short, long)]
    save: Option<String>,

    /// Load config
    #[arg(short, long, action)]
    load: bool
}

fn main() {
    let args = Args::parse();


    if let Some(save) = args.save{
        println!("Saving config {}", save);
        save_config(save);
    }

    if args.load {
        println!("Loading file")
    }
}

fn save_config(input_name: String) {

    let output = Command::new("xinput")
        .arg("list")
        .arg("--id-only")
        .arg(format!("pointer:{}", input_name))
        .output()
        .expect("failed to run xinput, is it installed ?");
    println!("output: {}!", String::from_utf8_lossy(&output.stdout));
    
    // let mut config_file = File::create(input_name)?;
    // file.write_all(accel_speed)
}
