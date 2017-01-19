use super::Timestamp;
use enums::*;
use gamestate::*;
#[derive(Clone, Debug)]
pub struct HeroData {
	name : &'static str,
	subtype : &'static str,
	spec : Spec,
	target_icon : bool,
	level1ATK : u8,
	level1HP : u8,
	level1rulestext : &'static str,
	midbandlevel : u8,
	midbandATK : u8,
	midbandHP : u8,
	midbandrulestext : &'static str,
	maxbandlevel : u8,
	maxbandATK : u8,
	maxbandHP : u8,
	maxbandrulestext : &'static str,
}

lazy_static! {
	pub static ref TROQ : HeroData = HeroData {
		name : "Troq Bashar",
		subtype : "Renegade Beast",
		spec : Spec::Bashing,
		target_icon : true,
		level1ATK : 2,
		level1HP : 3,
		level1rulestext : "",
		midbandlevel : 5,
		midbandATK : 3,
		midbandHP : 4,
		midbandrulestext : "Attacks: deal 1 damage to that opponent's base",
		maxbandlevel : 8,
		maxbandATK : 4,
		maxbandHP : 5,
		maxbandrulestext : "Readiness",
	};
	pub static ref RIVER : HeroData = HeroData {
		name : "River Montoya",
		subtype : "Dancing Fencer",
		spec : Spec::Finesse,
		target_icon : true,
		level1ATK : 2,
		level1HP : 3,
		level1rulestext : "",
		midbandlevel : 3,
		midbandATK : 2,
		midbandHP : 4,
		midbandrulestext : "Exhaust: Sideline a tech 0 or tech 1 patroller.",
		maxbandlevel : 5,
		maxbandATK : 3,
		maxbandHP : 4,
		maxbandrulestext : "Your tech 0 units cost (1) less to play",
	};
	
}

#[derive(Clone, Debug)]
pub struct HeroInstance {
	owner : Player,
	data : HeroData,
}

#[test]
fn create_hero_test() {
	let h = Entity {
        timestamp : Timestamp(1),
		controller : Player(0),
		card : Some(EntityCard::Hero(HeroInstance {
			owner : Player(0),
			data : TROQ.clone(),
		})),
		effects : vec![],
        td : EntityTypeData::Hero {},
	};
}
