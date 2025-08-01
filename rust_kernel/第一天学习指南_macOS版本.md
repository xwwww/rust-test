# 第一天学习指南：macOS 开发环境准备

## macOS 特别说明 🍎
本指南专门针对 macOS 系统优化，包含了 macOS 特有的安装方法、路径配置和常见问题解决方案。

## 学习目标 🎯
- 在 macOS 上搭建完整的 Rust 操作系统开发环境
- 配置 macOS 特有的开发工具
- 成功编译并运行第一个内核程序

## 时间分配 ⏰
- **环境安装** (25分钟)
- **项目设置** (20分钟) 
- **验证测试** (15分钟)

---

## 第一部分：macOS 系统准备 (10分钟)

### 1. 安装 Xcode Command Line Tools
```bash
# 安装 Xcode 命令行工具（包含 GCC、Make 等）
xcode-select --install
```

### 2. 安装 Homebrew（如果还没有）
```bash
# 安装 Homebrew 包管理器
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 验证安装
brew --version
```

### 3. 更新系统工具
```bash
# 更新 Homebrew
brew update

# 安装或更新 Git（如果需要）
brew install git
```

---

## 第二部分：安装 Rust 工具链 (10分钟)

### 1. 安装 Rust
```bash
# 下载并安装 Rust（macOS 优化版本）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 选择默认安装选项（输入 1）
# 重新加载环境变量
source ~/.cargo/env

# 验证安装
rustc --version
cargo --version
```

### 2. 配置 Rust 环境
```bash
# 添加到 shell 配置文件
echo 'source ~/.cargo/env' >> ~/.zshrc  # 如果使用 zsh
# 或者
echo 'source ~/.cargo/env' >> ~/.bash_profile  # 如果使用 bash

# 重新加载配置
source ~/.zshrc  # 或 source ~/.bash_profile
```

### 3. 安装必要组件
```bash
# 安装 Rust 源码
rustup component add rust-src

# 安装 LLVM 工具
rustup component add llvm-tools-preview

# 安装 bootimage 工具
cargo install bootimage

# 验证安装
bootimage --version
```

---

## 第三部分：安装 QEMU (macOS 优化) (10分钟)

### 1. 使用 Homebrew 安装 QEMU
```bash
# 安装 QEMU（推荐方法）
brew install qemu

# 验证安装
qemu-system-x86_64 --version
which qemu-system-x86_64
```

### 2. 配置 QEMU 路径
```bash
# 检查 QEMU 安装路径
ls -la /opt/homebrew/bin/qemu*  # Apple Silicon Mac
# 或
ls -la /usr/local/bin/qemu*     # Intel Mac

# 添加到 PATH（通常 Homebrew 会自动处理）
echo 'export PATH="/opt/homebrew/bin:$PATH"' >> ~/.zshrc  # Apple Silicon
# 或
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc     # Intel Mac
```

### 3. 测试 QEMU
```bash
# 测试 QEMU 是否正常工作
qemu-system-x86_64 -version
```

---

## 第四部分：克隆和设置项目 (15分钟)

### 1. 创建工作目录
```bash
# 创建专门的开发目录
mkdir -p ~/Development/rust-os
cd ~/Development/rust-os
```

### 2. 克隆 Blog OS 项目
```bash
# 克隆官方仓库
git clone https://github.com/phil-opp/blog_os.git
cd blog_os

# 查看项目结构
ls -la
tree .  # 如果安装了 tree: brew install tree
```

### 3. macOS 特定配置
创建 `.cargo/config.toml`（如果不存在）：
```toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]

[build]
target = "x86_64-blog_os.json"

[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

---

## 第五部分：编译和运行 (15分钟)

### 1. 编译内核
```bash
# 清理之前的构建（如果有）
cargo clean

# 编译项目
cargo build

# 如果遇到链接错误，尝试：
cargo build --target x86_64-blog_os.json
```

### 2. 创建启动镜像
```bash
# 创建可启动的磁盘镜像
cargo bootimage

# 检查生成的镜像
ls -la target/x86_64-blog_os/debug/
```

### 3. 运行内核
```bash
# 在 QEMU 中运行
cargo run

# 或者手动运行（macOS 优化参数）
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin \
    -display cocoa \
    -accel hvf  # 使用 macOS 硬件加速
```

### 4. macOS 特定的 QEMU 参数
```bash
# 创建运行脚本
cat > run_os.sh << 'EOF'
#!/bin/bash
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin \
    -display cocoa \
    -accel hvf \
    -m 128M \
    -no-reboot \
    -no-shutdown
EOF

chmod +x run_os.sh
./run_os.sh
```

---

## 第六部分：IDE 配置 (macOS 优化) (10分钟)

### 1. VS Code 安装和配置
```bash
# 使用 Homebrew 安装 VS Code
brew install --cask visual-studio-code

# 或者从官网下载 macOS 版本
```

### 2. 安装必要扩展
在 VS Code 中安装：
- **rust-analyzer**: Rust 语言支持
- **CodeLLDB**: 调试支持（macOS 优化版）
- **Better TOML**: TOML 文件支持

### 3. 创建 macOS 特定的 VS Code 配置
创建 `.vscode/settings.json`：
```json
{
    "rust-analyzer.cargo.target": "x86_64-blog_os.json",
    "rust-analyzer.checkOnSave.allTargets": false,
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true,
    "terminal.integrated.shell.osx": "/bin/zsh",
    "rust-analyzer.server.path": "/Users/$(whoami)/.cargo/bin/rust-analyzer"
}
```

创建 `.vscode/launch.json`（调试配置）：
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Blog OS",
            "cargo": {
                "args": ["build", "--target=x86_64-blog_os.json"],
                "filter": {
                    "name": "blog_os",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

---

## macOS 特有问题解决 🔧

### 1. Apple Silicon (M1/M2) 特殊处理
```bash
# 检查芯片类型
uname -m

# 如果是 arm64（Apple Silicon），需要额外配置
if [[ $(uname -m) == "arm64" ]]; then
    echo "检测到 Apple Silicon Mac"
    
    # 确保使用正确的 Homebrew 路径
    echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
    source ~/.zshrc
    
    # 安装 Rosetta 2（如果需要运行 x86 程序）
    softwareupdate --install-rosetta --agree-to-license
fi
```

### 2. 权限问题解决
```bash
# 如果遇到权限问题
sudo xattr -rd com.apple.quarantine /opt/homebrew/bin/qemu-system-x86_64

# 或者对整个 QEMU 目录
sudo xattr -rd com.apple.quarantine /opt/homebrew/bin/qemu*
```

### 3. 网络问题（中国大陆用户）
```bash
# 配置 Rust 国内镜像
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[net]
git-fetch-with-cli = true
EOF
```

### 4. macOS Gatekeeper 问题
```bash
# 如果 macOS 阻止运行 QEMU
sudo spctl --master-disable  # 临时禁用 Gatekeeper
# 运行完成后重新启用
sudo spctl --master-enable
```

---

## macOS 性能优化 🚀

### 1. 启用硬件加速
```bash
# 检查是否支持 HVF（Hypervisor Framework）
sysctl kern.hv_support

# 在运行 QEMU 时使用 HVF 加速
qemu-system-x86_64 -accel hvf ...
```

### 2. 内存和 CPU 优化
```bash
# 创建优化的运行脚本
cat > run_os_optimized.sh << 'EOF'
#!/bin/bash
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin \
    -display cocoa \
    -accel hvf \
    -m 256M \
    -smp 2 \
    -no-reboot \
    -no-shutdown \
    -serial stdio
EOF

chmod +x run_os_optimized.sh
```

---

## 验证和测试 ✅

### 1. 系统检查清单
```bash
# 运行完整的环境检查
echo "=== macOS 环境检查 ==="
echo "系统版本: $(sw_vers -productVersion)"
echo "芯片类型: $(uname -m)"
echo "Rust 版本: $(rustc --version)"
echo "Cargo 版本: $(cargo --version)"
echo "QEMU 版本: $(qemu-system-x86_64 --version | head -1)"
echo "Bootimage 版本: $(bootimage --version)"
echo "Xcode 工具: $(xcode-select -p)"
```

### 2. 运行测试
```bash
# 运行单元测试
cargo test

# 运行集成测试
cargo test --test basic_boot

# 测试 QEMU 启动
timeout 10s cargo run || echo "QEMU 测试完成"
```

---

## macOS 开发技巧 💡

### 1. 使用 iTerm2 增强终端体验
```bash
# 安装 iTerm2
brew install --cask iterm2

# 配置 Oh My Zsh（可选）
sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
```

### 2. 使用 tmux 管理多个会话
```bash
# 安装 tmux
brew install tmux

# 创建开发会话
tmux new-session -d -s rust-os
tmux send-keys -t rust-os 'cd ~/Development/rust-os/blog_os' C-m
```

### 3. 设置快捷命令
```bash
# 添加到 ~/.zshrc
echo 'alias rust-os="cd ~/Development/rust-os/blog_os"' >> ~/.zshrc
echo 'alias qemu-run="cargo run"' >> ~/.zshrc
echo 'alias qemu-test="cargo test"' >> ~/.zshrc
source ~/.zshrc
```

---

## 学习检查清单 ✅

完成第一天学习后，您应该能够：

- [ ] 在 macOS 上成功安装 Rust 工具链
- [ ] 配置 QEMU 并启用硬件加速
- [ ] 克隆并编译 Blog OS 项目
- [ ] 在 QEMU 中运行内核（看到输出）
- [ ] 配置 VS Code 开发环境
- [ ] 理解 macOS 特有的配置和优化
- [ ] 解决常见的 macOS 开发问题

---

## 下一步建议 🎯

1. **熟悉 macOS 开发工作流** - 掌握终端、VS Code 和 QEMU 的配合使用
2. **优化开发环境** - 根据个人喜好调整 IDE 和终端配置
3. **准备学习第二天内容** - Rust 系统编程基础

恭喜您完成了 macOS 环境的搭建！您现在拥有了一个专门为 macOS 优化的 Rust 操作系统开发环境。