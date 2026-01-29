#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Id(pub usize);

impl Id {
	pub(crate) fn new(id: usize) -> Self {
		Self(id)
	}
}
