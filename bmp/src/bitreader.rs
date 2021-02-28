use std::io;
use std::io::Read;

pub struct BitReader<T> {
    read: T,
    buf: [u8; 1],
    current: u8,
    mask: u8,
}

impl <T: Read> BitReader<T> {
    pub fn new(read: T) -> BitReader<T> {
        BitReader {
            read,
            buf: [0; 1],
            current: 0,
            mask: 0,
        }
    }

    pub fn read_bit(&mut self) -> io::Result<Option<bool>> {
        if self.mask == 0 {
            if let None = self.read_next_byte()? {
                return Ok(None)
            }
        }

        let bit = (self.current & self.mask) != 0;
        self.mask >>= 1;

        Ok(Some(bit))
    }

    pub fn read_next_byte(&mut self) -> io::Result<Option<()>> {
        let bytes_read = self.read.read(&mut self.buf)?;

        if bytes_read == 0 {
            return Ok(None);
        }

        self.current = self.buf[0];
        self.mask = 1 << 7;
        
        Ok(Some(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_reader() {
        let buf = &[] as &[u8];
        let mut bit_reader = BitReader::new(buf);

        for _ in 0..100 {
            assert!(bit_reader.read_bit().unwrap().is_none());
        }
    }

    #[test]
    fn test_reader() {
        let buf = &[0b10101010 as u8, 0b11110000, 0b11111111, 0b00000000] as &[u8];
        let mut bit_reader = BitReader::new(buf);

        for _ in 0..4 {
            assert!(bit_reader.read_bit().unwrap().unwrap());
            assert!(!bit_reader.read_bit().unwrap().unwrap());
        }
        for _ in 0..4 {
            assert!(bit_reader.read_bit().unwrap().unwrap());
        }
        for _ in 0..4 {
            assert!(!bit_reader.read_bit().unwrap().unwrap());
        }
        for _ in 0..8 {
            assert!(bit_reader.read_bit().unwrap().unwrap());
        }
        for _ in 0..8 {
            assert!(!bit_reader.read_bit().unwrap().unwrap());
        }
        for _ in 0..100 {
            assert!(bit_reader.read_bit().unwrap().is_none());
        }
    }
}
