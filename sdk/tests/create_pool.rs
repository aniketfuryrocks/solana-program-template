use self::common::TestResources;

mod common;

#[test]
pub fn create_pool() {
    let resources = TestResources::new_from_env();

    // Get latest blockhash
    let blockhash = resources.client.get_latest_blockhash().unwrap();

    // Create tx
    let ix = resources.sdk.create_pool_ix();
    let tx = resources.sdk.compose_solana_tx(&[ix], blockhash);

    resources.send_and_confirm_tx_with_logs(&tx);
}
