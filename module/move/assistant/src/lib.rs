#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/assistant/latest/assistant/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use mod_interface::mod_interface;

/// Internal namespace.
mod private
{
}

// pub mod client;
// pub mod clie;

crate::mod_interface!
{

  layer client;
  layer cli;
  layer debug;
  layer commands;
  layer actions;

  exposed use ::reflect_tools::
  {
    Fields,
    _IteratorTrait,
    IteratorTrait,
  };

}
