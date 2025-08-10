// VGA 文本缓冲区实现

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// VGA 文本缓冲区的内存地址
const VGA_BUFFER_ADDRESS: *mut u8 = 0xb8000 as *mut u8;

// 屏幕尺寸 (80x25)
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

// 颜色代码
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

// 颜色组合 (前景色 + 背景色)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// VGA 文本字符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// VGA 文本缓冲区
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

// VGA 写入器
pub struct VgaWriter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl VgaWriter {
    // 创建新的 VGA 写入器
    pub fn new() -> Self {
        VgaWriter {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut Buffer) },
        }
    }

    // 写入一个字符
    pub fn write_char(&mut self, c: u8) {
        match c {
            b'\n' => self.new_line(),
            c => {
                if self.column_position >= SCREEN_WIDTH {
                    self.new_line();
                }

                let row = SCREEN_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: c,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    // 写入字符串
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }

    // 换行
    fn new_line(&mut self) {
        for row in 1..SCREEN_HEIGHT {
            for col in 0..SCREEN_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(SCREEN_HEIGHT - 1);
        self.column_position = 0;
    }

    // 清除一行
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..SCREEN_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    // 设置颜色
    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }

    // 移动光标
    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.column_position = x;
            // 注意：这里只更新了x位置，完整实现还需要更新y位置
            // 并且需要与硬件光标同步
        }
    }
}

// 实现 fmt::Write trait
impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

// 静态 VGA 写入器 (使用互斥锁保证线程安全)
lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter::new());
}

// 打印宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::vga_buffer::WRITER.lock(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!('\n')
    }};
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!('\n')
    }};
}

// 初始化函数
pub fn init() {
    // 清除屏幕
    clear_screen();
}

// 清除屏幕
pub fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 0..SCREEN_HEIGHT {
        writer.clear_row(row);
    }
    writer.column_position = 0;
}