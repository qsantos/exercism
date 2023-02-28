use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    it: std::iter::Cycle<std::slice::Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        let key = key.as_ref();
        let it = key.iter().cycle();
        Xorcism { key, it }
    }
    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for (d, k) in data.iter_mut().zip(&mut self.it) {
            *d ^= k;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
    {
        let v: Vec<u8> = data
            .into_iter()
            .zip(&mut self.it)
            .map(|(d, k)| d.borrow() ^ k)
            .collect();
        v.into_iter()
    }
}
