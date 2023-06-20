use nom::{
    combinator::{cond, map, map_res, rest},
    multi::length_data,
    number::complete::be_u24,
    IResult, Parser, ToUsize,
};

use crate::frame::{Flags, FrameHeader};

#[inline(always)]
pub(super) fn metadata(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    length_data(be_u24)(input)
}

#[inline(always)]
pub(super) fn metadata_opt<'a>(
    header: &FrameHeader,
) -> impl Parser<&'a [u8], Option<&'a [u8]>, nom::error::Error<&'a [u8]>> {
    cond(header.flags.contains(Flags::METADATA), metadata)
}

#[inline(always)]
pub(super) fn rest_opt(input: &[u8]) -> nom::IResult<&[u8], Option<&[u8]>> {
    map(rest::<&[u8], _>, |buf| {
        if buf.is_empty() {
            None
        } else {
            Some(buf)
        }
    })(input)
}

#[inline(always)]
pub(super) fn rest_str<'a, E2, M>(
    mapper: M,
) -> impl Parser<&'a [u8], &'a str, nom::error::Error<&'a [u8]>>
where
    M: Fn(&'a [u8]) -> Result<&'a str, E2>,
{
    map_res(rest, mapper)
}

#[inline(always)]
pub(super) fn length_str<'a, L, N, E2, M>(
    len: L,
    mapper: M,
) -> impl Parser<&'a [u8], &'a str, nom::error::Error<&'a [u8]>>
where
    N: ToUsize,
    L: Parser<&'a [u8], N, nom::error::Error<&'a [u8]>>,
    M: Fn(&'a [u8]) -> Result<&'a str, E2>,
{
    map_res(length_data(len), mapper)
}

#[inline(always)]
pub(super) fn length_utf8<'a, L, N>(
    len: L,
) -> impl Parser<&'a [u8], &'a str, nom::error::Error<&'a [u8]>>
where
    N: ToUsize,
    L: Parser<&'a [u8], N, nom::error::Error<&'a [u8]>>,
{
    length_str(len, std::str::from_utf8)
}

#[inline(always)]
pub(super) fn rest_utf8(input: &[u8]) -> IResult<&[u8], &str> {
    rest_str(std::str::from_utf8).parse(input)
}
