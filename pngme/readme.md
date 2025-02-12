### PNGME：PNG 文件处理工具

为了学习 rust，我写了一个简单的 PNG 文件处理工具，它允许你在 PNG 文件中编码隐藏信息、解码提取信息、删除指定数据块以及打印所有数据块信息。

#### 功能特性
* 编码（Encode）：将指定的消息嵌入到 PNG 文件的数据块中。
* 解码（Decode）：从 PNG 文件中提取指定类型数据块里的消息。
* 删除（Remove）：从 PNG 文件中移除指定类型的第一个数据块。
* 打印（Print）：输出 PNG 文件中所有数据块的信息。

#### 使用步骤

##### 构建项目
```bash
cargo build --release
```

##### 将可执行文件添加到系统路径（可选）
```bash
export PATH=$PATH:/path/to/your/project/target/release
```

##### 使用方法

```bash
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [--output <OUTPUT_FILE>]
```
<FILE_PATH>：输入的 PNG 文件路径。 <br>
<CHUNK_TYPE>：自定义的数据块类型，必须是 4 个字符长且由 ASCII 字母组成。<br>
MESSAGE：要编码的消息。<br>
--output <OUTPUT_FILE>：可选参数，指定输出文件路径。若不指定，默认覆盖输入文件。

```bash
pngme decode <FILE_PATH> <CHUNK_TYPE>
```
<FILE_PATH>：输入的 PNG 文件路径。<br>
<CHUNK_TYPE>：要解码的数据块类型。

```bash
pngme remove <FILE_PATH> <CHUNK_TYPE>
```
<FILE_PATH>：输入的 PNG 文件路径。<br>
<CHUNK_TYPE>：要删除的数据块类型。<br>

```bash
pngme print <FILE_PATH>
```
<FILE_PATH>：输入的 PNG 文件路径。<br>

#### 示例

```bash
pngme encode input.png "HIDE" "Hello, World!" --output output.png
pngme decode output.png "HIDE"
pngme remove output.png "HIDE"