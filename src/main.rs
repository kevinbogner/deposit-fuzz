mod build_deposits;
mod env_vars;
mod exec_deposits;

fn main() -> anyhow::Result<()> {
    match env_vars::load_environment_variables() {
        Ok(env) => {
            build_deposits::build_deposits(&env)?;
            exec_deposits::execute_deposits(&env)?;
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
