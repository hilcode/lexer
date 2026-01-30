use crate::hilcode::offset::AbsOffset;
use ::imstr::ImString;

pub(crate) type TokenBuilder<TOKEN> = fn(AbsOffset, &ImString) -> Result<TOKEN, String>;
