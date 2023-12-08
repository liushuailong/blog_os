#![no_std] // 不使用标准库，应为标准库的实现依赖具体的操作系统
#![no_main] // 不使用rust提供的入口函数，main函数依赖运行时的实现，由于我们要自己编写操作系统，故需要使用硬件系统本身的入口函数
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 在变异代码时，不对函数名称进行转化（保持名称不变）。
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

// The standard library provides its own panic handler function, but in a no_std environment we need to define it ourselves:
// 标准库中提供了panic处理函数，所以我们需要定义我们自己的panic处理函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}
