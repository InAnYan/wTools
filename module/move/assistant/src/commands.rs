//!
//! CLI commands of the tool.
//!

/// Internal namespace.
mod private
{

  use clap::{ Parser, Subcommand };

  use crate::*;
  use client::Client;

  /// CLI commands of the tool.
  #[ derive ( Debug,Parser ) ]
  pub struct Cli
  {
    /// Root of the CLI commands.
    #[ command ( subcommand ) ]
    pub command : CliCommand,
  }

  /// Root of the CLI commands.
  #[ derive ( Debug,Subcommand ) ]
  pub enum CliCommand
  {
    /// OpenAI API commands.
    #[ command ( subcommand, name = "openai" ) ]
    OpenAi(OpenAiCommand),
  }

}

crate::mod_interface!
{
  layer openai;

  orphan use
  {
    Cli,
    CliCommand,
  };
}
