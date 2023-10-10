use std::process::Command;

pub struct GitCommand {
    pub args: Vec<String>,
    pub ssh_command: String,
}


pub enum CommandResult {
    CommandOk(),
    CommandError(),
    WrapperError(&'static str),
}


use CommandResult::{ CommandOk, CommandError, WrapperError };


pub fn run_git_command(command_info: GitCommand) -> CommandResult {
    let command_handle = match Command::new("git")
        .env("GIT_SSH_COMMAND", command_info.ssh_command)
        .args(command_info.args)
        .output() {
            Ok(command_output) => command_output,
            Err(_) => return WrapperError("Error when calling the command."),
    };
    if !command_handle.status.success() {
        println!("{}", match String::from_utf8(command_handle.stderr) {
            Ok(result) => result,
            Err(_) => return WrapperError("Error when decoding command stderr stream."),
        });
        return CommandError();
    }
    println!("{}", match String::from_utf8(command_handle.stdout) {
        Ok(result) => result,
        Err(_) => return WrapperError("Error when decoding command stdout stream."),
    });
    return CommandOk();
}
