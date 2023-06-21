use std::ops::RangeFrom;

use nom::combinator::verify;

macro_rules! non_zero_parser {
    ($e:ident => $t:ident) => {
        paste::paste! {
            #[allow(unused)]
            pub(crate) fn [<non_zero_ $e _ $t>]<I, E>(
                input: I,
            ) ->  nom::IResult<I, $t, E>
            where
                I: nom::Slice<RangeFrom<usize>>
                    + nom::InputIter<Item = u8>
                    + nom::InputLength
                    + nom::InputTake
                    + Clone,
                E: nom::error::ParseError<I>,
            {
                verify(nom::number::complete::[<$e _ $t>], |&v| v != 0)(input)
            }
        }
    };
}

non_zero_parser!(be => u8);
non_zero_parser!(be => u16);
non_zero_parser!(be => u32);
non_zero_parser!(be => u64);
