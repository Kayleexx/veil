use tokio::runtime::Runtime;
use veil::VeilError;

fn start_node() -> Result<(), VeilError> {
    println!("Do you want to run as server or client?");
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim();

    if mode == "server" {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { veil::network::start_server("127.0.0.1:7878").await })
    } else if mode == "client" {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                println!("\nEnter command:");
                println!("Commands: hash|text | reverse|text | aes|text | split|secret|n|k | combine|share1,share2,...");
                println!("Type 'exit' to quit.");

                let mut message = String::new();
                std::io::stdin().read_line(&mut message).unwrap();
                let message = message.trim();
                if message.eq_ignore_ascii_case("exit") {
                    break;
                }

                if let Err(e) = veil::network::start_client("127.0.0.1:7878", message).await {
                    eprintln!("Client error: {:?}", e);
                } else {
                    println!("Message processed successfully ✅");
                }
            }
            Ok(())
        })
    } else {
        println!("Invalid mode");
        Ok(())
    }
}

fn main() {
    if let Err(e) = start_node() {
        eprintln!("Error: {:?}", e);
    }
}
