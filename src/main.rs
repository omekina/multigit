pub const CONFIG_PATH: &'static str = "/home/user/.mgitconfig"; // Replace with your config filepath


mod systemhandler;
mod configloader;
use std::{process::exit, env::args};

use systemhandler::{ GitCommand, CommandResult::{ CommandOk, CommandError, WrapperError } };


fn parse_args(args: Vec<String>) -> Result<GitCommand, &'static str> {
    if args.len() < 2 { return Err("No space designator requested."); }
    if args.len() == 2 { return Err("No git command specified."); }
    let designator = args.get(1).unwrap().to_string();
    let mut stripped_args: Vec<String> = vec![];
    for i in 2..args.len() {
        stripped_args.push(args.get(i).unwrap().to_string());
    }
    return Ok(GitCommand {
        args: stripped_args,
        ssh_command: designator,
    });
}


fn main() {
    let mut git_command = match parse_args(args().collect()) {
        Ok(result) => result,
        Err(detail) => {
            println!("{}", detail);
            exit(1);
        },
    };
    git_command.ssh_command = match configloader::run(git_command.ssh_command) {
        Ok(result) => String::from("ssh -i ") + result.as_str(),
        Err(detail) => {
            println!("{}", detail);
            exit(1);
        },
    };
    match systemhandler::run_git_command(git_command) {
        CommandOk() => exit(0),
        CommandError() => exit(1),
        WrapperError(detail) => {
            println!("{}", detail);
            exit(1);
        },
    };
}
