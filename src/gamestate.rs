use super::*;
use enums::*;

#[derive(Clone, Debug)]
pub enum EntityTypeData {
	Unit {},
	Spell {},
	Building {},
	Upgrade {},
}

#[derive(Clone, Debug)]
pub struct Entity {
	timestamp : Timestamp,
	card : Option<CardInstance>,
	controller : Player,
	effects : Vec<Effect>,
}

#[derive(Clone, Debug)]
pub struct Deck {
	deck : Vec<CardInstance>,
	top : Option<CardInstance>,
}

#[derive(Clone, Copy, Debug)]
pub struct BuildingStats {
	health : u8,
	builtBefore : bool,
	builtThisTurn : bool,
	disabled : bool,
}

#[derive(Clone, Debug)]
pub struct Buildings {
	basehealth : u8,
	stats : [BuildingStats ; 3],
}

#[derive(Clone, Debug)]
pub struct PlayerState {
	gold : u8,
	deck : Deck,
	hand : Vec<CardInstance>,
	buildings : Buildings,
}

#[derive(Clone, Debug)]
pub struct GameState {
	field : Vec<Entity>,
	ps : Vec<PlayerState>,
}
