use serde_json::Value;
use zed::serde_json;
use zed::LanguageServerId;
use zed_extension_api::Worktree;
use zed_extension_api::{self as zed, settings::LspSettings, Result};
// 导入 HashMap 以设置环境变量
use std::collections::HashMap;
use std::env;

// 由于只需要一个 LSP 服务器，我们不需要一个单独的 language_servers 模块，
// 而是直接在下方定义一个私有结构体来代表我们的 Cangjie LSP。

// 定义 Cangjie LSP 的处理器
struct CangjieLanguageServer;

impl CangjieLanguageServer {
    // 定义唯一的语言服务器 ID
    const LANGUAGE_SERVER_ID: &'static str = "cangjie-lsp";

    // 构造函数
    fn new() -> Self {
        CangjieLanguageServer {}
    }

    // 获取语言服务器的二进制路径
    // 在这个场景中，路径是固定的，不需要复杂的查找逻辑
    fn language_server_binary_path(&self, _: &LanguageServerId) -> zed::Result<String> {
        // 1. 尝试获取 CANGJIE_HOME 环境变量的值
        // 如果获取失败（例如环境变量未设置），则返回一个 Zed 错误
        let cangjie_home = env::var("CANGJIE_HOME")
                    .unwrap_or_else(|_| {
                        // ** 🌟 修正：如果环境变量未设置，使用您已知的默认安装路径**
                        eprintln!("Warning: CANGJIE_HOME not found in environment, defaulting to /Users/lsmiao/cangjie");
                        "/Users/lsmiao/cangjie".to_string() 
                    });

        // 2. 组合路径：${CANGJIE_HOME}/tools/bin/LSPServer
        let binary_path = format!("{}/tools/bin/LSPServer", cangjie_home);

        Ok(binary_path)
    }
}

struct CangjieExtension {
    // 只需要一个 LSP 实例
    cangjie_language_server: Option<CangjieLanguageServer>,
}

impl zed::Extension for CangjieExtension {
    fn new() -> Self {
        Self {
            cangjie_language_server: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            CangjieLanguageServer::LANGUAGE_SERVER_ID => {
                let cangjie_language_server = self
                    .cangjie_language_server
                    .get_or_insert_with(CangjieLanguageServer::new);

                // 🌟 1. 获取动态二进制路径 (调用已修改的函数)
                let binary_path =
                    cangjie_language_server.language_server_binary_path(language_server_id)?;

                // 🌟 2. 再次读取 CANGJIE_HOME 构造动态库路径
                // 1. 获取 CANGJIE_HOME 环境变量的值，如果失败，使用默认路径
                let cangjie_home = env::var("CANGJIE_HOME")
                    .unwrap_or_else(|_| {
                        // ** 🌟 修正：如果环境变量未设置，使用您已知的默认安装路径**
                        eprintln!("Warning: CANGJIE_HOME not found in environment, defaulting to /Users/lsmiao/cangjie");
                        "/Users/lsmiao/cangjie".to_string() 
                    });

                // 使用 CANGJIE_HOME 构造正确的动态库路径
                let lib_path = format!("{}/runtime/lib/darwin_aarch64_llvm", cangjie_home);

                let mut env_map = HashMap::new();

                // 🌟 3. 设置必要的环境变量，修复之前的动态库加载错误
                env_map.insert("CANGJIE_HOME".to_string(), cangjie_home.to_string());
                env_map.insert("DYLD_LIBRARY_PATH".to_string(), lib_path.to_string());
                // LD_LIBRARY_PATH 保持兼容性
                env_map.insert("LD_LIBRARY_PATH".to_string(), lib_path.to_string());

                let env: Vec<(String, String)> = env_map.into_iter().collect();

                Ok(zed::Command {
                    command: binary_path,
                    args: vec![
                        "src".to_string(),
                        "--disableAutoImport".to_string(),
                        "--enable-log=true".to_string(),
                    ],
                    env, // 传递正确的环境变量
                })
            }
            _ => Err(format!(
                "Unrecognized language server for Cangjie: {language_server_id}"
            )),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        if let Ok(Some(settings)) = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings)
        {
            Ok(Some(settings))
        } else {
            self.language_server_initialization_options(language_server_id, worktree)
                .map(|init_options| {
                    init_options.and_then(|init_options| init_options.get("settings").cloned())
                })
        }
    }

    // fn language_server_workspace_configuration(
    //     &mut self,
    //     language_server_id: &LanguageServerId,
    //     worktree: &zed_extension_api::Worktree,
    // ) -> Result<Option<serde_json::Value>> {
    //     let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
    //         .ok()
    //         .and_then(|lsp_settings| lsp_settings.settings.clone())
    //         .unwrap_or_default();

    //     // 将用户设置嵌套在 "cangjie" 键下，符合 LSP 配置惯例
    //     Ok(Some(serde_json::json!({
    //         "cangjie": settings
    //     })))
    // }
}

// 注册扩展
zed::register_extension!(CangjieExtension);
