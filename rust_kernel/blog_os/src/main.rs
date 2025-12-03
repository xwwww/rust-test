// 1. 禁用标准库
#![no_std] 

// 2. 禁用Rust运行时（无main函数）
#![no_main]

// 3. 导入必要的模块
mod kernel_module;
mod vga_buffer_simple;

use core::arch::asm;
use core::panic::PanicInfo;
use kernel_module::kernel_module_demo_export;

// vga_buffer_simple 的功能通过完全限定名称使用

// 屏幕尺寸常量在vga_buffer_simple.rs中定义

// 自定义入口点
#[no_mangle]  // 防止 Rust 改变函数名
pub extern "C" fn _start() -> ! {
    // 初始化 VGA 缓冲区
    vga_buffer_simple::init();

    // 输出基本文本 - 使用println!宏（在vga_buffer_simple中实现）
    println!("Welcome to Blog OS!");
    println!("This is a simple operating system written in Rust.");
    println!("使用方向键移动光标，输入字符测试输入功能。");

    // 演示代码已经简化，主要功能通过kernel_module_demo_export展示

    // 调用内核模块演示
    kernel_module_demo_export();

    // 内核主循环
    loop {{        
        // 使用全局VGA写入器实例
        let writer = vga_buffer_simple::VgaWriter::get_global();
        
        // 处理键盘输入
        let _handled = vga_buffer_simple::handle_keyboard_input(writer);
        
        // 使用nop指令代替hlt，确保持续轮询
        unsafe {{                
            asm!("nop");
        }}
    }}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 输出 panic 信息到 VGA 缓冲区
    println!("\n!! KERNEL PANIC !!");
    if let Some(location) = info.location() {
        println!("Location: {}:{}:{}",
            location.file(),
            location.line(),
            location.column());
    }
    let message = info.message();
    if message.as_str() != Some("") {
        println!("Message: {}", message);
    }

    // 无限循环
    loop {}
}

// 直接使用kernel_module.rs中导出的函数