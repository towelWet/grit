    pub fn parse() -> Self {
        match Self::try_parse(std::env::args().skip(1)) {
            Ok(args) => args,
            Err(message) => {
                eprintln!("error: {message}\n\n{}", Self::usage());
                std::process::exit(64);
            }
        }
    }
