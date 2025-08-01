// 1. 禁用标准库
#![no_std] 

// 2. 禁用Rust运行时（无main函数）
#![no_main]

mod kernel_module;

use core::panic::PanicInfo;
use kernel_module::*;

// 自定义入口点
#[no_mangle]  // 防止 Rust 改变函数名
pub extern "C" fn _start() -> ! {
    // 在这里我们完全控制程序的执行
    
    // 简单的内核启动
    // 稍后我们会添加 VGA 输出功能
    
    // 内核主循环
    loop {
        // 处理中断、调度任务等
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 改进的 panic 处理
    if let Some(location) = info.location() {
        // 这里应该输出到 VGA 缓冲区
        // 现在先用简单的循环
    }
    
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