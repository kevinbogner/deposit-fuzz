use color_eyre::eyre::Context;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::env_vars::Environment;

pub fn execute_deposits(env: &Environment, deposit_data_file: &str) -> color_eyre::Result<()> {
    let lines = read_lines(deposit_data_file).context(format!(
        "Failed to read lines from file {}",
        deposit_data_file
    ))?;

    // Iterate over lines in the file
    for line in lines {
        match line {
            Ok(ip) => {
                // Parse JSON and retrieve account name and pubkey
                let v: Value = serde_json::from_str(&ip).context("Failed to parse json")?;
                let account_name = v["account"]
                    .as_str()
                    .ok_or(color_eyre::eyre::eyre!("Failed to get account from json"))?;
                let pubkey = v["pubkey"]
                    .as_str()
                    .ok_or(color_eyre::eyre::eyre!("Failed to get pubkey from json"))?;

                println!("Sending deposit for validator {} {}", account_name, pubkey);

                // Execute command
                let output = Command::new(&env.ethereal_path)
                    .arg("beacon")
                    .arg("deposit")
                    .arg("--allow-unknown-contract")
                    .arg("--allow-new-data")
                    .arg("--allow-old-data")
                    .arg("--allow-excessive-deposit")
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
                    return Err(color_eyre::eyre::eyre!(
                        "Command executed with failing error code"
                    ));
                }

                println!("Sent deposit for validator {} {}", account_name, pubkey);

                thread::sleep(Duration::from_millis(env.deposit_delay_ms));
            }
            Err(err) => return Err(color_eyre::eyre::eyre!("Failed to read line: {}", err)),
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
