use std::ops::RangeFrom;

use nom::{
    combinator::{cond, map, map_res, rest, verify},
    multi::length_data,
    number::complete::be_u24,
    Parser,
};

use crate::frame::Flags;

#[inline(always)]
pub(crate) fn none_if_empty<I, E, F>(parser: F) -> impl Parser<I, Option<I>, E>
where
    F: Parser<I, I, E>,
    I: nom::InputLength,
{
    map(parser, |v| if v.input_len() == 0 { None } else { Some(v) })
}

#[inline(always)]
pub(crate) fn length_metadata<'a, I, E>(
    cx: &super::ParseContext<'_>,
) -> impl Parser<I, Option<I>, E> + 'a
where
    I: nom::Slice<RangeFrom<usize>>
        + nom::InputIter<Item = u8>
        + nom::InputLength
        + nom::InputTake
        + 'a,
    E: nom::error::ParseError<I> + 'a,
{
    cond(
        cx.header.flags.contains(Flags::METADATA),
        length_data(be_u24),
    )
}

#[inline(always)]
pub(crate) fn length_ascii<'a, L, N>(
    len: L,
) -> impl Parser<&'a [u8], &'a str, nom::error::Error<&'a [u8]>>
where
    N: nom::ToUsize,
    L: Parser<&'a [u8], N, nom::error::Error<&'a [u8]>>,
{
    map(
        verify(length_data(len), |b: &[u8]| b.is_ascii()),
        |b| unsafe { std::str::from_utf8_unchecked(b) },
    )
}

#[inline(always)]
pub(crate) fn rest_utf8<'a, E>(
    input: &'a [u8],
) -> nom::IResult<&'a [u8], &'a str, E>
where
    E: nom::error::ParseError<&'a [u8]>
        + nom::error::FromExternalError<&'a [u8], std::str::Utf8Error>,
{
    map_res(rest, std::str::from_utf8)(input)
}
