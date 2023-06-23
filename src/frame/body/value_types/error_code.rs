use super::super::{Decodable, Encodable};

#[derive(Debug, Clone)]
#[from_to_repr::from_to_other(base_type = u32)]
pub enum ErrorCode {
    InvalidSetup = 0x00000001,
    UnsupportedSetup = 0x00000002,
    RejectSetup = 0x00000003,
    RejectResume = 0x00000004,
    ConnectionError = 0x00000101,
    ConnectionClose = 0x00000102,
    ApplicationError = 0x00000201,
    Rejected = 0x00000202,
    Canceled = 0x00000203,
    Invalid = 0x00000204,
    Other(u32),
}

impl<'a> Decodable<'a> for ErrorCode {
    fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        use nom::number::complete::be_u32;

        let (r, error_code) = be_u32(input)?;

        Ok((r, Self::from_base_type(error_code)))
    }
}

impl Encodable for ErrorCode {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write,
    {
        use byteorder::{WriteBytesExt, BE};

        writer.write_u32::<BE>(self.to_base_type())?;

        Ok(writer)
    }
}
