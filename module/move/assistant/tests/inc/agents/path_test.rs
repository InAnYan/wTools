use super::*;

use the_module::agents::Path;

#[ test ]
fn path_create_right()
{
  let path_str = "agent::completion";

  let path = Path::try_from( path_str );

  assert!( path.is_ok() );
  assert_eq! ( path.inner(), path_str );
}

#[ test ]
fn path_create_wrong()
{
  let path_str = "agent:completion";

  let path = Path::try_from( path_str );

  assert!( path.is_ok() );
  assert_eq! ( path.inner(), path_str );
}

#[ test ]
fn path_create_absolute()
{
  let path_str = "::agent::completion";

  let path = Path::try_from( path_str );

  assert!( path.is_ok() );
  assert_eq! ( path.inner(), path_str );
}

#[ test ]
fn path_get_some_parent_relative()
{
  let path_str = "agent::completion";
  let path = Path::try_from( path_str ).unwrap();

  let path_parent = path.parent();

  assert!( path_parent.is_some() );
  assert_eq!( path_parent.unwrap(), "agent" );
}

#[ test ]
fn path_get_some_parent_absolute()
{
  let path_str = "::agent";
  let path = Path::try_from( path_str ).unwrap();

  let path_parent = path.parent();

  assert!( path_parent.is_some() );
  assert_eq!( path_parent.unwrap(), "::" );
}

#[ test ]
fn path_get_none_parent()
{
  let path_str = "agent";
  let path = Path::try_from( path_str ).unwrap();

  let path_parent = path.parent();

  assert!( path_parent.is_none() );
}

#[ test ]
fn path_is_relative()
{
  let path_str = "agent";
  let path = Path::try_from( path_str ).unwrap();

  let is_relative = path.is_relative();
  let is_absolute = path.is_absolute();

  assert!( is_relative );
  assert!( !is_absolute );
}

#[ test ]
fn path_is_absolute()
{
  let path_str = "::agent";
  let path = Path::try_from( path_str ).unwrap();

  let is_relative = path.is_relative();
  let is_absolute = path.is_absolute();

  assert!( !is_relative );
  assert!( is_absolute );
}

#[ test ]
fn path_join_relative()
{
  let orig_path = Path::try_from( "agent" ).unwrap();
  let append = Path::try_from( "completion" ).unwrap();

  let combined = orig_path.join( append );

  assert!( combined.is_ok() );
  assert_eq!( combined.unwrap().inner(), "agent::completion" );
}

#[ test ]
fn path_join_absolute()
{
  let orig_path = Path::try_from( "agent" ).unwrap();
  let append = Path::try_from( "::completion" ).unwrap();

  let combined = orig_path.join( append );

  assert!( combined.is_err() );
}

#[ test ]
fn path_join_root()
{
  let orig_path = Path::try_from( "::" ).unwrap();
  let append = Path::try_from( "agent" ).unwrap();

  let combined = orig_path.join( append );

  assert!( combined.is_ok() );
  assert_eq!( combined.unwrap().inner(), "::agent" );
}

#[ test ]
fn path_starts_with_abs_abs()
{
  let a = Path::try_from( "::agent::completion" ).unwrap();
  let b = Path::try_from( "::agent" ).unwrap();

  let starts_with = a.starts_with( b );

  assert!( starts_with );
}

#[ test ]
fn path_starts_with_abs_rel()
{
  let a = Path::try_from( "::agent::completion" ).unwrap();
  let b = Path::try_from( "agent" ).unwrap();

  let starts_with = a.starts_with( b );

  assert!( !starts_with );
}

#[ test ]
fn path_starts_with_rel_abs()
{
  let a = Path::try_from( "agent" ).unwrap();
  let b = Path::try_from( "::agent::completion" ).unwrap();

  let starts_with = a.starts_with( b );

  assert!( !starts_with );
}

#[ test ]
fn path_starts_with_rel_rel()
{
  let a = Path::try_from( "agent::completion" ).unwrap();
  let b = Path::try_from( "agent" ).unwrap();

  let starts_with = a.starts_with( b );

  assert!( starts_with );
}

#[ test ]
fn path_not_starts_with_abs_abs()
{
  let a = Path::try_from( "::agent::completion" ).unwrap();
  let b = Path::try_from( "::output" ).unwrap();

  let starts_with = a.starts_with( b );

  assert!( !starts_with );
}

#[ test ]
fn path_not_starts_with_rel_rel()
{
  let a = Path::try_from( "agent::completion" ).unwrap();
  let b = Path::try_from( "output" ).unwrap();

  let starts_with = a.starts_with( b );

  assert!( !starts_with );
}

#[ test ]
fn path_inner()
{
  let path_str = "::agent::completion";
  let path = Path::try_from( path_str ).unwrap();

  let inner = path.inner();

  assert_eq!( inner, path_str );
}

#[ test ]
fn path_from_iter_right()
{
  let expected = "agents::completion";
  let elements = vec![ "agents", "completion" ];

  let path = Path::from_iter( elements.iter() );

  assert!( path.is_ok() )
  assert_eq!( path.unwrap().inner(), expected );
}

#[ test ]
fn path_from_iter_right()
{
  let elements = vec![ "agents:", "completion" ];

  let path = Path::from_iter( elements.iter() );

  assert!( path.is_err() )
}

#[ test ]
fn path_from_iter_with_root()
{
  let expected = "::agents::completion";
  let elements = vec![ "::", "agents", "completion" ];

  let path = Path::from_iter( elements.iter() );

  assert!( path.is_ok() )
  assert_eq!( path.unwrap().inner(), expected );
}