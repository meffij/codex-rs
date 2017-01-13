#![allow(dead_code)]
#![allow(non_snake_case)]
#[macro_use]
extern crate lazy_static;

mod neutral;

#[derive(Clone, Copy, Debug)]
pub struct Player(u8);

#[derive(Clone, Copy, Debug)]
pub enum Phase {
	Upkeep,
	Main,
	EOT,
}

#[derive(Clone, Copy, Debug)]
pub enum CardType {
	Unit,
	Spell,
	Building,
	Upgrade,
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
	Neutral,
	Red,
	Green,
	Blue,
	Black,
	Purple,
	White,
}

#[derive(Clone, Copy, Debug)]
pub enum Spec {
	Bashing,
	Finesse,
	Anarchy,
	Blood,
	Fire,
	Balance,
	Feral,
	Growth,
	Law,
	Peace,
	Truth,
	Demonology,
	Disease,
	Necromancy,
	Discipline,
	Strength,
	Ninjutsu,
	Past,
	Present,
	Future,
}

#[derive(Clone, Copy, Debug)]
pub enum TechLevel {
	Tech0,
	Tech1,
	Tech2,
	Tech3,
}

#[derive(Clone, Copy, Debug)]
pub enum Subtype {
	Virtuoso,
	Mercenary,
}

pub trait Card {
	fn color() -> Color;
	fn ty() -> CardType;
}

#[derive(Clone, Debug)]
pub struct CardData {
	name: &'static str,
	ty: CardType,
	color: Color,
	spec: Option<Spec>,
	techlevel: TechLevel,
	cost: u8,
	target_icon: bool,
	rules_text_1 : &'static str,
	rules_text_2 : &'static str,
	rules_text_3 : &'static str,
	// flavor_text : &'static str,
	ATK : u8,
	HP : u8,
	subtype : Vec<Subtype>,
}

impl CardData {
	fn new() -> CardData {
		CardData {
			name : "",
			ty : CardType::Unit,
			color : Color::Neutral,
			spec : None,
			techlevel : TechLevel::Tech0,
			cost : 0,
			target_icon : false,
			rules_text_1 : "",
			rules_text_2 : "",
			rules_text_3 : "",
			ATK : 0,
			HP : 0,
			subtype : vec![],
		}
	}
}

lazy_static! {
	pub static ref BASECARDDATA : CardData = CardData::new();
}

lazy_static! {
	pub static ref BASENEUTRALDATA : CardData = CardData { 
		color : Color::Neutral,
		.. BASECARDDATA.clone()
	};
}

#[derive(Clone, Debug)]
struct CardInstance {
	data : CardData,
	owner : Player,
}

trait EntityInterface {
	fn getcard() -> Option<CardInstance>;
}

#[derive(Clone, Debug)]
struct BuildingEntity {
	card : Option<CardInstance>,
	controller : Player,
	ty : CardType,
	HP : u8,
}

#[derive(Clone, Debug)]
enum Entity {
	UnitEntity(UnitEntity),
}

#[derive(Clone, Debug)]
struct Deck {
	deck : Vec<CardInstance>,
	top : Option<CardInstance>,
}

#[derive(Clone, Copy, Debug)]
struct BuildingStats {
	health : u8,
	builtBefore : bool,
}

#[derive(Clone, Debug)]
struct Buildings {
	basehealth : u8,
	stats : [BuildingStats ; 3],
}

#[derive(Clone, Debug)]
struct PlayerState {
	gold : u8,
	deck : Deck,
	hand : Vec<CardInstance>,
	buildings : Buildings,
}

#[derive(Clone, Debug)]
struct GameState {
	field : Vec<Entity>,
	ps : Vec<PlayerState>,
}

fn main() {
  println!("Hello, world!");
	let a = BASECARDDATA.clone();
	println!("{:?}", a.name);
}

#[test]
fn test_base() {
	let a = BASECARDDATA.clone();
	assert!(a.name == "");
}

#[test]
fn test_build() {
	let a = CardData {
		name : "Timely Messenger", 
		.. BASECARDDATA.clone()
	};
	assert!(a.name == "Timely Messenger");
	let neutral_template = CardData {
		color : Color::Neutral,
		.. BASECARDDATA.clone()
	};
}
