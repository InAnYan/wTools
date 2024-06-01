use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code) ]
#[ derive( Deref ) ]
enum EnumUnit
{
  A,
  B,
}

include!( "./only_tests/enum_unit.rs" );
