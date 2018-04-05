use std::cmp::Ordering;
use std::cmp::PartialOrd;

pub trait ClockTime: Sized + PartialOrd + Copy {
	fn first() -> Self;
	fn next(&self) -> Self;
	fn last() -> Self;
}

#[derive(PartialEq, Eq, Ord, Copy, Clone, Debug)]
pub struct Trivial;
impl PartialOrd for Trivial {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}
impl ClockTime for Trivial {
	fn first() -> Self {
		Trivial {}
	}
	fn next(&self) -> Self {
		Trivial {}
	}

	fn last() -> Self {
		Trivial {}
	}
}



// #[derive(PartialOrd, Ord, Copy, Clone, Debug)]
// pub struct U8ClockTime(u8);
// impl ClockTime for U8ClockTime {}
