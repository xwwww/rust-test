// 不再需要这个导入

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
}

// 全局模块管理器
static mut MODULES: [Option<KernelModule>; 10] = [None; 10];
static mut MODULE_COUNT: usize = 0;

pub fn register_module(module: KernelModule) -> Result<(), &'static str> {
    unsafe {
        // 使用一个固定的最大模块数，避免访问MODULES.len()
        const MAX_MODULES: usize = 10;
        if MODULE_COUNT >= MAX_MODULES {
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

// 导出内核模块演示函数，供main.rs调用
#[no_mangle]
pub fn kernel_module_demo_export() {
    let vga_module = KernelModule::new("VGA_DRIVER");
    let keyboard_module = KernelModule::new("KEYBOARD_DRIVER");
    
    // 注册模块
    register_module(vga_module).expect("注册 VGA 模块失败");
    register_module(keyboard_module).expect("注册键盘模块失败");
    
    // 初始化所有模块
    initialize_all_modules().expect("模块初始化失败");
}
