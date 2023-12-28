#![no_std] // 不使用标准库，应为标准库的实现依赖具体的操作系统
#![no_main] // 不使用rust提供的入口函数，main函数依赖运行时的实现，由于我们要自己编写操作系统，故需要使用硬件系统本身的入口函数
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use bootloader::{BootInfo, entry_point};
// use x86_64::structures::paging::{PageTable, Page};
entry_point!(kernel_main);
extern crate alloc;
use alloc::{
    vec::Vec,
    vec,
    rc::Rc,
    boxed::Box,
};


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use blog_os::memory::active_level_4_table;
    use x86_64::VirtAddr;
    // use x86_64::structures::paging::Translate;
    use blog_os::memory;
    // use x86_64::structures::paging::Page;
    // use x86_64::VirtAddr;
    use blog_os::memory::BootInfoFrameAllocator;
    use blog_os::allocator;
    println!("Hello world {}", "!");
    blog_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset)};

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe {
    //     page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)
    // };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // let x = Box::new(41);
    // println!("heap value at {:p}", x);

    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());

    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let clone_counted = reference_counted.clone();
    // println!("current reference count is {}", Rc::strong_count(&clone_counted));
    // core::mem::drop(reference_counted);
    // println!("reference count is {}", Rc::strong_count(&clone_counted));

    #[cfg(test)]
    test_main();

    println!("It did not crash.");
    blog_os::hlt_loop();
}


// static HELLO: &[u8] = b"Hello World!";

// #[no_mangle] // 在变异代码时，不对函数名称进行转化（保持名称不变）。
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     // let vga_buffer = 0xb8000 as *mut u8;
//     // for (i, &byte) in HELLO.iter().enumerate() {
//     //     unsafe {
//     //         *vga_buffer.offset(i as isize * 2) = byte;
//     //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//     //     }
//     // }
//     // vga_buffer::print_something();

//     // use core::fmt::Write;
//     // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
//     // write!(vga_buffer::WRITER.lock(), ", Some numbers: {} {}", 42, 1.337).unwrap();
//     println!("Hello world{}", "!");

//     blog_os::init();

//     // x86_64::instructions::interrupts::int3();

//     // unsafe {
//     //     *(0xdeadbedf as *mut u8) = 42;
//     // };

//     // fn stack_overflow() {
//     //     stack_overflow();
//     // }
//     // stack_overflow();

//     // let ptr = 0xdeadbeaf as *mut u8;
//     // unsafe { *ptr = 42; }

//     // let ptr = 0x2053ea as *mut u8;

//     // // read from a code page
//     // unsafe { let x = *ptr; }
//     // println!("read worked");

//     // // write to a code page
//     // unsafe { *ptr = 42; }
//     // println!("write worked");

//     use x86_64::registers::control::Cr3;
//     let (level_4_page_table, _) = Cr3::read();
//     println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

//     #[cfg(test)]
//     test_main();

//     println!("It did not crash!");

//     blog_os::hlt_loop();
//     // loop {
//     //     use blog_os::print;
//     //     print!("-");
//     // }
// }

// The standard library provides its own panic handler function, but in a no_std environment we need to define it ourselves:
// 标准库中提供了panic处理函数，所以我们需要定义我们自己的panic处理函数
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    blog_os::hlt_loop();
    // loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    blog_os::test_panic_handler(info)
}
