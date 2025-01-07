use clap::Parser;
use cwl_runner::error::{CommandError, ExitCode};
use s4n::{
    cli::{Cli, Commands},
    commands::{
        execute::handle_execute_commands,
        init::handle_init_command,
        tool::{create_tool, handle_tool_commands},
        workflow::handle_workflow_commands,
    },
};
use std::{error::Error, process::exit};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        if let Some(cmd_err) = e.downcast_ref::<CommandError>() {
            exit(cmd_err.exit_code());
        } else {
            exit(1);
        }
    }
    exit(0);
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::Init(args) => handle_init_command(args)?,
        Commands::Tool { command } => handle_tool_commands(command)?,
        Commands::Run(args) => create_tool(args)?,
        Commands::Workflow { command } => handle_workflow_commands(command)?,
        Commands::Annotate => todo!(),
        Commands::Execute { command } => handle_execute_commands(command)?,
        Commands::Sync => todo!(),
    }
    Ok(())
}
