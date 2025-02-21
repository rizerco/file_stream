use std::io::BufRead;

use crate::error::Result;

// Defines a type that can be read from the file stream.
pub trait Readable: Sized {
    /// Reads this type from the buffer using big endian byte order.
    fn read_be(reader: &mut dyn BufRead) -> Result<Self>;

    /// Reads this type from the buffer using little endian byte order.
    fn read_le(reader: &mut dyn BufRead) -> Result<Self>;
}

impl Readable for u8 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }
}

impl Readable for i8 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        Ok(i8::from_be_bytes(buffer))
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        Ok(i8::from_le_bytes(buffer))
    }
}

impl Readable for u16 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }
}

impl Readable for i16 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?;
        Ok(i16::from_le_bytes(buffer))
    }
}

impl Readable for u32 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl Readable for i32 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        Ok(i32::from_be_bytes(buffer))
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }
}

impl Readable for u64 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 8];
        reader.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 8];
        reader.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }
}

impl Readable for i64 {
    fn read_be(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 8];
        reader.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }

    fn read_le(reader: &mut dyn BufRead) -> Result<Self> {
        let mut buffer = [0; 8];
        reader.read_exact(&mut buffer)?;
        Ok(i64::from_le_bytes(buffer))
    }
}
