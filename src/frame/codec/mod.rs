mod context;
mod num;

pub(crate) use context::*;

pub(crate) trait Decodable<'a>: Sized {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self>;
}

pub(crate) trait ContextDecodable<'a, C>: Sized {
    fn decode_with(input: &'a [u8], cx: C) -> nom::IResult<&'a [u8], Self>;
}


