#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(slice_fill)]
global_asm!(include_str!("entry.asm"));

extern crate alloc;

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod interrupt;
mod memory;
mod algorithm;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);
    interrupt::init();
    memory::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }

    extern "C" {
        fn kernel_end();
    }
    println!("kernel_end = {:#x}", kernel_end as usize);
    println!("_kernel_end = {:#x}", (kernel_end as usize) / 4096);
    //println!("{}", *memory::config::KERNEL_END_ADDRESS);

    for _ in 0..2 {
        let frame_0 = match memory::frame::allocator::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::allocator::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }
    println!("xxxxxxxxxxxxxxxxxx");


    /// 1.完成页表的映射
    println!("{}", *memory::config::KERNEL_END_ADDRESS);

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    // remap.activate();

    // println!("{:?}",remap.mapping);



    interrupt::timer::init();

    loop {}
}

