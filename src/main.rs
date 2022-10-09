// std imports
use std::{
    env,
    cmp::max
};

// Crates
use throttled_bitcoin_rpc::BitcoinRpcClient;

// Block to Checkpoint String formatter
fn block_to_checkpoint(height: u32, hash: String) -> String {
    format!("{{{}, uint256S(\"{}\")}},", height, hash)
}

fn main() {
    // Settings and their defaults
    let mut rpchost = String::from("http://localhost:8332");
    let mut rpcuser = String::from("user");
    let mut rpcpass = String::from("pass");
    let mut start_block: u32 = 0;
    let mut interval: u32 = 1;

    // Init: Parse arguments and adjust settings accordingly
    for arg in env::args() {
        if arg == "-h" || arg == "--help" {
            // Log the full manual
            println!("A blockchain checkpoint automation tool for Bitcoin-based codebases\n\n\
                     Options:\n\
                       --help (-h)       | displays this screen, hi!\n\
                       --host [string]   | the RPC host, default: {}\n\
                       --user [string]   | the RPC user, default: {}\n\
                       --pass [string]   | the RPC pass, default: {}\n\
                       --start [int]     | the start height for the checkpointer, default: {}\n\
                       --interval [int]  | the interval, in blocks, that checkpoints are taken in, default: {}",
                     rpchost, rpcuser, rpcpass, start_block, interval
            );

            // Then exit the application early
            std::process::exit(0);
        }

        if arg.starts_with("--host=") {
            // User-set RPC host
            rpchost = arg[7..].to_string();
        }

        if arg.starts_with("--user=") {
            // User-set RPC user
            rpcuser = arg[7..].to_string();
        }

        if arg.starts_with("--pass=") {
            // User-set RPC pass
            rpcpass = arg[7..].to_string();
        }

        if arg.starts_with("--start=") {
            // User-set block height to start the checkpointer at
            start_block = max(arg[8..].parse().unwrap_or(0), start_block);
        }

        if arg.starts_with("--interval=") {
            // User-set interval in blocks to take checkpoints at
            interval = max(arg[11..].parse().unwrap_or(0), interval);
        }
    }

    // Instantiate the RPC client
    let client = BitcoinRpcClient::new(
        rpchost,
        Some(rpcuser),
        Some(rpcpass),
        3,
        10,
        1000
    );

    // Loop blocks and collect checkpoints until we run out
    let mut current_height = start_block;
    loop {
        // Fetch the matching block hash
        let hash = match client.getblockhash(current_height) {
            Ok(blockhash) => blockhash,
            Err(_) => break println!("Checkpoints done!")
        };

        // Convert data to a checkpoint and log
        println!("{}", block_to_checkpoint(current_height, hash));

        // Increment height for the next loop
        current_height += interval;
    }
}
