// 简单的VGA文本缓冲区实现
// 不依赖外部库，直接使用内联汇编



use core::arch::asm;
use core::fmt;

// 端口I/O函数 - 定义在文件顶部，确保在使用前可用
unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

// VGA文本缓冲区的内存地址
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

// VGA 光标控制端口
const VGA_CURSOR_COMMAND_PORT: u16 = 0x3D4;
const VGA_CURSOR_DATA_PORT: u16 = 0x3D5;

// 光标位置寄存器
const CURSOR_LOCATION_HIGH: u8 = 0x0E;
const CURSOR_LOCATION_LOW: u8 = 0x0F;

// 屏幕尺寸
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

// 颜色代码
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

// 位置转换函数
fn position_to_offset(x: usize, y: usize) -> u16 {
    (y * SCREEN_WIDTH + x) as u16
}

// 颜色组合
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// VGA文本字符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// VGA文本缓冲区
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

// 全局VGA写入器实例
pub static mut GLOBAL_WRITER: Option<VgaWriter> = None;

// VGA写入器
pub struct VgaWriter {
    column_position: usize,
    row_position: usize,        // 新增：当前行位置
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    cursor_enabled: bool,       // 新增：光标是否启用
}

impl VgaWriter {
    // 创建新的VGA写入器
    pub fn new() -> Self {
        let writer = VgaWriter {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(VGA_BUFFER as *mut Buffer) },
            cursor_enabled: true,
        };
        writer.update_hardware_cursor();
        writer
    }
    
    // 获取全局写入器实例
    pub fn get_global() -> &'static mut Self {
        // 安全地获取全局写入器实例
        unsafe {
            if GLOBAL_WRITER.is_none() {
                GLOBAL_WRITER = Some(VgaWriter::new());
            }
            GLOBAL_WRITER.as_mut().unwrap()
        }
    }

    // 写入一个字符
    pub fn write_char(&mut self, c: u8) {
        match c {
            b'\n' => self.new_line(),
            b'\r' => {
                self.column_position = 0;
                self.update_hardware_cursor();
            },
            b'\t' => {
                // 制表符支持（4个空格对齐）
                let spaces = 4 - (self.column_position % 4);
                for _ in 0..spaces {
                    self.write_char(b' ');
                }
            },
            b'\x08' => {
                // 退格键支持
                if self.column_position > 0 {
                    self.column_position -= 1;
                    self.write_char_at(b' ', self.column_position, self.row_position);
                    self.update_hardware_cursor();
                }
            },
            c => {
                if self.column_position >= SCREEN_WIDTH {
                    self.new_line();
                }
                
                self.write_char_at(c, self.column_position, self.row_position);
                self.column_position += 1;
                self.update_hardware_cursor();
            }
        }
    }
    
    // 在指定位置写入字符
    fn write_char_at(&mut self, c: u8, x: usize, y: usize) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.buffer.chars[y][x] = ScreenChar {
                ascii_character: c,
                color_code: self.color_code,
            };
        }
    }

    // 更新硬件光标位置
    fn update_hardware_cursor(&self) {
        if !self.cursor_enabled {
            return;
        }
        
        let position = position_to_offset(self.column_position, self.row_position);
        
        unsafe {
            // 设置光标位置的高字节
            outb(VGA_CURSOR_COMMAND_PORT, CURSOR_LOCATION_HIGH);
            outb(VGA_CURSOR_DATA_PORT, (position >> 8) as u8);
            
            // 设置光标位置的低字节
            outb(VGA_CURSOR_COMMAND_PORT, CURSOR_LOCATION_LOW);
            outb(VGA_CURSOR_DATA_PORT, position as u8);
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
        if self.row_position >= SCREEN_HEIGHT - 1 {
            // 滚动屏幕
            self.scroll_up();
        } else {
            self.row_position += 1;
        }
        self.column_position = 0;
        self.update_hardware_cursor();
    }
    
    // 屏幕向上滚动
    fn scroll_up(&mut self) {
        for row in 1..SCREEN_HEIGHT {
            for col in 0..SCREEN_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(SCREEN_HEIGHT - 1);
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

    // 移动光标到指定位置
    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.column_position = x;
            self.row_position = y;
            self.update_hardware_cursor();
        }
    }

    // 获取当前光标位置
    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.column_position, self.row_position)
    }

    // 相对移动光标
    pub fn move_cursor(&mut self, dx: isize, dy: isize) {
        let new_x = (self.column_position as isize + dx).max(0).min(SCREEN_WIDTH as isize - 1) as usize;
        let new_y = (self.row_position as isize + dy).max(0).min(SCREEN_HEIGHT as isize - 1) as usize;
        self.set_cursor_position(new_x, new_y);
    }
    
    // 启用/禁用光标
    pub fn set_cursor_enabled(&mut self, enabled: bool) {
        self.cursor_enabled = enabled;
        if enabled {
            self.update_hardware_cursor();
        } else {
            // 将光标移到屏幕外隐藏
            unsafe {
                outb(VGA_CURSOR_COMMAND_PORT, CURSOR_LOCATION_HIGH);
                outb(VGA_CURSOR_DATA_PORT, 0xFF);
                outb(VGA_CURSOR_COMMAND_PORT, CURSOR_LOCATION_LOW);
                outb(VGA_CURSOR_DATA_PORT, 0xFF);
            }
        }
    }

    // 显示光标
    pub fn show_cursor(&mut self) {
        self.set_cursor_enabled(true);
    }

    // 设置光标形状
    pub fn set_cursor_shape(&mut self, start: u8, end: u8) {
        unsafe {
            outb(0x3D4, 0x0A);
            outb(0x3D5, start);
            outb(0x3D4, 0x0B);
            outb(0x3D5, end);
        }
    }
}

// 实现fmt::Write trait
impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_char(byte);
        }
        Ok(())
    }
}

// 键盘相关常量
const KEYBOARD_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

// 键盘扫描码
const KEY_SCANCODE_UP: u8 = 0x48;
const KEY_SCANCODE_DOWN: u8 = 0x50;
const KEY_SCANCODE_LEFT: u8 = 0x4B;
const KEY_SCANCODE_RIGHT: u8 = 0x4D;

// 这些函数现在已经取消注释，以便可以被外部调用
// 检查键盘是否有输入
pub fn has_keyboard_input() -> bool {
    unsafe {
        inb(KEYBOARD_STATUS_PORT) & 0x01 != 0
    }
}

// 读取键盘输入的扫描码
pub fn read_keyboard_scancode() -> u8 {
    unsafe {
        inb(KEYBOARD_PORT)
    }
}

// 处理键盘输入并控制光标
// 返回值：是否处理了键盘输入
pub fn handle_keyboard_input(writer: &mut VgaWriter) -> bool {
    let mut handled = false;
    // 直接读取键盘状态和扫描码
    unsafe {
        // 检查键盘是否有输入
        let status = inb(KEYBOARD_STATUS_PORT);
        
        if status & 0x01 != 0 {
            let scancode = inb(KEYBOARD_PORT);
            handled = true;
            
            // 只处理按下键的扫描码
            if scancode & 0x80 == 0 {
                // 直接处理扫描码
                match scancode {
                    // 方向键
                    0x48 => writer.move_cursor(0, -1),  // 上
                    0x50 => writer.move_cursor(0, 1),   // 下
                    0x4B => writer.move_cursor(-1, 0),  // 左
                    0x4D => writer.move_cursor(1, 0),   // 右
                    // 特殊键
                    0x0E => writer.write_char(b'\x08'), // 退格键
                    0x1C => writer.write_char(b'\n'),   // 回车键
                    0x29 => writer.write_char(b' '),    // 空格键
                    // 字母键 - 基本字母
                    0x10 => writer.write_char(b'q'),
                    0x11 => writer.write_char(b'w'),
                    0x12 => writer.write_char(b'e'),
                    0x13 => writer.write_char(b'r'),
                    0x14 => writer.write_char(b't'),
                    0x1E => writer.write_char(b'a'),
                    0x1F => writer.write_char(b's'),
                    0x20 => writer.write_char(b'd'),
                    0x21 => writer.write_char(b'f'),
                    0x22 => writer.write_char(b'g'),
                    // 其他键暂时不处理
                    _ => {
                        // 其他键暂时不处理
                    }
                }
            }
        }
    }
    handled
}

// 打印宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let writer = $crate::vga_buffer_simple::VgaWriter::get_global();
        let _ = write!(writer, $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n")
    }};
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n")
    }};
}

// 初始化键盘控制器 - 更完整的版本
fn init_keyboard() {
    unsafe {
        // 1. 等待键盘控制器准备就绪
        let mut i = 0;
        while (inb(KEYBOARD_STATUS_PORT) & 0x02) != 0 {
            i += 1;
            if i > 1000 { break; } // 避免无限循环
        }
        
        // 2. 读取并丢弃可能存在的任何未处理的输入
        for _ in 0..20 {
            if (inb(KEYBOARD_STATUS_PORT) & 0x01) != 0 {
                inb(KEYBOARD_PORT);
            }
        }
        
        // 3. 确保键盘控制器处于正常状态
        // 发送命令：读取键盘控制器状态
        outb(KEYBOARD_STATUS_PORT, 0x20);
        
        // 4. 再次等待键盘控制器准备就绪
        i = 0;
        while (inb(KEYBOARD_STATUS_PORT) & 0x02) != 0 {
            i += 1;
            if i > 1000 { break; } // 避免无限循环
        }
        
        // 5. 再次读取并丢弃可能存在的任何未处理的输入
        for _ in 0..10 {
            if (inb(KEYBOARD_STATUS_PORT) & 0x01) != 0 {
                inb(KEYBOARD_PORT);
            }
        }
    }
}

// 初始化函数
pub fn init() {
    // 初始化全局VGA写入器实例
    let writer = VgaWriter::new();
    unsafe {
        GLOBAL_WRITER = Some(writer);
    }
    
    // 获取全局写入器实例
    let writer = VgaWriter::get_global();
    
    // 清除屏幕
    for row in 0..SCREEN_HEIGHT {
        writer.clear_row(row);
    }
    
    // 设置光标到屏幕左上角
    writer.set_cursor_position(0, 0);
    
    // 确保光标可见
    writer.show_cursor();
    
    // 初始化键盘控制器
    init_keyboard();
    
    // 输出使用说明
    writer.write_str("Blog OS启动成功！\n");
    writer.write_str("键盘输入测试模式已启用。\n");
    writer.write_str("按下任何键将显示其扫描码。\n");
    writer.write_str("支持的键：方向键、基本字母(qwertyasdfg)、退格、回车、空格\n");
}