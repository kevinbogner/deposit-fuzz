use std::process::Command;

pub fn build_deposits(
    acc_start_index: i32,
    acc_end_index: i32,
    deposit_amount: i64,
    fork_version: String,
    withdrawals_mnemonic: String,
    validators_mnemonic: String,
    deposit_datas_file_location: String,
    eth2_val_tools_path: String,
) -> anyhow::Result<()> {
    let output = Command::new(eth2_val_tools_path)
        .arg("deposit-data")
        .arg("--source-min")
        .arg(acc_start_index.to_string())
        .arg("--source-max")
        .arg(acc_end_index.to_string())
        .arg("--amount")
        .arg(deposit_amount.to_string())
        .arg("--fork-version")
        .arg(fork_version)
        .arg("--withdrawals-mnemonic")
        .arg(withdrawals_mnemonic)
        .arg("--validators-mnemonic")
        .arg(validators_mnemonic)
        .output()?;

    // Write output to file
    std::fs::write(deposit_datas_file_location, output.stdout)?;

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
