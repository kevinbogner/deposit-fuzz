use std::process::Command;

use crate::env_vars::Environment;

pub fn build_deposits(env: &Environment) -> color_eyre::Result<()> {
    let output = Command::new(&env.eth2_val_tools_path)
        .arg("deposit-data")
        .arg("--source-min")
        .arg(env.acc_start_index.to_string())
        .arg("--source-max")
        .arg(env.acc_end_index.to_string())
        .arg("--amount")
        .arg(env.deposit_amount.to_string())
        .arg("--fork-version")
        .arg(&env.fork_version)
        .arg("--withdrawals-mnemonic")
        .arg(&env.withdrawals_mnemonic)
        .arg("--validators-mnemonic")
        .arg(&env.validators_mnemonic)
        .output()?;

    // Write output to a temporary file
    std::fs::write("deposit_data.txt", output.stdout)?;

    if !output.status.success() {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command executed with failing error code",
        ))
        .into())
    } else {
        Ok(())
    }
}
