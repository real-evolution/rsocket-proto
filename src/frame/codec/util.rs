use nom::combinator::map;

#[inline(always)]
pub(crate) fn map_into<I, O1, O2, E, F>(
    parser: F,
) -> impl FnMut(I) -> nom::IResult<I, O2, E>
where
    O2: From<O1>,
    F: nom::Parser<I, O1, E>,
{
    map(parser, |v| O2::from(v))
}
