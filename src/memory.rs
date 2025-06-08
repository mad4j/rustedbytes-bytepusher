#![allow(dead_code)]
use std::ops::{Index, IndexMut, Range};

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    #[inline(always)]
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    #[inline(always)]
    pub fn copy_from(&mut self, offset: usize, rom: &[u8]) {
        self.data[offset..offset+rom.len()].copy_from_slice(rom);
    }

    #[inline(always)]
    pub fn read_24_bits(&self, addr: usize) -> usize {
        ((self.data[addr] as usize) << 16)
            | ((self.data[addr + 1] as usize) << 8)
            | (self.data[addr + 2] as usize)
    }

    #[inline(always)]
    pub fn write_24_bits(&mut self, addr: usize, value: usize) {
        let bytes = [
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ];
        self.data[addr..addr + 3].copy_from_slice(&bytes);
    }

    #[inline(always)]
    pub fn read_16_bits(&self, addr: usize) -> u16 {
        ((self.data[addr] as u16) << 8) | (self.data[addr + 1] as u16)
    }

    #[inline(always)]
    pub fn write_16_bits(&mut self, addr: usize, value: u16) {
        let bytes = value.to_be_bytes();
        self.data[addr..addr + 2].copy_from_slice(&bytes);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(64 * 1024) // Default size of 64KB
    }
}

impl Index<usize> for Memory {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [u8];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<Range<usize>> for Memory {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::Memory;

    #[test]
    fn test_new_memory() {
        let mem = Memory::new(1024);
        assert_eq!(mem.data.len(), 1024);
        assert!(mem.data.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_load_rom() {
        let mut mem = Memory::new(16);
        let rom = [1u8, 2, 3, 4];
        mem.copy_from(0, &rom);
        assert_eq!(&mem.data[..4], &rom);
    }

    #[test]
    fn test_write_24_bits() {
        let mut mem = Memory::new(8);
        mem.write_24_bits(0, 0x123456);

        assert_eq!(mem[0], 0x12);
        assert_eq!(mem[1], 0x34);
        assert_eq!(mem[2], 0x56);
    }

    #[test]
    fn test_read_24_bits() {
        let mut mem = Memory::new(8);
        mem[0] = 0x12;
        mem[1] = 0x34;
        mem[2] = 0x56;

        assert_eq!(mem.read_24_bits(0), 0x123456);
    }

    #[test]
    fn test_read_16_bits() {
        let mut mem = Memory::new(8);
        mem[0] = 0xAB;
        mem[1] = 0xCD;
        assert_eq!(mem.read_16_bits(0), 0xABCD);
    }

    #[test]
    fn test_write_16_bits() {
        let mut mem = Memory::new(8);
        mem.write_16_bits(0, 0xABCD);

        assert_eq!(mem.data[0], 0xAB);
        assert_eq!(mem.data[1], 0xCD);
    }

    #[test]
    fn test_indexing_read() {
        let mut mem = Memory::new(4);
        mem.data[0] = 42;
        assert_eq!(mem[0], 42);
    }

    #[test]
    fn test_indexing_write() {
        let mut mem = Memory::new(4);
        mem[0] = 42;
        assert_eq!(mem.data[0], 42);
    }
}
