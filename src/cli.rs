#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Serve,
    Build,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub command: Command,
    pub release: bool,
    pub example: Option<String>,
    pub assets_folder: Option<String>,
}

fn exit_with_usage() -> ! {
    println!("Things currently supported:");
    println!("cargo webquad {{serve|build}}");
    println!("cargo webquad {{serve|build}} --example whatever");
    println!("cargo webquad {{serve|build}} --assets FOLDER");

    std::process::exit(0)
}

pub fn parse_cli() -> Arguments {
    let mut args = std::env::args();

    // skip first CLI argument - the binary path
    let _ = args.next();

    let mut command = args.next();

    // if runned as a cargo subcommand - the second argument is "quad" string
    // if runned as a binary - no quad, just the command
    if matches!(command.as_deref(), Some("webquad")) {
        command = args.next();
    }
    let command = match command.as_deref() {
        Some("serve") => Command::Serve,
        Some("build") => Command::Build,
        _ => exit_with_usage(),
    };

    let mut example = None;
    let mut assets_folder = None;
    let mut release = false;

    loop {
        match args.next().as_deref() {
            Some("--example") => example = Some(args.next().unwrap().to_owned()),
            Some("--assets") => assets_folder = Some(args.next().unwrap().to_owned()),
            Some("--release") => release = true,
            _ => break,
        };
    }

    Arguments {
        command,
        example,
        release,
        assets_folder,
    }
}
