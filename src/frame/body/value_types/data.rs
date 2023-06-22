use derive_more::Deref;

#[derive(Debug, Deref)]
#[repr(transparent)]
pub struct Data<'a>(&'a [u8]);

impl<'a> Data<'a> {
    pub(crate) fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::combinator::rest;

        rest(input).map(|(r, o)| (r, Self(o)))
    }

    pub(crate) fn encode<'b, W: std::io::Write>(
        &self,
        writer: &'b mut W,
    ) -> std::io::Result<&'b mut W> {
        writer.write_all(self.0)?;

        Ok(writer)
    }
}
