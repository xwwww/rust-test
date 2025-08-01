// 1. 禁用标准库
#![no_std] 

// 2. 禁用Rust运行时（无main函数）
#![no_main]

use core::panic::PanicInfo;

// 自定义入口点
#[no_mangle]  // 防止 Rust 改变函数名
pub extern "C" fn _start() -> ! {
    // 在这里我们完全控制程序的执行
    
    // 初始化 VGA 缓冲区（稍后实现）
    vga_buffer::clear_screen();
    vga_buffer::print_string("欢迎来到我的操作系统！");
    
    // 内核主循环
    loop {
        // 处理中断、调度任务等
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 自定义 panic 处理
    vga_buffer::print_string("内核 Panic: ");
    if let Some(location) = info.location() {
        vga_buffer::print_string(&format!(
            "文件: {}, 行: {}", 
            location.file(), 
            location.line()
        ));
    }
    
    loop {}
}

// VGA 缓冲区模块（简化版本）
mod vga_buffer {
    pub fn clear_screen() {
        // 实现屏幕清理
    }
    
    pub fn print_string(s: &str) {
        // 实现字符串打印
    }
}p {}
}