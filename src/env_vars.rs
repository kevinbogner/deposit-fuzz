use color_eyre::{eyre::Context, Result};
use std::env;

pub struct Environment {
    pub acc_start_index: i32,
    pub acc_end_index: i32,
    pub deposit_amount: i64,
    pub fork_version: String,
    pub deposit_contract_address: String,
    pub withdrawals_mnemonic: String,
    pub validators_mnemonic: String,
    pub deposit_datas_file_location: String,
    pub ethereal_path: String,
    pub eth2_val_tools_path: String,
    pub eth1_from_addr: String,
    pub eth1_from_priv: String,
    pub eth1_network: String,
    pub deposit_delay_ms: u64,
}

pub fn load_environment_variables() -> Result<Environment> {
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
    let eth1_network: String = env::var("ETH1_NETWORK").context("ETH1_NETWORK not found")?;
    let deposit_delay_ms: u64 = env::var("DEPOSIT_DELAY_MS")
        .context("DEPOSIT_DELAY_MS not found")?
        .parse()?;

    Ok(Environment {
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
        deposit_delay_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_environment_variables() {
        env::set_var("ACC_START_INDEX", "0");
        env::set_var("ACC_END_INDEX", "100");
        env::set_var("DEPOSIT_AMOUNT", "32000000000");
        env::set_var("FORK_VERSION", "0x04000000");
        env::set_var("DEPOSIT_CONTRACT_ADDRESS", "0x...");
        env::set_var("WITHDRAWALS_MNEMONIC", "word1 word2 word3 ... wordN");
        env::set_var("VALIDATORS_MNEMONIC", "word1 word2 word3 ... wordN");
        env::set_var("DEPOSIT_DATAS_FILE_LOCATION", "deposit_data.txt");
        env::set_var("ETHEREAL_PATH", "/home/user/go/bin/ethereal");
        env::set_var("ETH2_VAL_TOOLS_PATH", "/home/user/go/bin/eth2-val-tools");
        env::set_var("ETH1_FROM_ADDR", "0x...");
        env::set_var("ETH1_FROM_PRIV", "0x...");
        env::set_var("ETH1_NETWORK", "http://127.0.0.1:8545");
        env::set_var("DEPOSIT_DELAY_MS", "1000");

        let result = load_environment_variables().unwrap();

        assert_eq!(result.acc_start_index, 0);
        assert_eq!(result.acc_end_index, 100);
        assert_eq!(result.deposit_amount, 32000000000);
        assert_eq!(result.fork_version, "0x04000000");
        assert_eq!(result.deposit_contract_address, "0x...");
        assert_eq!(result.withdrawals_mnemonic, "word1 word2 word3 ... wordN");
        assert_eq!(result.validators_mnemonic, "word1 word2 word3 ... wordN");
        assert_eq!(result.deposit_datas_file_location, "deposit_data.txt");
        assert_eq!(result.ethereal_path, "/home/user/go/bin/ethereal");
        assert_eq!(
            result.eth2_val_tools_path,
            "/home/user/go/bin/eth2-val-tools"
        );
        assert_eq!(result.eth1_from_addr, "0x...");
        assert_eq!(result.eth1_from_priv, "0x...");
        assert_eq!(result.eth1_network, "http://127.0.0.1:8545");
        assert_eq!(result.deposit_delay_ms, 1000);
    }
}
