#![no_std] // 不使用标准库，应为标准库的实现依赖具体的操作系统
#![no_main] // 不使用rust提供的入口函数，main函数依赖运行时的实现，由于我们要自己编写操作系统，故需要使用硬件系统本身的入口函数
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;


// static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 在变异代码时，不对函数名称进行转化（保持名称不变）。
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", Some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello world{}", "!");

    blog_os::init();

    // x86_64::instructions::interrupts::int3();

    // unsafe {
    //     *(0xdeadbedf as *mut u8) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    blog_os::hlt_loop();
    // loop {
    //     use blog_os::print;
    //     print!("-");
    // }
}

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
