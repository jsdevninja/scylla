#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unreachable_patterns)]
#![allow(unused_mut)]

pub fn _main() -> i32
{
  let mut x: u32 = 240u32;
  return ((x & 15u32) != 0u32) as i32
}
fn main() { assert_eq!(0, _main()) }
