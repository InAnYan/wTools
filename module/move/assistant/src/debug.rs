//!
//! Client of API.
//!

/// Internal namespace.
mod private
{

}

use format_tools::
{
  Fields,
  TableWithFields,
};
use std::borrow::Cow;

mod assistant_object;
mod file_data;
mod run_object;

crate::mod_interface!
{
  exposed use
  {
    assistant_object::AssistantObjectWrap,
    file_data::FileDataWrap,
    run_object::RunObjectWrap,
  };
}
