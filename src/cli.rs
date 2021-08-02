#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Serve,
    Build,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub command: Command,
    pub example: Option<String>,
    pub assets_folder: Option<String>,
}

fn exit_with_usage() -> ! {
    println!("Things currently supported:");
    println!("cargo quad {{serve|build}}");
    println!("cargo quad {{serve|build}} --example whatever");
    println!("cargo quad {{serve|build}} --assets FOLDER");

    std::process::exit(0)
}

pub fn parse_cli() -> Arguments {
    let mut args = std::env::args();

    // skip first CLI argument - the binary path
    let _ = args.next();

    let mut command = args.next();

    // if runned as a cargo subcommand - the second argument is "quad" string
    // if runned as a binary - no quad, just the command
    if matches!(command.as_deref(), Some("quad")) {
        command = args.next();
    }
    let command = match command.as_deref() {
        Some("serve") => Command::Serve,
        Some("build") => Command::Build,
        _ => exit_with_usage(),
    };

    let mut example = None;
    let mut assets_folder = None;

    loop {
        match (args.next().as_deref(), args.next().as_deref()) {
            (Some("--example"), Some(name)) => example = Some(name.to_owned()),
            (Some("--assets"), Some(name)) => assets_folder = Some(name.to_owned()),
            _ => break,
        };
    }

    Arguments {
        command,
        example,
        assets_folder,
    }
}
