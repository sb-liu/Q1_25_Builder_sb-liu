mod programs;

#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };
    use std::str::FromStr;
    use crate::programs::Turbin3_prereq::{WbaPrereqProgram, CompleteArgs};
    use solana_program::system_program;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!();
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair: Keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Connected to Solana Devnet RPC Client
        let client: RpcClient = RpcClient::new(RPC_URL);
        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    // 1st transfer
    // https://explorer.solana.com/tx/5UrjbgdyMreXeeKzc4cGJoBce1tMaHYV4VATmHiYkKzbnWngdhhVRmm2rGrvfaD9dpNoBiAxgXYa3sg6Ludt8y9L?cluster=devnet
    // 2nd transfer
    // https://explorer.solana.com/tx/3zoYfLEPdanDNwFRKjsHGJ9pdbC3yLAJMpxepJCrZtZeeMNpuUKFovPgJa9Bhb52vSDU523nMLhvcaNta4nAXp8t?cluster=devnet
    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("8NJ59grUmY3xHC9e3wuXUoW6qBbVRcVSXe88u66dLzRF").unwrap();
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee: u64 = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here:");
        println!(
            "https://explorer.solana.com/tx/{}?cluster=devnet",
            signature.to_string()
        );
    }

    #[test]
    fn sign_message() {
        let rpc_client = RpcClient::new(RPC_URL);
        // Let's define our accounts 
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        // Define our instruction data 
        let args = CompleteArgs { github: b"sb-liu".to_vec() };
        // Get recent blockhash 
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let transaction = WbaPrereqProgram::complete( 
            &[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()), &[&signer], 
            blockhash ); 
        let signature = rpc_client.send_and_confirm_transaction(&transaction) .expect("Failed to send transaction");
        // Print our transaction out
        println!("Signature: {}", signature.to_string());
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature.to_string());

    }
}
