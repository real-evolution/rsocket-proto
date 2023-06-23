mod num;

pub(crate) trait Decodable<'a>: Sized {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self>;
}

pub(crate) trait ContextDecodable<'a, C>: Sized {
    fn decode_with(input: &'a [u8], cx: C) -> nom::IResult<&'a [u8], Self>;
}

pub(crate) trait Encodable {
    fn encode<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write;
}
