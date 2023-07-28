mod build_deposits;
mod cmd_args;
mod env_vars;
mod exec_deposits;
mod randomizer;

fn main() -> color_eyre::Result<()> {
    let args = cmd_args::CmdArgs::new();

    match env_vars::load_environment_variables() {
        Ok(mut env) => {
            if let Some(rpc_url) = args.rpc {
                if !rpc_url.is_empty() {
                    env.eth1_network = rpc_url;
                }
            }

            build_deposits::build_deposits(&env)?;
            if args.randomize {
                randomizer::randomize_deposit_data(&env.deposit_datas_file_location)?;
            }
            exec_deposits::execute_deposits(&env, "deposit_data.txt")?;
        }
        Err(e) => {
            return Err(color_eyre::Report::msg(format!(
                "Failed to load environment variables: {}",
                e
            )))
        }
    }
    Ok(())
}
