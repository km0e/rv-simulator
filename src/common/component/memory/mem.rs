use crate::common::abi::*;
use std::collections::HashMap;

const PAGE_SIZE: usize = 0x1000;

#[derive(Debug)]
pub struct Mem {
    data: HashMap<usize, [u8; PAGE_SIZE]>,
}
impl Mem {
    pub fn with_data(addr: usize, data: Vec<u8>) -> Self {
        let mut mem = HashMap::new();
        let mut moffset = 0;
        let offset = addr % PAGE_SIZE;
        let mut size = PAGE_SIZE - (offset);
        size = if size > data.len() { data.len() } else { size };
        let mut page = [0; PAGE_SIZE];
        page[offset..offset + size].copy_from_slice(&data[moffset..moffset + size]);
        moffset += size;
        mem.insert(addr / PAGE_SIZE, page);
        while moffset + PAGE_SIZE < data.len() {
            let mut page = [0; PAGE_SIZE];
            page.copy_from_slice(&data[moffset..moffset + PAGE_SIZE]);
            mem.insert((addr + moffset) / PAGE_SIZE, page);
            moffset += PAGE_SIZE;
        }
        page[..data.len() - moffset].copy_from_slice(&data[moffset..]);
        mem.insert((addr + moffset) / PAGE_SIZE, page);
        Self { data: mem }
    }
}
impl Mem {
    pub fn write(&mut self, addr: usize, data: u32) {
        let page = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;
        let page = self.data.entry(page).or_insert_with(|| [0; PAGE_SIZE]);
        page[offset] = (data & 0xff) as u8;
        page[offset + 1] = ((data >> 8) & 0xff) as u8;
        page[offset + 2] = ((data >> 16) & 0xff) as u8;
        page[offset + 3] = ((data >> 24) & 0xff) as u8;
    }
}
impl IndexPort for Mem {
    fn read(&self, addr: usize) -> u32 {
        let page = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;
        if let Some(page) = self.data.get(&page) {
            u32::from_ne_bytes([
                page[offset],
                page[offset + 1],
                page[offset + 2],
                page[offset + 3],
            ])
        } else {
            0
        }
    }
}
