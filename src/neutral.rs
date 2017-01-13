#![allow(non_snake_case)]
use super::*;
lazy_static! {
	pub static ref OLDER_BROTHER : CardData = CardData {
		name : "Older Brother",
		ty : CardType::Unit,
		color : Color::Neutral,
		spec : None,
		techlevel : TechLevel::Tech0,
		cost : 2,
		target_icon : false,
		rules_text_1 : "",
		rules_text_2 : "",
		rules_text_3 : "",
		ATK : 2,
		HP : 2,
		subtype : vec![],
	};
	pub static ref HELPFUL_TURTLE : CardData = CardData {
		name : "Helpful Turtle",
		ty : CardType::Unit,
		color : Color::Neutral,
		spec : None,
		techlevel : TechLevel::Tech0,
		cost : 2,
		target_icon : false,
		rules_text_1 : "Healing 1",
		rules_text_2 : "",
		rules_text_3 : "",
		ATK : 1,
		HP : 2,
		subtype : vec![],
	};
	pub static ref TIMELY_MESSENGER : CardData = CardData {
		name : "Timely Messenger",
		ty : CardType::Unit,
		color : Color::Neutral,
		spec : None,
		techlevel : TechLevel::Tech0,
		cost : 1,
		target_icon : false,
		rules_text_1 : "Haste",
		rules_text_2 : "",
		rules_text_3 : "",
		ATK : 1,
		HP : 1,
		subtype : vec![Subtype::Mercenary],
	};
	pub static ref TENDERFOOT : CardData = CardData {
		name : "Tenderfoot",
		ty : CardType::Unit,
		color : Color::Neutral,
		spec : None,
		techlevel : TechLevel::Tech0,
		cost : 1,
		target_icon : false,
		rules_text_1 : "",
		rules_text_2 : "",
		rules_text_3 : "",
		ATK : 1,
		HP : 2,
		subtype : vec![],
	};

}

