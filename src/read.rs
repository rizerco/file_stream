pub mod readable;

use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::path::Path;

use crate::error::*;
pub use readable::*;

/// The file stream allows for incremental reading of data
/// from the start of the file to the end.
pub struct FileStreamReader {
    /// The reader.
    reader: Box<dyn BufRead>,
}

// MARK: Creation

impl FileStreamReader {
    /// Creates a new file stream by opening a file.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let reader = Box::new(reader);
        let file_stream = Self { reader };
        Ok(file_stream)
    }

    /// Creates a new file stream from data.
    pub fn from_data(data: Vec<u8>) -> Result<Self> {
        let cursor = Cursor::new(data);
        let reader = Box::new(cursor);
        let file_stream = Self { reader };
        Ok(file_stream)
    }
}

// MARK: Reading

impl FileStreamReader {
    /// Reads data from the file with a big endian byte order.
    pub fn read_be<T: Readable>(&mut self) -> Result<T> {
        T::read_be(self.reader.as_mut())
    }

    /// Reads data from the file with a little endian byte order.
    pub fn read_le<T: Readable>(&mut self) -> Result<T> {
        T::read_le(&mut self.reader)
    }

    /// Skips the specified number of bytes of the stream.
    pub fn skip_bytes(&mut self, length: usize) -> Result<()> {
        _ = self.read_bytes(length)?;
        Ok(())
    }

    /// Reads and returns bytes.
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; length];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Reads a string with UTF8 encoding.
    pub fn read_string(&mut self, length: usize) -> Result<String> {
        let bytes = self.read_bytes(length)?;
        let string = String::from_utf8(bytes)?;
        Ok(string)
    }

    /// Reads and returns all of the remaining data.
    pub fn read_remaining_data(&mut self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
