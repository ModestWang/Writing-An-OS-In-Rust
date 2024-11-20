/*
 * @FilePath: serial.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-11-20 17:51:28
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-11-20 18:34:02
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: Print to serial port
 */
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

// 使用 lazy_static 和一个自旋锁来创建一个 static writer实例。
// 通过使用 lazy_static ，我们可以保证 init 方法只会在该示例第一次被使用使被调用。
lazy_static! {
    /// # SERIAL1
    /// 串口 1 的全局接口,
    ///
    /// I/O 端口: 0x3F8,
    ///
    /// 波特率: 115200,
    ///
    /// 数据位: 8 位,
    ///
    /// 停止位: 1 位
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// 实现打印的函数，该函数将使用串口接口打印到主机
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// 通过串口接口打印到主机
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// 通过串口接口打印到主机并换行
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
