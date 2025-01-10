use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom, Write};

use serde::{Deserialize, Serialize};

use crate::superblock::bincode_err_to_io_err;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct Inode {
    pub identifier: u32,    /*inode number*/
    pub mode: u32,          /*can this file be read/written/executed*/
    pub ftype: FileType,    /*file type*/
    pub uid: u32,           /*who owns this file*/
    pub size: u64,          /*file size in bytes*/
    pub time: u64,          /*last accessed*/
    pub ctime: u64,         /*created at*/
    pub mtime: u64,         /*modified at*/
    pub dtime: u64,         /*time this inode is deleted*/
    pub gid: u32,           /*owner group*/
    pub dblocks: [u32; 12], /*direct blocks*/
    pub iblocks: [u32; 3],  /*indirect blocks*/
    pub lcount: u32,        /*direct links count*/
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum FileType {
    #[default]
    RegularFile,
    Directory,
    SymLink,
}

impl Inode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        self.size > 0 && self.identifier > 0 && self.mode > 0
    }

    pub fn get_identifier(&self) -> u32 {
        self.identifier
    }

    pub fn set_identifier(&mut self, id: u32) {
        self.identifier = id;
    }

    pub fn read_from_disk(&self, path: &str, offset: u64) -> Result<Self> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(offset))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let inode = bincode::deserialize(&buffer).map_err(bincode_err_to_io_err)?;
        Ok(inode)
    }

    pub fn write_to_disk(&mut self, path: &str, offset: u64) -> Result<()> {
        let mut file = File::create(path)?;
        file.seek(SeekFrom::Start(offset))?;

        let buffer = bincode::serialize(self).map_err(bincode_err_to_io_err)?;

        file.write_all(&buffer)?;
        Ok(())
    }

    pub fn get_file_type(&self) -> FileType {
        self.ftype.clone()
    }

    pub fn set_file_type(&mut self, t: u32) {
        let ftype = match t {
            0 => Some(FileType::RegularFile),
            1 => Some(FileType::Directory),
            2 => Some(FileType::SymLink),
            _ => None,
        };

        if let Some(t) = ftype {
            self.ftype = t;
        }
    }

    pub fn get_file_permissions(&self) -> u32 {
        self.mode
    }

    pub fn set_file_permissions(&mut self, mode: u32) {
        self.mode = mode;
    }

    pub fn get_owner(&self) -> u32 {
        self.uid
    }

    pub fn set_owner(&mut self, id: u32) {
        self.uid = id;
    }

    pub fn get_group(&self) -> u32 {
        self.gid
    }

    pub fn set_group(&mut self, id: u32) {
        self.gid = id;
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn get_direct_blocks(&self) -> &[u32] {
        &self.dblocks
    }

    pub fn set_direct_blocks(&mut self, blocks: [u32; 12]) {
        self.dblocks.copy_from_slice(&blocks);
    }

    pub fn get_indirect_blocks(&self) -> &[u32] {
        &self.iblocks
    }

    /*3 indirect blocks*/
    pub fn set_indirect_blocks(&mut self, blocks: [u32; 3]) {
        self.iblocks.copy_from_slice(&blocks);
    }

    pub fn get_creation_time(&self) -> u64 {
        self.ctime
    }

    pub fn set_creation_time(&mut self, t: u64) {
        self.ctime = t;
    }

    pub fn get_mod_time(&self) -> u64 {
        self.mtime
    }

    pub fn set_mod_time(&mut self, t: u64) {
        self.mtime = t;
    }

    pub fn get_links_count(&self) -> u32 {
        self.lcount
    }

    pub fn set_links_count(&mut self, count: u32) {
        self.lcount = count;
    }

    pub fn get_last_access(&self) -> u64 {
        self.time
    }

    pub fn set_last_access(&mut self, time: u64) {
        self.time = time;
    }

    pub fn get_deletion_time(&self) -> u64 {
        self.dtime
    }

    pub fn set_deletion_time(&mut self, time: u64) {
        self.dtime = time;
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn inode_tests_work() {
        assert_eq!(true, true);
    }

    #[test]
    fn loading_inode() {
        let dir = tempdir().unwrap();
        let filepath = dir.path().join("inode.bin");
        let filepath = filepath.to_string_lossy().to_string();

        let mut inode = Inode::new();

        inode.write_to_disk(&filepath, 0).unwrap();

        let loaded = inode.read_from_disk(&filepath, 0).unwrap();

        assert_eq!(inode, loaded);
    }
}
