use std::env;

pub struct CmdArgs {
    pub randomize: bool,
    pub rpc: Option<String>,
}

impl CmdArgs {
    pub fn new() -> Self {
        let mut randomize = false;
        let mut rpc: Option<String> = None;
        let mut args_iter = env::args().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-r" | "--randomize" => randomize = true,
                "--rpc" => {
                    rpc = Some(args_iter.next().unwrap_or_else(|| {
                        eprintln!("Error: Missing RPC URL for --rpc argument");
                        std::process::exit(1);
                    }))
                }
                _ => {
                    eprintln!("Error: Unrecognized command line argument '{}'", arg);
                    eprintln!("Usage: deposit-fuzz [-r | --randomize] [--rpc RPC_URL]");
                    std::process::exit(1);
                }
            }
        }

        CmdArgs { randomize, rpc }
    }
}
