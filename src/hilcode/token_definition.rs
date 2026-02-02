use crate::hilcode::offset::AbsOffset;
use ::imstr::ImString;

pub trait TokenDefinition: Clone + Ord {
	type This;

	fn start_offset(self: &Self) -> AbsOffset;

	fn text(self: &Self) -> &ImString;

	fn sentinels() -> &'static [&'static str];

	fn skip_invalid(source: &ImString) -> ImString {
		let mut index_of_first_sentinel: Option<(usize, &str)> = None;
		for sentinel in Self::sentinels() {
			match source.find(sentinel) {
				Some(index_of_sentinel) => match index_of_first_sentinel {
					None => {
						index_of_first_sentinel = Option::Some((index_of_sentinel, sentinel));
					}

					Some((current_index_of_first_sentinel, _)) => {
						if current_index_of_first_sentinel > index_of_sentinel {
							index_of_first_sentinel = Option::Some((index_of_sentinel, sentinel));
						}
					}
				},

				None => {
					// Nothing to do.
				}
			}
		}
		match index_of_first_sentinel {
			Some((index, sentinel)) => {
				if index == 0 {
					return source.slice(0..sentinel.len());
				} else {
					return source.slice(0..index);
				}
			}

			None => {
				return source.clone();
			}
		}
	}

	fn error(
		message: impl Into<ImString>,
		start_offset: AbsOffset,
		text: &ImString,
	) -> Self::This;
}
