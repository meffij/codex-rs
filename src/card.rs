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
	pub name: &'static str,
	pub ty: CardType,
	pub color: Color,
	pub spec: Option<Spec>,
	pub techlevel: TechLevel,
	pub cost: u8,
	pub target_icon: bool,
	pub rules_text_1 : &'static str,
	pub rules_text_2 : &'static str,
	pub rules_text_3 : &'static str,
	pub flavor_text : &'static str,
	pub ATK : u8,
	pub HP : u8,
	pub subtype : Vec<Subtype>,
}

impl CardData {
	pub fn new() -> CardData {
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
			flavor_text : "",
			ATK : 0,
			HP : 0,
			subtype : vec![],
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

