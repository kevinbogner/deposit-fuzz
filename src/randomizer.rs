use rand::Rng;
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::env_vars::Environment;

pub fn randomize_deposit_data(env: &Environment) -> anyhow::Result<()> {
    let input = File::open(&env.deposit_datas_file_location)?;
    let reader = BufReader::new(input);

    let mut output = File::create("randomized_deposit_data.txt")?;

    let mut rng = rand::thread_rng();

    // Define the range of deposit values (in Gwei)
    let min_deposit: u64 = 32_000_000_000;
    let max_deposit: u64 = 50_000_000_000;

    for line in reader.lines() {
        let line = line?;
        let mut data: Value = serde_json::from_str(&line)?;

        // Randomize deposit value
        if let Some(deposit) = data.get_mut("value") {
            let random_deposit = rng.gen_range(min_deposit..=max_deposit);
            *deposit = Value::Number(serde_json::Number::from(random_deposit));
        }

        let line = serde_json::to_string(&data)?;

        writeln!(output, "{}", line)?;
    }

    Ok(())
}
