use std::io::{self, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct BE<T> {
    num: T,
}

impl<T> BE<T> {
    #[inline]
    fn new(num: T) -> Self {
        Self { num }
    }

    #[inline]
    fn into_inner(self) -> T {
        self.num
    }
}

macro_rules! impl_be {
    ($type:ty, $read_fn_name:ident, $write_fn_name:ident) => {
        impl BE<$type> {
            #[inline]
            fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
                let num = reader.$read_fn_name::<BigEndian>()?;
                Ok(BE::new(num))
            }

            #[inline]
            fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                writer.$write_fn_name::<BigEndian>(self.num)?;
                Ok(())
            }
        }
    };
}

impl BE<u8> {
    fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let num = reader.read_u8()?;
        Ok(BE::new(num))
    }

    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8(self.num)?;
        Ok(())
    }
}

impl BE<i8> {
    fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let num = reader.read_i8()?;
        Ok(BE::new(num))
    }

    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i8(self.num)?;
        Ok(())
    }
}

impl_be!(u16, read_u16, write_u16);
impl_be!(i16, read_i16, write_i16);

impl_be!(u32, read_u32, write_u32);
impl_be!(i32, read_i32, write_i32);

impl_be!(u64, read_u64, write_u64);
impl_be!(i64, read_i64, write_i64);

impl_be!(u128, read_u128, write_u128);
impl_be!(i128, read_i128, write_i128);
