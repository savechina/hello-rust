use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};

use memmap2::Mmap;
use tempfile;

pub(crate) fn memmap_file_sample() {
    let mut tmpfile = tempfile::tempfile().expect("failed to open the file");

    println!("tempfile : {:?}", tmpfile);

    write!(tmpfile, "Hello World!").unwrap();

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // Read
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();

    assert_eq!("Hello World!", buf);

    let mmap = unsafe { Mmap::map(&tmpfile).expect("failed to map the file") };

    assert_eq!(b"Hello World!", &mmap[..]);

    //write 
    let mut mm = mmap.make_mut().unwrap();

    (&mut mm[..]).write_all(b"Hello Zen!");
    
    // mm.deref_mut().write_all(b"hello, world!");

    assert_eq!(b"Hello Zen!d!", &mm[..]);
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_mmap_file() {
        memmap_file_sample();
    }
}
