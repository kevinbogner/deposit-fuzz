mod build_deposits;
mod env_vars;
mod exec_deposits;
mod randomizer;

fn main() -> anyhow::Result<()> {
    match env_vars::load_environment_variables() {
        Ok(env) => {
            build_deposits::build_deposits(&env)?;
            randomizer::randomize_deposit_data(&env)?;
            exec_deposits::execute_deposits(&env, "randomized_deposit_data.txt")?;
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
