pub mod writable;

use std::io::{Cursor, Write};

use crate::error::*;
pub use writable::*;

/// The file stream writer allows for incremental writing of data
/// to a file.
pub struct FileStreamWriter {
    /// The cursor to write to.
    cursor: Cursor<Vec<u8>>,
}

// MARK: Creation

impl FileStreamWriter {
    /// Creates a new file stream writer.
    pub fn new() -> Self {
        let cursor = Cursor::new(Vec::new());
        Self { cursor }
    }
}

// MARK: Writing

impl FileStreamWriter {
    /// Writes data to the file with a big endian byte order.
    pub fn write_be<T: Writable>(&mut self, value: &T) -> Result<()> {
        value.write_be(&mut self.cursor)
    }

    /// Writes data to the file with a little endian byte order.
    pub fn write_le<T: Writable>(&mut self, value: &T) -> Result<()> {
        value.write_le(&mut self.cursor)
    }

    /// Writes raw bytes.
    pub fn write_bytes(&mut self, data: &[u8]) -> Result<()> {
        self.cursor.write_all(data)?;
        Ok(())
    }

    /// Writes zeros to the file for the specified length.
    pub fn write_zeros(&mut self, length: usize) -> Result<()> {
        let buffer = vec![0; length];
        self.cursor.write_all(&buffer)?;
        Ok(())
    }

    /// Returns the data written to the stream.
    pub fn data(&self) -> &[u8] {
        self.cursor.get_ref()
    }
}
