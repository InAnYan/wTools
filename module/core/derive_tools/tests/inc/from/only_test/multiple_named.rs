#[ test ]
fn from_named()
{
  let got : StructNamedFields = StructNamedFields::from((10, true));
  let exp = StructNamedFields{ a : 10 , b : true };
  a_id!( got, exp );
}
