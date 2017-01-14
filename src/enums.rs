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

#[derive(Clone, Debug)]
enum Effect {

}
