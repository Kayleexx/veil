use std::io::{self, Write};
use veil::network::{start_server, start_client};
use tokio::runtime::Runtime;

fn main() {
    // Use a runtime because our network code is async
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    println!("\n================================");
    println!("          Welcome to Veil       ");
    println!("================================\n");

    println!("Choose a mode:");
    println!("  server / node  -> Run as a server node");
    println!("  client         -> Connect as a client");
    print!("\n> ");
    io::stdout().flush().unwrap();

    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();

    if mode == "server" || mode == "node" {
        print!("\nEnter the address to bind (e.g., 127.0.0.1:7878): ");
        io::stdout().flush().unwrap();

        let mut addr = String::new();
        io::stdin().read_line(&mut addr).unwrap();
        let addr = addr.trim();

        println!("\nStarting server on {}...\n", addr);
        rt.block_on(async {
            if let Err(e) = start_server(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

    } else if mode == "client" {
        print!("\nEnter the server address to connect to (e.g., 127.0.0.1:7878): ");
        io::stdout().flush().unwrap();

        let mut addr = String::new();
        io::stdin().read_line(&mut addr).unwrap();
        let addr = addr.trim();

        println!("\nConnected to {}\n", addr);

        loop {
            println!("-----------------------------------");
            println!("Enter a command:");
            println!("  aes|text                -> Encrypt text with AES");
            println!("  hash|text               -> Hash text");
            println!("  reverse|text            -> Reverse text");
            println!("  split|secret|k|n        -> Secret sharing split");
            println!("  combine|share1,share2   -> Combine secret shares");
            println!("  mpc|party1,party2|...   -> Run MPC operation");
            println!("(Press Enter with no input to skip)");
            print!("\n> ");
            io::stdout().flush().unwrap();

            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).unwrap();
            let cmd = cmd.trim();

            if cmd.is_empty() {
                continue;
            }

            let addr = addr.to_string();
            let cmd = cmd.to_string();

            rt.block_on(async {
                match start_client(&addr, &cmd).await {
                    Ok(response) => {
                        println!("\nServer response: {}\n", response);
                    }
                    Err(e) => {
                        eprintln!("Client error: {:?}", e);
                    }
                }
            });
        }

    } else {
        eprintln!("\nUnknown mode: '{}'. Please enter 'server' or 'client'.\n", mode);
    }
}
