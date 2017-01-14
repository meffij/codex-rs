use super::*;

#[derive(Clone, Debug)]
struct BuildingEntity {
	card : Option<CardInstance>,
	controller : Player,
	ty : CardType,
	HP : u8,
}

#[derive(Clone, Debug)]
struct Entity {
	card : Option<CardInstance>,
	controller : Player,
	ty : CardTypeData,
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
	builtThisTurn : bool,
	disabled : bool,
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
