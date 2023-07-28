use std::env;

pub struct CmdArgs {
    pub randomize: bool,
}

impl CmdArgs {
    pub fn new() -> Self {
        let mut randomize = false;

        for arg in env::args().skip(1) {
            match arg.as_str() {
                "-r" | "--randomize" => randomize = true,
                _ => {
                    eprintln!("Error: Unrecognized command line argument '{}'", arg);
                    eprintln!("Usage: deposit-fuzz [-r | --randomize]");
                    std::process::exit(1);
                }
            }
        }

        CmdArgs { randomize }
    }
}
