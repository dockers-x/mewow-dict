# MDD资源文件支持说明

## 功能概述

现在程序已经支持解析和提供MDD（Mdict Data）资源文件。MDD文件是MDX词典的配套文件，包含图片、音频、CSS、JavaScript等媒体资源。

## 主要特性

✅ **自动加载**: 程序启动时自动加载所有MDD文件  
✅ **多种格式**: 支持图片、音频、视频、字体等20多种文件格式  
✅ **智能匹配**: 不区分大小写的路径匹配  
✅ **按需解压**: 只在请求时才解压资源，节省内存  
✅ **全局缓存**: 所有MDD文件缓存在内存中，快速访问  

## 使用方法

### 1. 放置MDD文件

将`.mdd`文件放在与`.mdx`文件相同的目录中：

```
resources/mdx/
├── 牛津高阶英汉双解词典.mdx
├── 牛津高阶英汉双解词典.mdd    ← MDD资源文件
├── 韦氏高阶英汉双解词典.mdx
└── 韦氏高阶英汉双解词典.mdd
```

### 2. 启动程序

```bash
cargo run --bin mewow-dict
```

程序启动时会显示：
```
Loading MDD resource files...
Loading MDD file: resources/mdx/牛津高阶英汉双解词典.mdd
Successfully loaded MDD: 牛津高阶英汉双解词典
Total MDD files loaded: 2
MDD resource files loaded
```

### 3. 访问资源

资源通过以下URL格式访问：
```
http://localhost:8181/mdd/{资源路径}
```

示例：
- 图片: `http://localhost:8181/mdd/images/example.png`
- 音频: `http://localhost:8181/mdd/audio/pronunciation.mp3`
- CSS: `http://localhost:8181/mdd/styles.css`

## 支持的资源类型

### 图片格式
- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- SVG (.svg)
- WebP (.webp)
- ICO (.ico)

### 音频格式
- MP3 (.mp3)
- WAV (.wav)
- OGG (.ogg)

### 视频格式
- MP4 (.mp4)
- WebM (.webm)

### 字体格式
- WOFF (.woff)
- WOFF2 (.woff2)
- TTF (.ttf)
- OTF (.otf)

### 网页资源
- CSS (.css)
- JavaScript (.js)
- HTML (.html, .htm)
- JSON (.json)
- XML (.xml)

## 技术实现

### 文件结构
```
src/
├── mdict/
│   ├── mdd.rs          # MDD文件解析器
│   ├── mdx.rs          # MDX文件解析器
│   └── ...
├── mdd_manager.rs      # MDD资源管理器
├── handlers/mod.rs     # HTTP请求处理（新增MDD处理器）
└── main.rs            # 主程序（初始化MDD管理器）
```

### 工作流程

1. **启动阶段**
   - 扫描配置的目录
   - 发现所有`.mdd`文件
   - 解析MDD文件头和索引
   - 将资源路径缓存到内存

2. **请求阶段**
   - 客户端请求 `/mdd/example.png`
   - MDD管理器查找资源
   - 定位并解压对应的数据块
   - 返回资源并设置正确的MIME类型

3. **内存优化**
   - 只缓存资源的路径和位置信息
   - 实际资源数据保持压缩状态
   - 按需解压，用完即弃

## 示例代码

如果你想在代码中直接使用MDD解析器：

```rust
use std::fs;
use mewow_dict::mdict::mdd::Mdd;

// 加载MDD文件
let data = fs::read("dictionary.mdd")?;
let mdd = Mdd::new(&data);

// 获取资源
if let Some(resource_data) = mdd.get_resource_by_path("images/example.png") {
    // 使用资源数据
    fs::write("output.png", resource_data)?;
}
```

完整示例见：`examples/mdd_usage.rs`

## 配置说明

MDD文件的加载目录与MDX文件相同，通过以下环境变量配置：

- `BUILTIN_DICT_DIR`: 内置词典目录
- `USER_DICT_DIR`: 用户词典目录

## 测试验证

运行测试脚本：
```bash
./test_mdd.sh
```

或手动测试：
```bash
# 1. 编译项目
cargo build --release

# 2. 运行服务器
cargo run --bin mewow-dict

# 3. 在浏览器访问
# http://localhost:8181
```

## 故障排查

### MDD文件未加载
检查控制台输出，确认：
- MDD文件是否在正确的目录
- 文件权限是否正确
- 是否有解析错误信息

### 资源404错误
确认：
- 资源路径是否正确（不区分大小写）
- MDD文件是否成功加载
- 使用 `/mdd/` 前缀访问资源

### 性能问题
- 大型MDD文件会占用较多内存
- 首次访问资源时需要解压，可能稍慢
- 考虑将大文件拆分为多个小MDD文件

## 相关文件

- `CHANGELOG_MDD.md` - 详细的英文更改日志
- `README.md` - 项目总体说明
- `examples/mdd_usage.rs` - 使用示例
- `test_mdd.sh` - 测试脚本

## 兼容性

- 支持 MDX/MDD v2.0 格式
- 支持加密（encrypted=2）和非加密（encrypted=0）文件
- 支持多种压缩格式（zlib, lzo）

## 未来改进

可能的增强功能：
- LRU缓存策略优化常用资源
- 支持响应压缩（gzip, brotli）
- 支持Range请求，适合大文件
- 资源预加载提升性能
