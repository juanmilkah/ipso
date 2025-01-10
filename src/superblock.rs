use std::fs::File;
use std::io::{self, Read, Result, Seek, SeekFrom, Write};

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct Superblock {
    pub version: u32, /*filesystem version*/
    pub magic: u32,   /*to identify the filesystem*/
    pub rinode: u32,  /*root inode*/
    pub finodes: u32, /*free inodes*/
    pub icount: u32,  /*inodes count*/
    pub i_size: u32,  /*inode size*/
    pub bcount: u32,  /*blocks count*/
    pub bsize: u32,   /*block size*/
    pub fblocks: u32, /*free blocks*/
}

impl Superblock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_from_disk(&self, path: &str, offset: u64) -> Result<Self> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(offset))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let superblock = bincode::deserialize(&buffer).map_err(bincode_err_to_io_err)?;
        Ok(superblock)
    }

    pub fn write_to_disk(&mut self, path: &str, offset: u64) -> Result<()> {
        let mut file = File::create(path)?;
        file.seek(SeekFrom::Start(offset))?;

        let buffer = bincode::serialize(self).map_err(bincode_err_to_io_err)?;
        file.write_all(&buffer)?;
        Ok(())
    }

    pub fn get_free_blocks(&self) -> u32 {
        self.fblocks
    }

    pub fn set_free_blocks(&mut self, blocks: u32) {
        self.fblocks = blocks;
    }

    pub fn get_free_inodes(&self) -> u32 {
        self.finodes
    }

    pub fn set_free_inodes(&mut self, inodes: u32) {
        self.finodes = inodes;
    }

    pub fn get_block_size(&self) -> u32 {
        self.bsize
    }

    pub fn set_block_size(&mut self, size: u32) {
        self.bsize = size;
    }

    pub fn get_inode_size(&self) -> u32 {
        self.i_size
    }

    pub fn set_inode_size(&mut self, size: u32) {
        self.i_size = size;
    }

    pub fn get_total_blocks(&self) -> u32 {
        self.bcount
    }

    pub fn set_total_blocks(&mut self, count: u32) {
        self.bcount = count;
    }

    pub fn get_total_inodes(&self) -> u32 {
        self.icount
    }

    pub fn set_total_inodes(&mut self, count: u32) {
        self.icount = count;
    }

    pub fn is_valid(&self) -> bool {
        self.magic == 0xDEADBEEF && self.version >= 1
    }

    pub fn get_root_inode(&self) -> u32 {
        self.rinode
    }

    pub fn set_root_inode(&mut self, inode: u32) {
        self.rinode = inode;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub fn get_magic(&self) -> u32 {
        self.magic
    }

    pub fn set_magic(&mut self, magic: u32) {
        self.magic = magic;
    }
}

pub fn bincode_err_to_io_err(err: bincode::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

#[cfg(test)]
mod tests {

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn superblock_tests_work() {
        assert_eq!(true, true);
    }

    #[test]
    fn loading_superblock() {
        let dir = tempdir().expect("Failed to create temp dir");
        let filepath = dir.path().join("superblock.bin");
        let filepath = filepath.to_string_lossy().to_string();
        File::create_new(&filepath).unwrap();

        let mut sp = Superblock::new();
        sp.set_version(1);
        sp.set_magic(0xDEADBEEF);

        sp.write_to_disk(&filepath, 0).unwrap();

        let loaded = sp.read_from_disk(&filepath, 0).unwrap();

        assert_eq!(sp, loaded);
        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.magic, 0xDEADBEEF);
    }
}
