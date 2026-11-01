    fn try_parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut words = args.peekable();
        let mut positionals = Vec::new();
        let mut parsed = Self::defaults();
        while let Some(word) = words.next() {
            match word.as_str() {
                "--help" | "-h" => {
                    println!("{}", Self::usage());
                    std::process::exit(0);
                }
                "--repo" | "-C" => parsed.repo = Some(PathBuf::from(next(&mut words, &word)?)),
                "--format" => parsed.format = next(&mut words, &word)?,
                "--message" | "-m" => parsed.message = Some(next(&mut words, &word)?),
                "--author" => parsed.author = next(&mut words, &word)?,
                "--path" | "--paths" | "-p" => parsed.paths.push(next(&mut words, &word)?),
                "--force" | "-f" => parsed.force = true,
                "--id" => parsed.id = Some(next(&mut words, &word)?),
                "--limit" | "-n" => {
                    parsed.limit = next(&mut words, &word)?
                        .parse()
                        .map_err(|_| "--limit requires an integer".to_string())?;
                }
                "--metadata" => parsed.metadata.push(next(&mut words, &word)?),
                "--no-safety-save" => parsed.no_safety_save = true,
                "--full" => parsed.full = true, // verify: reread every object
                option if option.starts_with('-') => {
                    return Err(format!("unknown option: {option}"))
                }
                _ => positionals.push(word),
            }
        }
        parsed.command = parsed.command_from(&positionals)?;
        Ok(parsed)
    }
