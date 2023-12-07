#![no_std] // 不使用标准库，应为标准库的实现依赖具体的操作系统
#![no_main] // 不使用rust提供的入口函数，main函数依赖运行时的实现，由于我们要自己编写操作系统，故需要使用硬件系统本身的入口函数
use core::panic::PanicInfo;

#[no_mangle] // 在变异代码时，不对函数名称进行转化（保持名称不变）。
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}
