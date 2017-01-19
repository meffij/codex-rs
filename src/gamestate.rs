use super::*;
use enums::*;
use hero::*;

#[derive(Clone, Debug)]
pub enum EntityTypeData {
	Unit {},
	Spell {},
	Building {},
	Upgrade {},
    Hero {},
}

#[derive(Clone, Debug)]
pub enum EntityCard {
    Hero(HeroInstance),
    Card(CardInstance),
}


#[derive(Clone, Debug)]
pub struct Entity {
	pub timestamp : Timestamp,
	pub card : Option<EntityCard>,
	pub controller : Player,
	pub effects : Vec<Effect>,
    pub td : EntityTypeData,
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
pub struct HeroSlot {
    instance : HeroInstance,
    turnsTillActive : u8,
}

#[derive(Clone, Debug)]
pub struct PlayerState {
	gold : u8,
	deck : Deck,
	hand : Vec<CardInstance>,
	buildings : Buildings,
    heros : Vec<HeroSlot>,
}

#[derive(Clone, Debug)]
pub struct GameState {
	field : Vec<Entity>,
	ps : Vec<PlayerState>,
}
