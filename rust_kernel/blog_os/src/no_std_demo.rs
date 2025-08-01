#![no_std]

use core::panic::PanicInfo;

// 基本类型仍然可用
fn basic_types_demo() {
    let x: u32 = 42;
    let y: bool = true;
    let z: char = 'A';
    
    // 数组是可用的
    let arr: [u8; 4] = [1, 2, 3, 4];
    
    // 切片也是可用的
    let slice: &[u8] = &arr[1..3];
}

// Option 和 Result 仍然可用
fn option_result_demo() -> Option<u32> {
    let maybe_value: Option<u32> = Some(42);
    
    match maybe_value {
        Some(val) => Some(val * 2),
        None => None,
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}