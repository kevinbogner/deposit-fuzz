use anyhow::{Context, Result};
use std::env;

pub fn load_environment_variables() -> Result<(
    i32,    // ACC_START_INDEX
    i32,    // ACC_END_INDEX
    i64,    // DEPOSIT_AMOUNT
    String, // FORK_VERSION
    String, // DEPOSIT_CONTRACT_ADDRESS
    String, // WITHDRAWALS_MNEMONIC
    String, // VALIDATORS_MNEMONIC
    String, // DEPOSIT_DATAS_FILE_LOCATION
    String, // ETHEREAL_PATH
    String, // ETH2_VAL_TOOLS_PATH
    String, // ETH1_FROM_ADDR
    String, // ETH1_FROM_PRIV
    String, // ETH1_NETWORK
)> {
    dotenv::from_filename("secrets.env").ok();

    // Account indexes
    let acc_start_index: i32 = env::var("ACC_START_INDEX")
        .context("ACC_START_INDEX not found")?
        .parse()?;
    let acc_end_index: i32 = env::var("ACC_END_INDEX")
        .context("ACC_END_INDEX not found")?
        .parse()?;

    // Deposit-related values
    let deposit_amount: i64 = env::var("DEPOSIT_AMOUNT")
        .context("DEPOSIT_AMOUNT not found")?
        .parse()?;
    let fork_version: String = env::var("FORK_VERSION").context("FORK_VERSION not found")?;
    let deposit_contract_address: String =
        env::var("DEPOSIT_CONTRACT_ADDRESS").context("DEPOSIT_CONTRACT_ADDRESS not found")?;

    // Mnemonic words
    let withdrawals_mnemonic: String =
        env::var("WITHDRAWALS_MNEMONIC").context("WITHDRAWALS_MNEMONIC not found")?;
    let validators_mnemonic: String =
        env::var("VALIDATORS_MNEMONIC").context("VALIDATORS_MNEMONIC not found")?;

    // File and path specifications
    let deposit_datas_file_location: String =
        env::var("DEPOSIT_DATAS_FILE_LOCATION").context("DEPOSIT_DATAS_FILE_LOCATION not found")?;
    let ethereal_path: String = env::var("ETHEREAL_PATH").context("ETHEREAL_PATH not found")?;
    let eth2_val_tools_path: String =
        env::var("ETH2_VAL_TOOLS_PATH").context("ETH2_VAL_TOOLS_PATH not found")?;

    // Ethereum network details
    let eth1_from_addr: String = env::var("ETH1_FROM_ADDR").context("ETH1_FROM_ADDR not found")?;
    let eth1_from_priv: String = env::var("ETH1_FROM_PRIV").context("ETH1_FROM_PRIV not found")?;
    let eth1_network: String = env::var("ETH1_NETWORK").unwrap_or_default();

    Ok((
        acc_start_index,
        acc_end_index,
        deposit_amount,
        fork_version,
        deposit_contract_address,
        withdrawals_mnemonic,
        validators_mnemonic,
        deposit_datas_file_location,
        ethereal_path,
        eth2_val_tools_path,
        eth1_from_addr,
        eth1_from_priv,
        eth1_network,
    ))
}
