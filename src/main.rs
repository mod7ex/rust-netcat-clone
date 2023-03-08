mod client;
mod server;

// use std::env;

async fn run() -> Result<(), String> {
    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();

    tokio::spawn(async move {
        tokio::io::copy(&mut stdin, &mut stdout).await.unwrap();
    }).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await.unwrap();
    
    /*
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
            println!("The first argument is {}", args[1]);
        }
    */
}
