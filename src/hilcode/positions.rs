use crate::hilcode::id::Id;
use ::bit_set::BitSet;
use ::std::fmt::Debug;
use ::std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FirstPos;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LastPos;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FollowPos;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StartPos;

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Positions<POS>(BitSet, PhantomData<POS>);

#[coverage(off)]
impl<POS> Debug for Positions<POS> {
	fn fmt(
		&self,
		formatter: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		formatter.debug_tuple("Positions").field(&self.0).finish()
	}
}

impl<POS> Default for Positions<POS> {
	fn default() -> Self {
		Positions(BitSet::default(), PhantomData)
	}
}

pub(crate) fn new_first_pos() -> Positions<FirstPos> {
	Positions::<FirstPos>(BitSet::new(), PhantomData)
}

pub(crate) fn new_last_pos() -> Positions<LastPos> {
	Positions::<LastPos>(BitSet::new(), PhantomData)
}

impl<POS> Positions<POS> {
	#[cfg(test)]
	pub(crate) fn new(elements: &[usize]) -> Positions<POS> {
		let mut bit_set: BitSet = BitSet::new();
		elements.iter().for_each(|element: &usize| {
			bit_set.insert(*element);
		});
		Positions(bit_set, PhantomData)
	}

	pub(crate) fn union_first_pos(
		self: &mut Self,
		additional_first_pos: &Positions<FirstPos>,
	) {
		self.0.union_with(&additional_first_pos.0);
	}

	pub(crate) fn union_last_pos(
		self: &mut Self,
		additional_last_pos: &Positions<LastPos>,
	) {
		self.0.union_with(&additional_last_pos.0);
	}

	pub(crate) fn for_each<FUNCTION>(
		self: &Self,
		mut function: FUNCTION,
	) where
		FUNCTION: FnMut(Id),
	{
		self.0.iter().for_each(|id: usize| {
			let id: Id = Id::new(id);
			function(id)
		});
	}
}

impl<FirstPos> Positions<FirstPos> {
	pub(crate) fn insert(
		self: &mut Self,
		id: Id,
	) {
		self.0.insert(id.0);
	}

	pub(crate) fn to_last_pos(self: &Self) -> Positions<LastPos> {
		Positions::<LastPos>(self.0.clone(), PhantomData)
	}

	pub(crate) fn to_start_pos(self: &Self) -> Positions<StartPos> {
		Positions::<StartPos>(self.0.clone(), PhantomData)
	}
}
