<img width="1370" height="84" alt="image" src="https://github.com/user-attachments/assets/63fa4b0b-f9ab-4578-a5fb-5ae78288c093" /># fuckHttp 浏览器选择器

一个Windows应用程序，用于拦截URL重定向并允许用户选择使用哪个浏览器打开链接。

故事背景：https://linux.do/t/topic/914393

## 功能特性

- 拦截来自QQ、微信、企业微信安全页面的URL
- 从重定向页面提取真实URL
- 提供图形界面进行浏览器选择
- 支持自定义浏览器配置
- 微信链接异步提取处理
- 系统集成作为默认浏览器处理程序

## 安装说明

1. 从发布页面下载最新版本
2. 以管理员身份运行可执行文件以注册为默认浏览器
3. 在设置中选择"注册为默认浏览器"

## 使用方法

### 命令行使用
```
fuckHttp.exe "https://example.com"
```

### 支持的URL模式

- QQ电脑版: `https://c.pc.qq.com/ios.html?level=14&url=*`
- 微信: `https://weixin110.qq.com/security/readtemplate?*`
- 企业微信: `https://open.work.weixin.qq.com/wwopen/mpnews?*`

## 从源码构建

### 前置要求
- Rust 1.70 或更高版本
- Windows SDK（用于Windows构建）

### 构建命令

```powershell
# 调试构建
cargo build

# 发布构建
cargo build --release

# 清理构建
cargo clean
```

### 依赖项

- eframe: GUI框架
- regex: URL模式匹配
- reqwest: URL提取的HTTP请求
- winreg: Windows注册表访问
- serde: 配置序列化

## 配置

应用程序将配置存储在:
```
%APPDATA%\fuckHttp\config.json
```

配置包括:
- 隐藏的浏览器列表
- 自定义浏览器命令

## 系统集成

应用程序可以注册为:
- HTTP/HTTPS协议的默认浏览器
- 特定URL方案的处理程序

使用设置面板管理系统集成。
