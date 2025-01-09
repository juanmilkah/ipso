pub struct Inode {
    pub identifier: u32,   /*inode number*/
    pub mode: u32,         /*can this file be read/written/executed*/
    pub ftype: FileType,   /*file type*/
    pub uid: u32,          /*who owns this file*/
    pub size: u64,         /*file size in bytes*/
    pub time: u64,         /*last accessed*/
    pub ctime: u64,        /*created at*/
    pub mtime: u64,        /*modified at*/
    pub dtime: u64,        /*time this inode is deleted*/
    pub gid: u32,          /*owner group*/
    pub dblocks: Vec<u32>, /*direct blocks*/
    pub iblocks: Vec<u32>, /*indirect blocks*/
    pub lcount: u32,       /*direct links count*/
}

pub enum FileType {
    RegularFile,
    Directory,
    SymLink,
}

impl Inode {}
