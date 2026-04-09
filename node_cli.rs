pub struct NodeCLI;

impl NodeCLI {
    pub fn start() {
        println!("=== Rust Blockchain Node CLI ===");
        println!("Commands:");
        println!("  info - show chain info");
        println!("  peers - list peers");
        println!("  tx <to> <amount> - send transaction");
        println!("  exit - quit node");
    }

    pub fn handle_input(input: &str) {
        match input.trim() {
            "info" => Self::show_info(),
            "peers" => Self::show_peers(),
            "exit" => std::process::exit(0),
            _ => println!("Unknown command"),
        }
    }

    fn show_info() {
        println!("Chain height: 100");
        println!("Node status: running");
        println!("Peers connected: 8");
    }

    fn show_peers() {
        println!("192.168.1.10:3030");
        println!("192.168.1.11:3030");
    }
}
