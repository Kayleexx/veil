use veil::network::{start_client, start_server};
use veil::VeilError;

fn main() {
    start_node();
}

fn start_node() {
    println!("Do you want to run as server or client?");
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim();

    if mode == "server" {
        println!("Starting server...");
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            start_server("127.0.0.1:7878").await.unwrap();
        });
    } else if mode == "client" {
        let rt = tokio::runtime::Runtime::new().unwrap();
        loop {
            println!("\nEnter command:");
            println!("Commands: hash|text | reverse|text | aes|text | split|secret|n|k | combine|share1,share2,...");
            println!("Type 'exit' to quit.");
            print!("> ");
            use std::io::Write;
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input.eq_ignore_ascii_case("exit") {
                println!("Exiting client.");
                break;
            }

            if let Err(e) = rt.block_on(start_client("127.0.0.1:7878", input)) {
                eprintln!("Client error: {}", e);
            } else {
                println!("Message processed successfully.");
            }
        }
    } else {
        println!("Invalid mode");
    }
}
