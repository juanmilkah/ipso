pub struct IBitmap {
    pub bitmap: Vec<u64>,
    pub icount: u64, /*inode count*/
}

pub struct DBitmap {
    pub bitmap: Vec<u64>,
    pub bcount: u64, /*blocks count*/
}

impl IBitmap {
    pub fn new(inode_count: u64) -> Self {
        let bitmap_size = (inode_count + 63) / 64;

        Self {
            bitmap: vec![0; bitmap_size as usize],
            icount: inode_count,
        }
    }

    pub fn set(&mut self, inode_id: u64) {
        let index = inode_id / 64;
        let bit_index = inode_id % 64;
        self.bitmap[index as usize] |= 1 << bit_index;
    }

    pub fn clear(&mut self, inode_id: u64) {
        let index = inode_id / 64;
        let bit_index = inode_id % 64;
        self.bitmap[index as usize] &= 1 << bit_index;
    }

    pub fn is_set(&self, inode_id: u64) -> bool {
        let index = inode_id / 64;
        let bit_index = inode_id % 64;
        self.bitmap[index as usize] & (1 << bit_index) != 0
    }

    pub fn find_first_available(&self) -> Option<u64> {
        for (i, &block) in self.bitmap.iter().enumerate() {
            if block != u64::MAX {
                for j in 0..64 {
                    if block & (1 << j) == 0 {
                        return Some((i * 64 + j) as u64);
                    }
                }
            }
        }
        None
    }
}

impl DBitmap {
    pub fn new(block_count: u64) -> Self {
        let bitmap_size = (block_count + 63) / 64;

        Self {
            bitmap: vec![0; bitmap_size as usize],
            bcount: block_count,
        }
    }

    pub fn set(&mut self, block_id: u64) {
        let index = block_id / 64;
        let bit_index = block_id % 64;
        self.bitmap[index as usize] |= 1 << bit_index;
    }

    pub fn clear(&mut self, block_id: u64) {
        let index = block_id / 64;
        let bit_index = block_id % 64;
        self.bitmap[index as usize] &= 1 << bit_index;
    }

    pub fn is_set(&self, block_id: u64) -> bool {
        let index = block_id / 64;
        let bit_index = block_id % 64;
        self.bitmap[index as usize] & (1 << bit_index) != 0
    }

    pub fn find_first_available(&self) -> Option<u64> {
        for (i, &block) in self.bitmap.iter().enumerate() {
            if block != u64::MAX {
                for j in 0..64 {
                    if block & (1 << j) == 0 {
                        return Some((i * 64 + j) as u64);
                    }
                }
            }
        }
        None
    }
}
