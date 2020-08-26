//! 文件系统
//!
//! 将读取第一个块设备作为根文件系统
use crate::kernel::Condvar;
use alloc::{sync::Arc, vec::Vec};
use core::any::Any;
use lazy_static::lazy_static;
use rcore_fs_sfs::SimpleFileSystem;
use spin::Mutex;

mod config;
mod inode_ext;
mod stdin;
mod stdout;
mod device;
pub use config::*;
pub use inode_ext::INodeExt;
pub use rcore_fs::{dev::block_cache::BlockCache, vfs::*};
pub use stdin::STDIN;
pub use stdout::STDOUT;

lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        let device={
            extern "C"{
                fn _user_img_start();
                fn _user_img_end();
            }

            let start=_user_img_start as usize;
            let end=_user_img_end as usize;

            println!("{:x}",start);
            Arc::new(
                unsafe{
                    device::MemBuf::new(start, end)
                }
            )
        };
            let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
            sfs.root_inode()
    };
}

/// 触发 [`static@ROOT_INODE`] 的初始化并打印根目录内容
pub fn init() {
    ROOT_INODE.ls();
    println!("mod fs initialized");
}
