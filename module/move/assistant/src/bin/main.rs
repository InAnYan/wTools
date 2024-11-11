#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/assistant/latest/assistant/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use std::
{
  env,
  error::Error,
};

use dotenv::dotenv;
use clap::Parser;

use assistant::commands::{ Cli, self };

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn Error > >
{
  dotenv().ok();

  let cli = Cli::parse();

  commands::execute(cli).await?;

  Ok( () )
}
