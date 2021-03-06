use crate::core::LqReader;
use liquesco_common::error::LqError;
use std::io::Read;
use std::io::Write;

/// Can be used to get a `LqReader` from a slice of `u8`. It contains the slice and an
/// offset (cursor). It's cheap to clone, since embedded data won't be cloned, only the
/// offset (cursor) will be cloned.
pub struct SliceReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> From<&'a [u8]> for SliceReader<'a> {
    fn from(data: &'a [u8]) -> Self {
        SliceReader { data, offset: 0 }
    }
}

impl<'a> From<&'a Vec<u8>> for SliceReader<'a> {
    fn from(data: &'a Vec<u8>) -> Self {
        SliceReader {
            data: data.as_slice(),
            offset: 0,
        }
    }
}

impl<'a> SliceReader<'a> {
    /// Makes sure the reader has been read completely and there's no additional data.
    pub fn finish(&self) -> Result<(), LqError> {
        if self.offset != self.data.len() {
            LqError::err_new(
                "There's additional data not read from any. The any data must have been consumed
            entirely (for security reasons).",
            )
        } else {
            Result::Ok(())
        }
    }
}

impl<'a> LqReader<'a> for SliceReader<'a> {
    #[inline]
    fn peek_u8(&self) -> Result<u8, LqError> {
        let len = self.data.len();
        if self.offset >= len {
            LqError::err_new("End of reader")
        } else {
            let value = self.data[self.offset];
            Result::Ok(value)
        }
    }

    #[inline]
    fn read_u8(&mut self) -> Result<u8, LqError> {
        let len = self.data.len();
        if self.offset >= len {
            LqError::err_new("End of reader")
        } else {
            let value = self.data[self.offset];
            self.offset += 1;
            Result::Ok(value)
        }
    }

    #[inline]
    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], LqError> {
        let data_len = self.data.len();
        if self.offset + len > data_len {
            LqError::err_new("End of reader")
        } else {
            let end_index = self.offset + len;
            let data = self.data;
            let value = &data[self.offset..end_index];
            self.offset += len;
            Result::Ok(value)
        }
    }

    /// Note: This is a cheap operation since the binary data won't be cloned, only the offset.
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            offset: self.offset,
        }
    }
}

impl<'a> Read for SliceReader<'a> {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let slice = self
            .read_slice(len)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        buf.write(slice)
    }
}
