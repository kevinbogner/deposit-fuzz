use ethereum_types::H256 as EthH256;
use ethereum_types::H512 as EthH512;
use hex::encode as hex_encode;
use rand::Rng;
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::env_vars::Environment;

pub fn randomize_deposit_data(env: &Environment) -> anyhow::Result<()> {
    let input = File::open(&env.deposit_datas_file_location)?;
    let reader = BufReader::new(input);

    let mut output = File::create("randomized_deposit_data.txt")?;

    let mut rng = rand::thread_rng();

    // Define the range of deposit values (in Gwei)
    let min_deposit: u64 = 32_000_000_000;
    let max_deposit: u64 = 50_000_000_000;

    for line in reader.lines() {
        let line = line?;
        let mut data: Value = serde_json::from_str(&line)?;

        if let Some(deposit) = data.get_mut("value") {
            let random_deposit = rng.gen_range(min_deposit..=max_deposit);
            *deposit = Value::Number(serde_json::Number::from(random_deposit));
        }

        if let Some(account) = data.get_mut("account") {
            let random_account = format!("m/12381/3600/{}/0/0", rng.gen_range(0..=2147483647));
            *account = Value::String(random_account);
        }

        if let Some(pubkey) = data.get_mut("pubkey") {
            let random_pubkey: EthH512 = EthH512::random();
            *pubkey = Value::String(format!("0x{}", hex_encode(random_pubkey.to_fixed_bytes())));
        }

        if let Some(signature) = data.get_mut("signature") {
            let random_signature: EthH512 = EthH512::random();
            *signature = Value::String(format!(
                "0x{}",
                hex_encode(random_signature.to_fixed_bytes())
            ));
        }

        if let Some(version) = data.get_mut("version") {
            let random_version = rng.gen_range(1..=10);
            *version = Value::Number(serde_json::Number::from(random_version));
        }

        if let Some(withdrawal_credentials) = data.get_mut("withdrawal_credentials") {
            let random_withdrawal_credentials: EthH256 = EthH256::random();
            *withdrawal_credentials =
                Value::String(hex_encode(random_withdrawal_credentials.to_fixed_bytes()));
        }

        let line = serde_json::to_string(&data)?;

        writeln!(output, "{}", line)?;
    }

    Ok(())
}
