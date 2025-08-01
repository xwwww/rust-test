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