mod build_deposits;
mod env_vars;
mod exec_deposits;

fn main() -> anyhow::Result<()> {
    match env_vars::load_environment_variables() {
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
        )) => {
            build_deposits::build_deposits(
                acc_start_index,
                acc_end_index,
                deposit_amount,
                fork_version,
                withdrawals_mnemonic,
                validators_mnemonic,
                deposit_datas_file_location.clone(),
                eth2_val_tools_path,
            )?;

            exec_deposits::execute_deposits(
                deposit_contract_address,
                eth1_from_addr,
                eth1_from_priv,
                eth1_network,
                deposit_amount.to_string(),
                deposit_datas_file_location,
                ethereal_path,
            )?;
        }
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Error loading environment variables: {:?}",
                e
            ));
        }
    }
    Ok(())
}
