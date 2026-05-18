#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

pub fn _main() -> i32
{
  let mut x: i32 = 1i32;
  return (if x != 0i32 { 0i32 } else { 1i32 })
}
fn main() { assert_eq!(0, _main()) }
