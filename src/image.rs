use std::{
    fs::File,
    io::{BufReader, Error, Read},
};

use crate::memory::Memory;

pub fn load_image_file(path: &str, mem: &mut Memory) -> Result<(), Error> {
    let file = File::open(path)?;
    load_image(BufReader::new(file), mem)
}

fn load_image(mut reader: impl Read, mem: &mut Memory) -> Result<(), Error> {
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    let mut iter = buf.iter();
    let mut addr: usize;
    if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
        addr = (((*first as u16) << 8) | *second as u16) as usize;
    } else {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "could not read first two bytes of data",
        ));
    }

    while let Some(byte) = iter.next() {
        let mut value = (*byte as u16) << 8;
        if let Some(byte) = iter.next() {
            value |= *byte as u16;
        }
        mem.set(addr, value);
        addr += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image() -> Result<(), Error> {
        // first two bytes = where to store the program in memory
        // data is stored in big endian, so image bytes must be swapped
        // 0x01 0x02 => 0x0001
        // 0x02 0x00 => 0x0002
        // 0xFF 0x00 => 0x00FF
        // 0x00 0xFF => 0xFF00
        // 0x12 0x34 => 0x3412
        let image = [0x00, 0x01, 0x02, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x12, 0x34];
        let mut mem = Memory::new();
        load_image(&image[..], &mut mem)?;
        assert_eq!(
            mem.get_range(0..5),
            [0x0000, 0x0200, 0xFF00, 0x00FF, 0x1234]
        );
        Ok(())
    }
}
