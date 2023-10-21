use std::thread;
use structopt::StructOpt;

mod keygen;

#[derive(StructOpt)]
struct Options {
    #[structopt(long, short)]
    oem: bool,
    #[structopt(long, short, default_value = "1")]
    quantity: u32,
}

fn main() {
    let options = Options::from_args();

    let mut threads = vec![];
    for _ in 0..options.quantity {
        let thread_handle = if options.oem {
            thread::spawn(|| keygen::generate_oem_key())
        } else {
            thread::spawn(|| keygen::generate_retail_key())
        };
        threads.push(thread_handle);
    }

    let keys: Vec<_> = threads.into_iter().map(|handle| handle.join().unwrap()).collect();

    for (_i, key) in keys.iter().enumerate() {
        if options.oem {
            println!("{}", key);
        } else {
            println!("{}", key);
        }
    }
}

