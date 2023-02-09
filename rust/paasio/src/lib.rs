use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    bytes_through: usize,
    reads: usize,
    wrapped: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            bytes_through: 0,
            reads: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        let n = self.wrapped.read(buf)?;
        self.bytes_through += n;
        Result::Ok(n)
    }
}

pub struct WriteStats<W> {
    bytes_through: usize,
    writes: usize,
    wrapped: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            bytes_through: 0,
            writes: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        let n = self.wrapped.write(buf)?;
        self.bytes_through += n;
        Result::Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
