use std::io::Write;

use crate::error::*;

// Defines a type that can be written to a file stream.
pub trait Writable: Sized {
    /// Writes this type to the buffer using big endian byte order.
    fn write_be(&self, writer: &mut dyn Write) -> Result<()>;

    /// Writes this type to the buffer using little endian byte order.
    fn write_le(&self, writer: &mut dyn Write) -> Result<()>;
}

impl Writable for u8 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i8 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for u16 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i16 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for u32 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i32 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for u64 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i64 {
    fn write_be(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_be_bytes())?;
        Ok(())
    }

    fn write_le(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write(&self.to_le_bytes())?;
        Ok(())
    }
}
