/*
 * @FilePath: main.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-09-14 21:11:21
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-11-19 22:35:41
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */
#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

mod vga_buffer;
use core::panic::PanicInfo;

/// # 程序入口点
/// 由于我们禁用了 std，所以不能使用 `main` 这个函数名(默认命名为 `_start`)
/// 我们需要告诉 Rust 编译器这是程序入口函数
/// 这个函数不会返回，所以返回值类型是 `!`
/// 这个函数是一个裸函数，没有栈展开和异常处理
/// 这个函数是一个 extern 函数，因此它遵循 C 调用约定
/// 因为链接器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "ModestWang");
    loop {}
}

/// Hello World
fn print_hello() {
    static HELLO: &[u8] = b"Hello World!";

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

/// # Panic 函数
/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
