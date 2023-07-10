use recode::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug, Default)]
pub struct ComposedCodec<D, C> {
    decoder: D,
    encoder: C,
}

impl<D, C> Decoder for ComposedCodec<D, C>
where
    D: Decoder,
{
    type Error = D::Error;
    type Item = D::Item;

    #[inline]
    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        self.decoder.decode(src)
    }
}

impl<D, C> Encoder<D::Item> for ComposedCodec<D, C>
where
    D: Decoder,
    C: Encoder<D::Item>,
{
    type Error = C::Error;

    #[inline]
    fn encode(
        &mut self,
        item: D::Item,
        dst: &mut recode::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        self.encoder.encode(item, dst)
    }
}
