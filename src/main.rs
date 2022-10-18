use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
};
use solana_program::program_pack::Pack;
use solana_sdk::commitment_config::CommitmentConfig;
use spl_token::state::{
    Account as TokenAccount,
    Mint,
};

const USDC_ADDR: &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let keys = vec![
        Pubkey::from_str(&"HWc8uSfiVxAHQSTHWTqyD51ikamgxJRVGi9uE5Sh5SxY").unwrap(),
        Pubkey::from_str(&"SecGauVARr6aDhJEk38D52P5H54i3grKG66vZH3f3GV").unwrap()
    ];

    let accounts = client.get_multiple_accounts(&keys).unwrap();
    for (key, account) in std::iter::zip(keys, accounts).into_iter() {
        println!("{}: {}.{:09} SOL",
            key,
            account.as_ref().unwrap().lamports/LAMPORTS_PER_SOL,
            account.as_ref().unwrap().lamports%LAMPORTS_PER_SOL,
        );
    }

    let usdc_pubkey = Pubkey::from_str(USDC_ADDR).unwrap();
    let usdc_mint_data = client.get_account_data(&usdc_pubkey).unwrap();
    let usdc_mint = Mint::unpack(usdc_mint_data.as_slice()).unwrap();

    let commitment = CommitmentConfig::finalized();
    let me_usdc_pubkey = Pubkey::from_str("DmNbvoX6X2T5f4hfc11JyLJUQpnQ6V6XBFQXerJtJ38D").unwrap();
    let usdc_token_account = client.get_token_account(&me_usdc_pubkey).unwrap().unwrap();
    println!("{:?}", usdc_mint);
    println!("{:?}", usdc_token_account);
}
