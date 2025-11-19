# Zed 编辑器的仓颉（Cangjie）扩展

此扩展为 Zed 编辑器添加了对仓颉编程语言的全面语言支持。

## 功能特性

-   语法高亮
-   代码补全
-   诊断信息（错误和警告报告）
-   转到定义 (Go to Definition)
-   查找所有引用 (Find All References)
-   悬停信息 (Hover Information)
-   文档符号大纲 (Document Symbols Outline)

## 环境要求

-   **Zed 编辑器**: 版本 0.135.0 或更高。
-   **Rust 工具链**: 需要从源代码构建扩展。
-   **仓颉语言服务器 (LSP)**: 需要包含 `LSPServer` 的预编译仓颉 SDK。

## 安装

### 方式一：通过 Zed 市场安装（推荐 - 未来可用）

*(注意：此扩展正在等待提交/审核。如果可用，从市场安装将是最快捷的方法。)*

### 方式二：手动安装（加载开发扩展）

1.  **克隆仓库**:
    将此仓库克隆到你计算机上的一个本地目录。
    ```bash
    git clone https://github.com/Cangjie-Pub/zed-extensions-cangjie.git # 或者是你 Fork 的地址
    cd zed-extensions-cangjie/cangjie # 根据实际情况调整路径
    ```
2.  **构建扩展**:
    使用 Cargo 以发布模式编译扩展。
    ```bash
    cargo build --release
    ```
3.  **在 Zed 中加载扩展**:
    -   打开 Zed。
    -   转到 `Zed > Settings` (或按 `Ctrl/Cmd + ,`)。
    -   导航到 `Extensions` (扩展) 选项卡。
    -   点击 **`Install Dev Extension`** (安装开发扩展) 按钮。
    -   在弹出的文件对话框中，导航到并选择此扩展的**根文件夹**（即包含 `Cargo.toml` 文件的文件夹）。例如，选择 `zed-extensions/cangjie` 目录。
    -   Zed 将自动从此目录检测并加载扩展。
    -   如果出现提示询问是否信任该扩展，请查看源代码并确认信任。
4.  **(可选) 启用扩展**:
    扩展加载后通常会默认启用。你可以在 `Extensions` (扩展) 设置列表中验证其状态。如有必要，请将其切换为开启状态。

*(注意：现在不再需要手动复制二进制文件或将文件夹放入 Zed 的扩展目录了。Zed 会直接从你的源代码文件夹加载。)*
## 配置仓颉语言服务器 (LSP)

此扩展需要仓颉语言服务器 (`LSPServer`) 才能正常工作。你需要手动安装仓颉 SDK，并告诉扩展在哪里可以找到 `LSPServer` 可执行文件。

### 步骤:

1.  **下载仓颉 SDK**:
    访问仓颉官方网站或其 GitHub 发布页面，下载适用于你操作系统的 SDK 包（例如 `cangjie-sdk-mac-aarch64-1.0.4.tar.gz`）。

2.  **解压 SDK**:
    将下载的压缩包解压到你选择的目录。例如：
    ```bash
    tar -xzf cangjie-sdk-mac-aarch64-1.0.4.tar.gz -C ~/
    ```
    这可能会创建一个像 `~/cangjie-sdk/cangjie/` 这样的目录。

3.  **配置 Zed 以定位 LSP 服务器**:
    你需要在 Zed 的设置中提供 `LSPServer` 可执行文件的绝对路径。
    -   打开 Zed 设置 (`Cmd/Ctrl + ,`)。
    -   切换到 `JSON` 视图。
    -   添加或修改 `cangjie-lsp` 的配置块。将 `/absolute/path/to/...` 替换为你系统上的实际路径。

    ```json
     {
       "lsp": {
         "cangjie-lsp": {
           "settings": {
             "serverPath": "/Users/yourusername/cangjie-sdk/cangjie/tools/bin/LSPServer" // <-- 更新此路径
           }
         }
       },
       "file_types": {
         "Cangjie": ["cj"]
       },
       "languages": {
         "Cangjie": {
           "language_servers": ["cangjie-lsp"]
         }
       }
     }
    ```

    *   **`lsp.cangjie-lsp.settings.serverPath`**: 将此项设置为你解压后的 SDK 中 `LSPServer` 可执行文件的完整绝对路径。

4.  **重启 Zed**:
    保存你的设置，并完全重启 Zed 编辑器以确保所有更改生效。

配置正确后，打开 `.cj` 文件就应该能激活此扩展提供的仓颉语言功能。

## 贡献

非常欢迎贡献！欢迎提交 Issue 或 Pull Request 来改进此扩展。

## 构建扩展 (面向开发者)

导航到扩展目录并运行标准的 Cargo 构建命令：

```bash
cd /path/to/zed-extensions/cangjie # 或者是你克隆到的地方
cargo build --release
