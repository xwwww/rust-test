# 第二天学习指南：Rust 系统编程基础

## 学习目标 🎯
- 深入理解 `#![no_std]` 和 `#![no_main]` 的含义
- 掌握裸机编程的核心概念
- 理解内存安全与所有权在系统编程中的应用
- 学会处理系统级错误和异常

## 时间分配 ⏰
- **理论学习** (25分钟)
- **代码实践** (25分钟) 
- **实验和调试** (10分钟)

---

## 第一部分：理解 `#![no_std]` 属性 (15分钟)

### 1. 什么是 `#![no_std]`？

```rust
#![no_std]  // 告诉 Rust 不要链接标准库

// 标准库 vs 核心库
// std: 包含文件系统、网络、线程等操作系统功能
// core: 只包含基础类型、trait、宏等不依赖操作系统的功能
```

### 2. `#![no_std]` 的影响

**不可用的功能：**
```rust
// ❌ 这些在 no_std 环境中不可用
use std::vec::Vec;           // 动态数组
use std::collections::HashMap; // 哈希表
use std::thread;             // 线程
use std::fs::File;           // 文件操作
use std::net::TcpStream;     // 网络
```

**仍然可用的功能：**
```rust
// ✅ 这些在 no_std 环境中可用
use core::mem;               // 内存操作
use core::ptr;               // 指针操作
use core::slice;             // 切片
use core::option::Option;    // 选项类型
use core::result::Result;    // 结果类型
```

### 3. 实践：理解差异

创建文件 `src/no_std_demo.rs`：
```rust
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
```

---

## 第二部分：理解 `#![no_main]` 属性 (10分钟)

### 1. 什么是 `#![no_main]`？

```rust
#![no_main]  // 告诉 Rust 不要使用标准的 main 函数入口

// 标准程序的入口点
fn main() {  // ❌ 在 no_main 环境中不会被调用
    println!("Hello, world!");
}

// 系统级程序的入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {  // ✅ 真正的入口点
    // 内核初始化代码
    loop {}
}
```

### 2. 为什么需要 `#![no_main]`？

**标准程序启动过程：**
```
BIOS/UEFI → 操作系统 → 运行时库 → main()
```

**内核程序启动过程：**
```
BIOS/UEFI → Bootloader → _start()
```

### 3. 实践：自定义入口点

修改 `src/main.rs`：
```rust
#![no_std]
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
}
```

---

## 第三部分：内存安全与所有权 (15分钟)

### 1. 系统编程中的内存安全挑战

**传统 C 语言的问题：**
```c
// C 语言中的常见内存错误
char* buffer = malloc(100);
// ... 使用 buffer ...
free(buffer);
// buffer 现在是悬垂指针！

char* ptr = buffer;  // 使用已释放的内存 - 未定义行为！
```

**Rust 的解决方案：**
```rust
// Rust 在编译时防止这些错误
fn memory_safety_demo() {
    let buffer = [0u8; 100];  // 栈上分配
    let slice = &buffer[0..50];  // 借用检查确保安全
    
    // 编译器确保 slice 不会超过 buffer 的生命周期
} // buffer 和 slice 在这里自动清理
```

### 2. 所有权在系统编程中的应用

```rust
// 内存管理示例
struct MemoryRegion {
    start: usize,
    size: usize,
}

impl MemoryRegion {
    fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }
    
    // 获取内存区域的所有权
    fn take_ownership(self) -> usize {
        self.start  // self 被移动，调用者失去所有权
    }
    
    // 借用内存区域
    fn borrow_region(&self) -> usize {
        self.start  // self 被借用，调用者保持所有权
    }
}

fn ownership_demo() {
    let region = MemoryRegion::new(0x1000, 4096);
    
    // 借用使用
    let start = region.borrow_region();  // ✅ region 仍然可用
    
    // 移动使用
    let start2 = region.take_ownership();  // ✅ region 被移动
    // let start3 = region.borrow_region();  // ❌ 编译错误：region 已被移动
}
```

### 3. 实践：安全的内存操作

创建 `src/memory_safe.rs`：
```rust
#![no_std]

// 安全的内存操作封装
pub struct SafePtr<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> SafePtr<T> {
    // 创建安全指针（不安全操作被封装）
    pub unsafe fn new(ptr: *mut T, len: usize) -> Self {
        Self { ptr, len }
    }
    
    // 安全的读取操作
    pub fn read(&self, index: usize) -> Option<T> 
    where 
        T: Copy 
    {
        if index < self.len {
            unsafe { Some(self.ptr.add(index).read()) }
        } else {
            None  // 越界访问返回 None 而不是崩溃
        }
    }
    
    // 安全的写入操作
    pub fn write(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index < self.len {
            unsafe { 
                self.ptr.add(index).write(value);
                Ok(())
            }
        } else {
            Err("索引越界")
        }
    }
}

// 使用示例
fn safe_memory_demo() {
    let mut buffer = [0u8; 100];
    let mut safe_ptr = unsafe { 
        SafePtr::new(buffer.as_mut_ptr(), buffer.len()) 
    };
    
    // 安全操作
    match safe_ptr.write(10, 42) {
        Ok(()) => {
            if let Some(value) = safe_ptr.read(10) {
                // 成功读取值
            }
        }
        Err(e) => {
            // 处理错误
        }
    }
}
```

---

## 第四部分：错误处理机制 (10分钟)

### 1. 系统级错误处理

```rust
// 定义系统错误类型
#[derive(Debug, Clone, Copy)]
pub enum KernelError {
    OutOfMemory,
    InvalidAddress,
    PermissionDenied,
    DeviceNotFound,
}

// 结果类型别名
pub type KernelResult<T> = Result<T, KernelError>;

// 错误处理示例
fn allocate_memory(size: usize) -> KernelResult<*mut u8> {
    if size == 0 {
        return Err(KernelError::InvalidAddress);
    }
    
    if size > available_memory() {
        return Err(KernelError::OutOfMemory);
    }
    
    // 模拟内存分配
    Ok(0x1000 as *mut u8)
}

fn available_memory() -> usize {
    // 模拟可用内存查询
    1024 * 1024  // 1MB
}

// 使用错误处理
fn memory_management_demo() -> KernelResult<()> {
    let ptr = allocate_memory(4096)?;  // ? 操作符传播错误
    
    // 使用分配的内存
    unsafe {
        ptr.write(42);
    }
    
    Ok(())
}
```

### 2. Panic 处理策略

```rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 1. 禁用中断（防止递归 panic）
    disable_interrupts();
    
    // 2. 记录 panic 信息
    if let Some(location) = info.location() {
        print_emergency(&format!(
            "KERNEL PANIC at {}:{}\n", 
            location.file(), 
            location.line()
        ));
    }
    
    if let Some(message) = info.message() {
        print_emergency(&format!("Message: {}\n", message));
    }
    
    // 3. 尝试保存系统状态
    save_system_state();
    
    // 4. 停机或重启
    halt_system();
}

fn disable_interrupts() {
    // 禁用 CPU 中断
    unsafe {
        asm!("cli");  // x86_64 禁用中断指令
    }
}

fn print_emergency(msg: &str) {
    // 直接写入 VGA 缓冲区，不依赖其他系统
}

fn save_system_state() {
    // 保存关键系统信息到稳定存储
}

fn halt_system() -> ! {
    loop {
        unsafe {
            asm!("hlt");  // 停机指令
        }
    }
}
```

---

## 第五部分：实践项目 (15分钟)

### 项目：创建一个安全的内核模块

创建 `src/kernel_module.rs`：
```rust
#![no_std]

use core::panic::PanicInfo;

// 内核模块结构
pub struct KernelModule {
    name: &'static str,
    initialized: bool,
}

impl KernelModule {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Err("模块已经初始化");
        }
        
        // 模拟初始化过程
        self.initialized = true;
        Ok(())
    }
    
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    pub fn name(&self) -> &'static str {
        self.name
    }
}

// 全局模块管理器
static mut MODULES: [Option<KernelModule>; 10] = [None; 10];
static mut MODULE_COUNT: usize = 0;

pub fn register_module(module: KernelModule) -> Result<(), &'static str> {
    unsafe {
        if MODULE_COUNT >= MODULES.len() {
            return Err("模块数量超限");
        }
        
        MODULES[MODULE_COUNT] = Some(module);
        MODULE_COUNT += 1;
        Ok(())
    }
}

pub fn initialize_all_modules() -> Result<(), &'static str> {
    unsafe {
        for i in 0..MODULE_COUNT {
            if let Some(ref mut module) = MODULES[i] {
                module.initialize()?;
            }
        }
    }
    Ok(())
}

// 使用示例
fn kernel_module_demo() {
    let vga_module = KernelModule::new("VGA_DRIVER");
    let keyboard_module = KernelModule::new("KEYBOARD_DRIVER");
    
    // 注册模块
    register_module(vga_module).expect("注册 VGA 模块失败");
    register_module(keyboard_module).expect("注册键盘模块失败");
    
    // 初始化所有模块
    initialize_all_modules().expect("模块初始化失败");
}
```

### 测试代码

在 `src/main.rs` 中集成：
```rust
#![no_std]
#![no_main]

mod kernel_module;

use core::panic::PanicInfo;
use kernel_module::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 初始化内核模块
    kernel_module_demo();
    
    // 主循环
    loop {
        // 内核主要工作
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
```

---

## 第六部分：编译和测试 (10分钟)

### 1. 编译项目
```bash
# 清理之前的构建
cargo clean

# 编译项目
cargo build --target x86_64-blog_os.json

# 如果遇到错误，检查语法
cargo check
```

### 2. 运行测试
```bash
# 创建启动镜像
cargo bootimage

# 运行内核
cargo run
```

### 3. 调试技巧
```bash
# 使用详细输出查看编译过程
cargo build --target x86_64-blog_os.json --verbose

# 检查生成的汇编代码
cargo rustc --target x86_64-blog_os.json -- --emit asm
```

---

## 学习检查清单 ✅

完成第二天学习后，您应该能够：

- [ ] 理解 `#![no_std]` 和 `#![no_main]` 的作用
- [ ] 知道哪些 Rust 功能在系统编程中可用/不可用
- [ ] 理解所有权系统在系统编程中的优势
- [ ] 实现基本的错误处理机制
- [ ] 创建简单的内核模块系统
- [ ] 编写安全的内存操作代码

---

## 今日总结 📝

### 关键概念
1. **裸机编程**: 不依赖操作系统的程序开发
2. **内存安全**: Rust 如何在系统级别保证内存安全
3. **错误处理**: 系统编程中的错误处理策略
4. **模块化设计**: 内核模块的组织方式

### 重要技能
- 使用 `core` 库而不是 `std` 库
- 安全地封装不安全操作
- 设计系统级错误类型
- 实现模块化的内核架构

### 下一步预告
明天我们将学习：
- VGA 文本缓冲区的详细实现
- 内存映射 I/O 操作
- 颜色和格式化输出
- 创建 `print!` 和 `println!` 宏

---

## 额外练习 🚀

1. **扩展错误类型**: 为内核添加更多错误类型
2. **改进模块系统**: 添加模块依赖关系管理
3. **实验内存操作**: 尝试不同的内存安全封装方式
4. **阅读源码**: 查看 `core` 库的实现

恭喜您完成了第二天的学习！您现在对 Rust 系统编程有了深入的理解。明天我们将开始实现具体的硬件交互功能。