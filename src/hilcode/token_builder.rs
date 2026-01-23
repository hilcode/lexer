use ::imstr::ImString;

pub(crate) type TokenBuilder<TOKEN> = fn(&ImString) -> TOKEN;
