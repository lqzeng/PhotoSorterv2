mod core;
mod pipeline;

use clap::Parser;
use pipeline::scan::scan_images;

#[derive(Parser)]
struct Args {
    path: String
}

fn main() {
    let args = Args::parse();

    let images = scan_images(&args.path);

    println!("Found {} images", images.len());

    for img in images.iter().take(10) {
        println!("{}", img.display());
    }
}