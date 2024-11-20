/*
 * @FilePath: basic_boot.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-11-20 18:33:10
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-11-20 18:43:55
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */
// 集成测试的二进制文件在非测试模式下不会被编译构建
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
