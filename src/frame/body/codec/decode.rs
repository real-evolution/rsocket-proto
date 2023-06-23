pub(crate) trait Decodable<'a>: Sized {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self>;
}

pub(crate) trait ContextDecodable<'a, C>: Sized {
    fn decode_with(input: &'a [u8], cx: C) -> nom::IResult<&'a [u8], Self>;
}

pub(crate) type NomErr<I> = nom::Err<nom::error::Error<I>>;

#[derive(Debug)]
pub(crate) struct ChainedDecoder<'a>(&'a [u8]);

impl<'a> ChainedDecoder<'a> {
    #[inline(always)]
    pub(crate) fn next<D>(&mut self) -> Result<D, NomErr<&'a [u8]>>
    where
        D: super::Decodable<'a>,
    {
        let out;

        (self.0, out) = D::decode(self.0)?;

        Ok(out)
    }

    #[inline(always)]
    pub(crate) fn next_with<D, C>(
        &mut self,
        cx: C,
    ) -> Result<D, NomErr<&'a [u8]>>
    where
        D: super::ContextDecodable<'a, C>,
    {
        let out;

        (self.0, out) = D::decode_with(self.0, cx)?;

        Ok(out)
    }
}

#[inline(always)]
pub(crate) fn decode_chained<'a, F, O>(
    mut mapper: F,
) -> impl FnMut(&'a [u8]) -> nom::IResult<&'a [u8], O>
where
    F: FnMut(&mut ChainedDecoder<'a>) -> Result<O, NomErr<&'a [u8]>>,
{
    move |input| {
        let mut decoder = ChainedDecoder(input);
        let output = mapper(&mut decoder)?;

        Ok((decoder.0, output))
    }
}
