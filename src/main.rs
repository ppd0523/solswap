use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_program::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com");

    let keys = vec![
        Pubkey::from_str(&"HWc8uSfiVxAHQSTHWTqyD51ikamgxJRVGi9uE5Sh5SxY").unwrap(),
        Pubkey::from_str(&"SecGauVARr6aDhJEk38D52P5H54i3grKG66vZH3f3GV").unwrap()
    ];

    let accounts = client.get_multiple_accounts(&keys).unwrap();
    for (key, account) in std::iter::zip(keys, accounts).into_iter() {
        println!("{}: {}.{:09}",
            key,
            account.as_ref().unwrap().lamports/LAMPORTS_PER_SOL,
            account.as_ref().unwrap().lamports%LAMPORTS_PER_SOL,
        );
    }

    // client.get_token_account()
    // client.get_account_with_commitment("")

}
