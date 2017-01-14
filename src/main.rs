#![allow(dead_code)]
#![allow(non_snake_case)]
#[macro_use]
extern crate lazy_static;

mod neutral;

mod enums;
use enums::*;

mod card;
use card::*;

mod gamestate;
use gamestate::*;

#[derive(Clone, Debug)]
pub struct Timestamp(u32);

fn main() {

}
