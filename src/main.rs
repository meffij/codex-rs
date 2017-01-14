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

/*
lazy_static! {
	pub static ref BASECARDDATA : CardData = CardData::new();
}

lazy_static! {
	pub static ref BASENEUTRALDATA : CardData = CardData { 
		color : Color::Neutral,
		.. BASECARDDATA.clone()
	};
}
*/

fn main() {
	/*
  println!("Hello, world!");
	let a = BASECARDDATA.clone();
	println!("{:?}", a.name());
	*/
}

/*
#[test]
fn test_base() {
	let a = BASECARDDATA.clone();
	assert!(a.name() == "");
}

#[test]
fn test_build() {
	let a = CardData {
		name : "Timely Messenger", 
		.. BASECARDDATA.clone()
	};
	assert!(a.name() == "Timely Messenger");
	let neutral_template = CardData {
		color : Color::Neutral,
		.. BASECARDDATA.clone()
	};
}
*/
