/*
 * @FilePath: lib.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-11-20 18:35:41
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-11-20 18:46:21
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Lib
 */
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga_buffer;
use core::panic::PanicInfo;

/// QEMU 退出代码
/// 用于告诉 QEMU 运行成功或失败
/// 0x10: 成功
/// 0x11: 失败
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// 退出 QEMU
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    // 该函数在 0xf4 处创建了一个新的端口，该端口同时也是 isa-debug-exit 设备的 iobase。
    // 然后它会向端口写入传递的退出代码。这里我们使用 u32 来传递数据，
    // 因为我们之前已经将 isa-debug-exit 设备的 iosize 指定为4字节了。
    // 上述两个操作都是 unsafe 的，因为I/O端口的写入操作通常会导致一些不可预知的行为。
}

/// 测试表
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// 测试 run
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// 测试 panic 处理
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// 程序入口点
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

/// panic 处理
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
