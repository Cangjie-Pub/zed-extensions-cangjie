// --- 文件顶部 ---
use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

// --- 配置区 ---
const LANGUAGE_SERVER_ID: &str = "cangjie-lsp";
const SDK_DOWNLOAD_URL: &str = "https://github.com/Cangjie-Pub/cangjie_sdk_release/releases/download/1.0.4/cangjie-sdk-mac-aarch64-1.0.4.tar.gz";
const SDK_DIR_NAME: &str = "cangjie-sdk";
const SDK_BIN_SUBPATH: &str = "tools/bin/LSPServer";
const SDK_LIB_SUBPATH: &str = "runtime/lib/darwin_aarch64_llvm";

// --- 结构体定义 ---
struct CangjieExtension;

impl CangjieExtension {
    fn new() -> Self {
        Self
    }

    /// 确保 SDK 存在。如果不存在，则下载并解压到扩展的全局工作目录。
    fn ensure_sdk_exists(&mut self, language_server_id: &LanguageServerId) -> Result<()> {
        // 使用相对路径，Zed 会将其解析为扩展的 work 目录下的路径
        let sdk_target_dir = SDK_DIR_NAME;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let download_result = zed::download_file(
            SDK_DOWNLOAD_URL,
            &sdk_target_dir,
            zed::DownloadedFileType::GzipTar,
        );

        match download_result {
            Ok(()) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::None,
                );
                Ok(())
            }
            Err(e) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::None,
                );
                Err(e)
            }
        }
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

        // 确保 SDK 已下载到扩展的全局工作目录
        self.ensure_sdk_exists(language_server_id)?;

        // 构建相对于扩展 work 目录的命令和环境变量
        let relative_server_path = format!("./{}/{}/{}", SDK_DIR_NAME, "cangjie", SDK_BIN_SUBPATH);
        let relative_lib_path = format!("./{}/{}/{}", SDK_DIR_NAME, "cangjie", SDK_LIB_SUBPATH);

        let mut env_vars = std::collections::HashMap::new();
        env_vars.insert(
            "CANGJIE_HOME".to_string(),
            format!("./{}/{}", "cangjie", SDK_DIR_NAME),
        );
        env_vars.insert("DYLD_LIBRARY_PATH".to_string(), relative_lib_path);

        Ok(zed::Command {
            command: relative_server_path,
            args: vec![],
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
