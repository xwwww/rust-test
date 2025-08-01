use core::panic::PanicInfo;

// 内核模块结构
#[derive(Copy, Clone)]
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
