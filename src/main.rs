mod client;
mod server;

use server::run;

// use std::env;

fn main() {
    /*
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
            println!("The first argument is {}", args[1]);
        }
    */

    run("127.0.0.1", 3000).unwrap();
}
