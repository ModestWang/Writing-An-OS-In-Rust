/*
 * @FilePath: vga_buffer.rs
 * @Author: ModestWang 1598593280@qq.com
 * @Date: 2024-11-19 21:40:37
 * @LastEditors: ModestWang
 * @LastEditTime: 2024-11-20 18:28:12
 * 2024 by ModestWang, All Rights Reserved.
 * @Descripttion: VGA text buffer
 */
use core::fmt;
use volatile::Volatile;

/// # WRITER(全局接口)
/// 全局 Writer 实例
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// # VGA 文本模式的颜色
/// 4 位前景色，4 位背景色
///
/// 0: 黑色
/// 1: 蓝色
/// 2: 绿色
/// 3: 青色
/// 4: 红色
/// 5: 洋红色
/// 6: 棕色
/// 7: 亮灰色
/// 8: 暗灰色
/// 9: 亮蓝色
/// 10: 亮绿色
/// 11: 亮青色
/// 12: 亮红色
/// 13: 粉红色
/// 14: 黄色
/// 15: 白色
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// 屏幕字符的颜色和 ASCII 字符
/// 一个字节表示颜色，一个字节表示 ASCII 字符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// # ScreenChar
/// 屏幕上的一个字符
/// 包含 ASCII 字符和颜色
/// 一个字节表示颜色，一个字节表示 ASCII 字符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,   // ASCII 字符
    color_code: ColorCode, // 颜色
}

const BUFFER_HEIGHT: usize = 25; // 缓冲区高度
const BUFFER_WIDTH: usize = 80; // 缓冲区宽度

/// 文本缓冲区
/// 二维数组，每个元素是一个字符
/// BUFFER_HEIGHT 行，BUFFER_WIDTH 列
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// VGA 文本模式的缓冲区
/// 位于 0xb8000
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// 在 VGA 文本模式缓冲区的指定位置写入一个字符
    /// 如果当前行写满，换行
    /// 如果写入字符是换行符，换行
    /// 如果写入字符是可打印字符，写入字符
    /// 如果写入字符是不可打印字符，写入空格
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // 换行符
            b'\n' => self.new_line(),

            // 可打印 ASCII 码
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// 在 VGA 文本模式缓冲区的指定位置写入一个字符串
    /// 如果写入字符是可打印字符，写入字符
    /// 如果写入字符是不可打印字符，写入空格
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可打印 ASCII 码或换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // 不可打印 ASCII 码
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// 换行
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// 清空一行
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// $$ 实现 core::fmt::Write trait；用于 write! 宏
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/* ============================== 以下用于测试 ============================== */

/// 打印一些内容
/// 用于测试
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
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

/// 简单测试
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

/// 多次测试
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

/// 测试输出
#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);

    // 比较打印结果与输入是否一致
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
