//!
//! List runs in OpenAI API (action part).
//!

mod private
{

  use std::fmt;

  use format_tools::
  {
    AsTable,
    TableFormatter,
    output_format,
  };

  use crate::*;
  use client::Client;
  use debug::RunObjectWrap;
  use actions::openai::Result;
  use commands::{ TableConfig, TableStyle };
  use util::display_table::display_tabular_data;

  /// Report for `openai runs list`.
  #[ derive( Debug ) ]
  pub struct ListReport
  {
    /// Configure table formatting.
    pub table_config : TableConfig,

    /// Current OpenAI runs.
    pub runs : Vec< RunObjectWrap >,
  }

  impl fmt::Display for ListReport
  {
    fn fmt
    ( 
      &self, 
      f : &mut fmt::Formatter< '_ >
    ) -> fmt::Result
    {
      display_tabular_data( &self.runs, f, &self.table_config )
    }
  }

  /// List OpenAI runs action.
  pub async fn action
  (
    client : &Client,
    thread_id : String,
    table_config : TableConfig,
  ) -> Result < ListReport >
  {
    let response = client.list_run( thread_id, None, None, None, None ).await?;
    let runs = response.data.into_iter().map( RunObjectWrap ).collect();
    Ok( ListReport { table_config, runs } )
  }

}

crate::mod_interface!
{
  own use action;
}