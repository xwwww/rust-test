// 1. 禁用标准库
#![no_std] 
#![feature(asm)]
#![feature(panic_info_message)]

// 2. 禁用Rust运行时（无main函数）
#![no_main]

// 3. 导入必要的模块
mod kernel_module;
mod vga_buffer_simple;

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use kernel_module::*;
use vga_buffer_simple::*;

// 自定义入口点
#[no_mangle]  // 防止 Rust 改变函数名
pub extern "C" fn _start() -> ! {
    // 初始化 VGA 缓冲区
    vga_buffer_simple::init();

    // 输出基本文本
    println!("欢迎来到 Blog OS!");
    println!("这是一个用 Rust 编写的简单操作系统。");

    // 演示彩色文本
    {{
        let mut writer = vga_buffer_simple::VgaWriter::new();
        writer.set_color(vga_buffer_simple::Color::Red, vga_buffer_simple::Color::Black);
        writer.write_str("红色文本 ");
        writer.set_color(vga_buffer_simple::Color::Green, vga_buffer_simple::Color::Black);
        writer.write_str("绿色文本 ");
        writer.set_color(vga_buffer_simple::Color::Blue, vga_buffer_simple::Color::Black);
        writer.write_str("蓝色文本");
        writer.set_color(vga_buffer_simple::Color::White, vga_buffer_simple::Color::Black);
        writeln!(writer, "");
    }}

    // 演示光标位置控制
    {{
        let mut writer = vga_buffer_simple::VgaWriter::new();
        writer.set_cursor_position(10, 5);
        writer.write_str("这里是光标控制的演示");
    }}

    // 内核模块演示
    kernel_module_demo();

    // 内核主循环
    loop {{
        // 处理中断、调度任务等
        unsafe {{
            // 简单的hlt指令
            asm!("hlt");
        }}
    }}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 输出 panic 信息到 VGA 缓冲区
    println!("\n!! 内核崩溃 !!");
    if let Some(location) = info.location() {
        println!("位置: {}:{}:{}",
            location.file(),
            location.line(),
            location.column());
    }
    if let Some(message) = info.message() {
        println!("信息: {}", message);
    }

    // 无限循环
    loop {}
}

fn kernel_module_demo() {
    let vga_module = KernelModule::new("VGA_DRIVER");
    
    match register_module(vga_module) {
        Ok(()) => {
            // 模块注册成功
        }
        Err(e) => {
            // 处理错误
        }
    }
    
    match initialize_all_modules() {
        Ok(()) => {
            // 所有模块初始化成功
        }
        Err(e) => {
            // 处理初始化错误
        }
    }
}