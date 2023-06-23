use derive_more::Deref;
use nom::combinator::verify;

use crate::frame::codec::Decodable;

pub type NonZero<T> = Number<T, false>;

#[derive(Debug, Clone, Deref)]
#[repr(transparent)]
pub struct Number<T, const ALLOW_ZERO: bool = true>(T);

pub trait NumTraits: Sized {
    fn zero() -> Self;
    fn min() -> Self;
    fn max() -> Self;
}

macro_rules! impl_unit {
    ($t:ty: dec => $dec:ident, enc => $($enc:tt)*) => {
        impl NumTraits for $t {
            fn zero() -> Self {
                Default::default()
            }

            fn min() -> Self {
                Self::min_value()
            }

            fn max() -> Self {
                Self::max_value()
            }
        }

        impl<'a, const ALLOW_ZERO: bool> Decodable<'a> for Number<$t, ALLOW_ZERO> {
            fn decode(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
                use nom::number::complete::$dec;

                let (r, value) = if ALLOW_ZERO {
                    $dec(input)?
                } else {
                    verify($dec, |&v| v != <$t>::zero())(input)?
                };

                Ok((r, Self(value)))
            }
        }

        impl<const ALLOW_ZERO: bool> Number<$t, ALLOW_ZERO> {
            #[allow(unused)]
            pub(crate) fn encode<'b, W: std::io::Write>(
                &self,
                writer: &'b mut W,
            ) -> std::io::Result<&'b mut W> {
                use byteorder::{WriteBytesExt, BE};

                writer.$($enc)*(self.0)?;

                Ok(writer)
            }
        }
    };
}

impl_unit!(u8: dec => be_u8, enc => write_u8);
impl_unit!(u16: dec => be_u16, enc => write_u16::<BE>);
impl_unit!(u32: dec => be_u32, enc => write_u32::<BE>);
impl_unit!(u64: dec => be_u64, enc => write_u64::<BE>);
impl_unit!(u128: dec => be_u128, enc => write_u128::<BE>);
