use std::str::FromStr;

use sdk::Sdk;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;

pub struct TestResources {
    /// RPC client
    pub client: RpcClient,
    /// Sdk
    pub sdk: Sdk,
}

impl TestResources {
    /// Create a new [TestResources] instance from environment variables
    pub fn new_from_env() -> Self {
        // Load environment variables
        let rpc_url =
            std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8899".to_string());
        let program_id = std::env::var("PROGRAM_ID").expect("PROGRAM_ID is not set");
        let payer_keypair_path =
            std::env::var("PAYER_KEYPAIR_PATH").expect("PAYER_KEYPAIR_PATH is not set");

        // Create resources
        let payer =
            read_keypair_file(payer_keypair_path).expect("Failed to read payer keypair file");
        let program_id = Pubkey::from_str(&program_id).expect("Failed to parse program ID");
        let client = RpcClient::new(rpc_url);
        let sdk = Sdk::new(program_id, payer);

        // Return resources
        Self { client, sdk }
    }

    /// Send and confirm transaction and print meta
    pub fn send_and_confirm_tx_with_logs(&self, tx: &solana_sdk::transaction::Transaction) {
        // Print signature
        println!(
            "https://explorer.solana.com/tx/{}?cluster=custom",
            tx.signatures[0]
        );

        // Send tx
        let signature = self.client.send_and_confirm_transaction(tx).unwrap();

        // Get transaction
        let tx = self
            .client
            .get_transaction(
                &signature,
                solana_transaction_status::UiTransactionEncoding::JsonParsed,
            )
            .unwrap();

        let meta = tx.transaction.meta.unwrap();

        println!("Err: {:?}", meta.err);
        println!("Log: {:?}", meta.log_messages);
    }
}
