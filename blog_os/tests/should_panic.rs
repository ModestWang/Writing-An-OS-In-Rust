/*
 * @FilePath: should_panic.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-11-20 18:49:27
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-11-20 18:59:03
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion:
 */
#![no_std]
#![no_main]

use blog_os::serial_print;
use blog_os::{exit_qemu, serial_println, QemuExitCode};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}
