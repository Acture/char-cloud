# Char Cloud

[![Crates.io](https://img.shields.io/crates/v/char-cloud)](https://crates.io/crates/char-cloud)
[![Release Build](https://github.com/Acture/char-cloud/actions/workflows/release.yml/badge.svg)](https://github.com/Acture/char-cloud/actions/workflows/release.yml)
[![License](https://img.shields.io/crates/l/char-cloud)](LICENSE)

Char Cloud 是一个用 Rust 编写的命令行工具，可以生成自定义形状的文字云图片。它允许您使用文本作为轮廓，并用指定的词语填充形状，生成 SVG 格式的输出文件。

## 特性

- 支持自定义形状文本
- 自动适应字体大小
- 自定义输出图像大小
- 支持自定义字体
- 多种颜色支持
- SVG 格式输出
- 内置中文字体支持

## 安装
```bash
cargo install char-cloud
```


## 使用方法

基本用法：
```bash
char-cloud --text "目标文本" --words "词1,词2,词3" --output output.svg
```

## 许可
本项目采用 Affero GPL v3 许可协议。请在使用前仔细阅读 [LICENSE](LICENSE) 文件。
