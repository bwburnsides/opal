/*
opal new :proj-name:
opal build
opal run :args:
*/

#[derive(Debug)]
pub enum Subcommand {
    New(String),      // Create a new Opal project with the given name
    Build,            // Build the project without running it
    Run(Vec<String>), // Build and run the project with the provided args
}

pub fn parse_args(args: Vec<String>) -> Option<Subcommand> {
    let mut args = args;
    args.reverse();

    let prog_name = args
        .pop()
        .expect("Command line arguments should always have at least one member - the program name");

    let subcommand = match args.pop() {
        None => {
            eprintln!("Expected subcommand following `{prog_name}`");
            None
        }
        Some(arg) => match arg.as_str() {
            "new" => parse_subcommand_new(prog_name, &mut args),
            "build" => parse_subcommand_build(prog_name, &mut args),
            "run" => parse_subcommand_run(prog_name, &mut args),
            unrecognized => {
                eprintln!("Unrecognized subcommand `{unrecognized}`");
                None
            }
        },
    };

    if let Some(cmd) = subcommand {
        if args.len() == 0 {
            Some(cmd)
        } else {
            eprintln!("Unexpected arguments following subcommand");
            None
        }
    } else {
        None
    }
}

pub fn parse_subcommand_new(program_name: String, args: &mut Vec<String>) -> Option<Subcommand> {
    match args.pop() {
        None => {
            eprintln!("Expected project name following subcommand `new`");
            None
        }
        Some(arg) => Some(Subcommand::New(arg)),
    }
}

pub fn parse_subcommand_build(program_name: String, args: &mut Vec<String>) -> Option<Subcommand> {
    Some(Subcommand::Build)
}

pub fn parse_subcommand_run(program_name: String, args: &mut Vec<String>) -> Option<Subcommand> {
    Some(Subcommand::Run(args.to_vec()))
}
