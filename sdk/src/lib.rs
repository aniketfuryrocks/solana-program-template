use lib::ix::Ix;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

/// Client interface
pub struct Sdk {
    /// Program ID
    program_id: Pubkey,
    /// Payer
    payer: Keypair,
}

impl Sdk {
    /// Create a new [Sdk] instance
    pub fn new(program_id: Pubkey, payer: Keypair) -> Self {
        Self { program_id, payer }
    }

    /// Create [Instruction] to create a pool
    pub fn create_pool_ix(&self) -> Instruction {
        self.create_instruction(Ix::CreatePool, vec![])
    }

    /// Create [Instruction] from [Ix] and list of [AccountMeta]
    pub fn create_instruction(&self, ix: Ix, accounts: Vec<AccountMeta>) -> Instruction {
        let program_id = self.program_id;

        Instruction::new_with_bincode(program_id, &ix, accounts)
    }

    /// Compose list of [Instruction]s to [Transaction]
    pub fn compose_solana_tx(&self, ixs: &[Instruction], blockhash: Hash) -> Transaction {
        let msg = Message::new(ixs, Some(&self.payer.pubkey()));

        Transaction::new(&[&self.payer], msg, blockhash)
    }
}
