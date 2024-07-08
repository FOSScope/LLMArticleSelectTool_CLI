# LLMArticleSelectTool_CLI

本项目用于从网页中提取和过滤 HTML 内容，并通过 OpenAI-API 调用DeepSeek-V2-Chat模型生成相应的 Markdown 文本输出。
输出文件保存在工作目录下的 output 目录中。

## 目录

- [LLMArticleSelectTool_CLI](#LLMArticleSelectTool_CLI)
  - [目录](#目录)
  - [特性](#特性)
  - [先决条件](#先决条件)
  - [安装](#安装)
  - [使用方法](#使用方法)
  - [环境变量](#环境变量)
  - [配置文件](#配置文件)
  - [许可证](#许可证)

## 特性

- 从指定的 URL 获取 HTML 内容。
- 根据指定的标签和类名过滤 HTML 内容。
- 通过 OpenAI 的 API 生成相应的文本。
- 将生成的文本保存到本地文件系统中的 `output` 目录。

## 先决条件

在运行此项目之前，请确保你已经安装并配置了以下环境：

- [Rust](https://www.rust-lang.org/): 1.54.0 或更高版本
- [DeepSeek API Key](https://platform.deepseek.com/api_keys): 需要一个有效的 DeekSeek API 密钥

## 安装

克隆此项目到本地：

```bash
git clone https://github.com/FOSScope/LLMArticleSelectTool_CLI.git
cd LLMArticleSelectTool_CLI
```

使用 `cargo` 构建项目：

```bash
cargo build --release
```

## 使用方法

运行程序，指定 URL、标签和类名：

```bash
cargo run --release -- --url https://example.com --tag "tag1,tag2" --class "class1,class2"
```

## 环境变量

请在项目根目录下创建一个 `.env` 文件，并添加你的 DeepSeek API 密钥：

```
AUTH_TOKEN=sk_xxxxxxxxxx
```

## 配置文件

项目使用 `config.toml` 文件来设置模板和提示词片段。请确保在项目根目录下有一个 `config.toml` 文件。可直接使用项目自带的 `config.toml` 文件。

```toml
[general]
template = "Your template here"
prompt = "Your prompt fragment here"
```

## 许可证

该项目使用 MIT 许可证。有关更多信息，请参阅 [LICENSE](LICENSE) 文件。
