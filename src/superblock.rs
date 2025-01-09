pub struct Superblock {
    pub version: u32, /*filesystem version*/
    pub magic: u32,   /*to identify the filesystem*/
    pub rinode: u32,  /*root inode*/
    pub finode: u32,  /*free inodes*/
    pub icount: u32,  /*inodes count*/
    pub bcount: u32,  /*blocks count*/
    pub bsize: u32,
    pub fblocks: u32, /*free blocks*/
}

impl Superblock {}
