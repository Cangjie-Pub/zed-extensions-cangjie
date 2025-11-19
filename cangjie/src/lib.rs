// --- 文件顶部 ---
use std::collections::HashMap;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result}; // 重新引入 HashMap

// --- 配置区 ---
const LANGUAGE_SERVER_ID: &str = "cangjie-lsp";

// --- 结构体定义 ---
struct CangjieExtension;

impl CangjieExtension {
    fn new() -> Self {
        Self
    }
}

// --- CangjieExtension 的方法实现 ---
impl zed::Extension for CangjieExtension {
    fn new() -> Self {
        Self::new()
    }

    /// 返回启动语言服务器的命令。
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // 验证 Language Server ID
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Err(format!(
                "Unsupported language server ID: {}",
                language_server_id.as_ref()
            )
            .into());
        }

        // --- 从用户配置中读取路径 ---
        let config =
            LspSettings::for_worktree(language_server_id.as_ref(), _worktree).map_err(|err| {
                format!(
                    "Failed to read Cangjie LSP settings. Did you configure the paths? Error: {}",
                    err
                )
            })?;

        let settings = config
            .settings
            .as_ref()
            .ok_or("No settings found for Cangjie LSP. Please configure the paths.")?;

        // 获取 LSP Server 的可执行文件路径
        let server_path = settings.get("serverPath")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'serverPath' in Cangjie LSP settings. Please specify the path to the LSPServer executable.")?;

        // --- 关键修改：获取 CANGJIE_HOME 路径 ---
        // 假设用户配置的是 LSPServer 的完整路径，我们需要推导出 SDK 的根目录 (CANGJIE_HOME)
        // 例如，如果 serverPath 是 `/path/to/cangjie-sdk/cangjie/tools/bin/LSPServer`
        // 那么 CANGJIE_HOME 应该是 `/path/to/cangjie-sdk/cangjie`
        // 我们可以通过截断 serverPath 来得到它。
        use std::path::Path;
        let server_path_obj = Path::new(server_path);
        // tools/bin/LSPServer -> tools/bin -> tools -> (parent) -> CANGJIE_HOME
        let cangjie_home_path_obj = server_path_obj
            .parent() // bin
            .and_then(|p| p.parent()) // tools
            .and_then(|p| p.parent()) // CANGJIE_HOME
            .ok_or(
                "Could not determine CANGJIE_HOME from serverPath. Is the serverPath correct?",
            )?;

        let cangjie_home_str = cangjie_home_path_obj
            .to_str()
            .ok_or("CANGJIE_HOME path contains invalid characters.")?
            .to_string();

        // --- 构建命令和环境变量 ---
        let mut env_vars = HashMap::new();
        // 设置 CANGJIE_HOME 环境变量，让 LSPServer 知道 SDK 根目录
        env_vars.insert("CANGJIE_HOME".to_string(), cangjie_home_str.to_string());

        // 注意：不再设置 DYLD_LIBRARY_PATH，除非后续发现确实还需要
        env_vars.insert(
            "DYLD_LIBRARY_PATH".to_string(),
            format!("{}/runtime/lib/darwin_aarch64_llvm", cangjie_home_str),
        );

        Ok(zed::Command {
            command: server_path.to_string(),
            args: vec![
                "src".to_string(),
                "--disableAutoImport".to_string(),
                "--enable-log=true".to_string(),
            ],
            // args: vec![],
            env: env_vars.into_iter().collect(),
        })
    }

    /// 提供工作区配置
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        match LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            Ok(settings) => Ok(settings.settings),
            Err(_) => Ok(None),
        }
    }
}

// --- 注册扩展 ---
zed::register_extension!(CangjieExtension);
