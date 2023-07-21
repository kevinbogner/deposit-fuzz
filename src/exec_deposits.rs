use anyhow::Context;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::env_vars::Environment;

pub fn execute_deposits(env: &Environment) -> anyhow::Result<()> {
    // Open the file
    let lines = read_lines(&env.deposit_datas_file_location).context(format!(
        "Failed to read lines from file {}",
        env.deposit_datas_file_location
    ))?;

    // Iterate over lines in the file
    for line in lines {
        match line {
            Ok(ip) => {
                // Parse JSON and retrieve account name and pubkey
                let v: Value = serde_json::from_str(&ip).context("Failed to parse json")?;
                let account_name = v["account"]
                    .as_str()
                    .ok_or(anyhow::anyhow!("Failed to get account name from json"))?;
                let pubkey = v["pubkey"]
                    .as_str()
                    .ok_or(anyhow::anyhow!("Failed to get pubkey from json"))?;

                println!("Sending deposit for validator {} {}", account_name, pubkey);

                // Execute command
                let output = Command::new(&env.ethereal_path)
                    .arg("beacon")
                    .arg("deposit")
                    .arg("--allow-unknown-contract")
                    .arg("--address")
                    .arg(&env.deposit_contract_address)
                    .arg("--connection")
                    .arg(&env.eth1_network)
                    .arg("--data")
                    .arg(&ip)
                    .arg("--value")
                    .arg(&env.deposit_amount.to_string())
                    .arg("--from")
                    .arg(&env.eth1_from_addr)
                    .arg("--privatekey")
                    .arg(&env.eth1_from_priv)
                    .output()
                    .context("Failed to execute command")?;

                if !output.status.success() {
                    return Err(anyhow::anyhow!("Command executed with failing error code"));
                }

                println!("Sent deposit for validator {} {}", account_name, pubkey);

                thread::sleep(Duration::from_millis(env.deposit_delay_ms));
            }
            Err(err) => return Err(anyhow::anyhow!(err)),
        }
    }

    Ok(())
}

// Read lines of a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
