#[deprecated]
pub(crate) trait Encodable {
    fn encode<'a, W>(&self, writer: &'a mut W) -> std::io::Result<&'a mut W>
    where
        W: std::io::Write;
}

#[deprecated]
pub(crate) trait ChainedEncoder {
    fn encode<'a, E: super::Encodable>(
        &'a mut self,
        item: &E,
    ) -> std::io::Result<&'a mut Self>;

    fn encode_opt<'a, E: super::Encodable>(
        &'a mut self,
        item: &Option<E>,
    ) -> std::io::Result<&'a mut Self> {
        if let Some(ref item) = item {
            self.encode(item)
        } else {
            Ok(self)
        }
    }
}

impl<W: std::io::Write> ChainedEncoder for W {
    fn encode<'a, E: super::Encodable>(
        &'a mut self,
        item: &E,
    ) -> std::io::Result<&'a mut Self> {
        item.encode(self)
    }
}
