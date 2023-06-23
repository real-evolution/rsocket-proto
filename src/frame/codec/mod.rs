mod context;
mod num;
mod sequence;
mod util;

pub(crate) use chained::*;
pub(crate) use context::*;
pub(crate) use num::*;
pub(crate) use sequence::*;
pub(crate) use util::*;

pub(crate) trait Decodable<'a>: Sized {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self>;
}

pub(crate) trait ContextDecodable<'a, C>: Sized {
    fn decode_with(input: &'a [u8], cx: C) -> nom::IResult<&'a [u8], Self>;
}
