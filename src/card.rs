use enums::*;

#[derive(Clone, Debug)]
pub enum CardTypeData {
	Unit {
		ATK : u8,
		HP : u8,
	},
	Spell {},
	Upgrade {},
	Building {
		HP : u8,	
	},
}

#[derive(Clone, Debug)]
pub struct CardData {
	pub name : &'static str,
	pub color : Color,
	pub spec : Option<Spec>,
	pub techlevel : TechLevel,
	pub cost : u8,
	pub target_icon : bool,
	pub rules_text_1 : &'static str,
	pub rules_text_2 : &'static str,
	pub rules_text_3 : &'static str,
	pub flavor_text : &'static str,
	pub subtype : Vec<Subtype>,
	pub ty : CardTypeData,
	pub base_effects : Vec<Effect>,
}

impl CardData {
	pub fn new() -> CardData {
		CardData {
			name : "",
			color : Color::Neutral,
			spec : None,
			techlevel : TechLevel::Tech0,
			cost : 0,
			target_icon : false,
			rules_text_1 : "",
			rules_text_2 : "",
			rules_text_3 : "",
			flavor_text : "",
			subtype : vec![],
			ty : CardTypeData::Unit {
				ATK : 0,
				HP : 0,
			},
			base_effects : vec![],
		}
	}
	pub fn name(&self) -> &'static str {
		&self.name
	}
}

#[derive(Clone, Debug)]
pub struct CardInstance {
	pub data : CardData,
	pub owner : Player,
}

