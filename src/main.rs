use clap::Parser;
use rayon::prelude::*;

mod keygen;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    oem: bool,

    #[arg(short, long, default_value = "1")]
    number: usize,
}

fn main() {
    let options = Args::parse();
    let quantity = options.number;

    let keys: Vec<String> = (0..quantity)
        .into_par_iter()
        .map(|_| {
            if options.oem {
                keygen::KeyGenerator::generate_oem_key()
            } else {
                keygen::KeyGenerator::generate_retail_key()
            }
        })
        .collect();

    for key in keys {
        println!("{}", key);
    }
}
