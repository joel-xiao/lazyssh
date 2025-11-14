use std::env;

pub struct I18n {
    pub is_chinese: bool,
}

impl I18n {
    pub fn new() -> Self {
        Self::with_lang(None)
    }

    pub fn with_lang(lang_override: Option<&str>) -> Self {
        let lang = if let Some(lang) = lang_override {
            lang.to_string()
        } else {
            env::var("LAZYSSH_LANG")
                .or_else(|_| env::var("LANG"))
                .or_else(|_| env::var("LC_ALL"))
                .or_else(|_| env::var("LC_MESSAGES"))
                .unwrap_or_else(|_| String::from("en"))
        };
        
        let lang_lower = lang.to_lowercase();
        let is_chinese = lang_lower.starts_with("zh") 
            || lang_lower == "cn"
            || lang_lower.contains("cn") 
            || lang_lower.contains("tw")
            || lang_lower.contains("hans")
            || lang_lower.contains("hant");
        
        Self { is_chinese }
    }

    pub fn unknown_arg(&self, arg: &str) -> String {
        if self.is_chinese {
            format!("未知参数: {}", arg)
        } else {
            format!("Unknown argument: {}", arg)
        }
    }

    pub fn use_help(&self) -> &str {
        if self.is_chinese {
            "使用 --help 查看帮助信息"
        } else {
            "Use --help for help information"
        }
    }

    pub fn help_title(&self) -> &str {
        "LazySSH - A cross-platform SSH management tool with TUI interface"
    }

    pub fn help_usage(&self) -> &str {
        if self.is_chinese {
            "用法: lazyssh [选项]"
        } else {
            "Usage: lazyssh [OPTIONS]"
        }
    }

    pub fn help_options(&self) -> &str {
        if self.is_chinese {
            "选项:"
        } else {
            "Options:"
        }
    }

    pub fn help_version(&self) -> &str {
        if self.is_chinese {
            "显示版本信息"
        } else {
            "Show version information"
        }
    }

    pub fn help_help(&self) -> &str {
        if self.is_chinese {
            "显示帮助信息"
        } else {
            "Show help information"
        }
    }

    pub fn help_lang(&self) -> &str {
        if self.is_chinese {
            "指定语言 (zh/en)"
        } else {
            "Specify language (zh/en)"
        }
    }

    pub fn help_no_args(&self) -> &str {
        if self.is_chinese {
            "如果没有指定选项，将启动图形化 TUI 界面。"
        } else {
            "If no options are specified, the graphical TUI interface will be launched."
        }
    }

    pub fn sshpass_not_found(&self) -> &str {
        if self.is_chinese {
            "sshpass 未找到。请手动安装 sshpass（macOS 或 Linux）。"
        } else {
            "sshpass not found. Please install it manually for macOS or Linux."
        }
    }

    pub fn sshpass_cannot_login(&self) -> &str {
        if self.is_chinese {
            "sshpass 未找到。无法自动登录。"
        } else {
            "sshpass not found. Cannot auto-login."
        }
    }

    pub fn ssh_connection_failed(&self, userhost: &str) -> String {
        if self.is_chinese {
            format!("\n❌ SSH 连接失败: {}", userhost)
        } else {
            format!("\n❌ SSH connection failed: {}", userhost)
        }
    }

    pub fn exit_code(&self) -> &str {
        if self.is_chinese {
            "   退出代码:"
        } else {
            "   Exit code:"
        }
    }

    pub fn possible_reasons(&self) -> &str {
        if self.is_chinese {
            "   可能的原因：网络问题、主机不可达、认证失败等"
        } else {
            "   Possible reasons: network issues, host unreachable, authentication failure, etc."
        }
    }

    pub fn press_enter_to_return(&self) -> &str {
        if self.is_chinese {
            "\n按回车键返回..."
        } else {
            "\nPress Enter to return..."
        }
    }

    pub fn wait_ssh_process_error(&self, e: &str) -> String {
        if self.is_chinese {
            format!("\n❌ 等待 SSH 进程时出错: {}", e)
        } else {
            format!("\n❌ Error waiting for SSH process: {}", e)
        }
    }

    pub fn execute_ssh_error(&self, e: &str) -> String {
        if self.is_chinese {
            format!("\n❌ 无法执行 SSH 命令: {}", e)
        } else {
            format!("\n❌ Failed to execute SSH command: {}", e)
        }
    }

    pub fn invalid_host_format(&self) -> &str {
        if self.is_chinese {
            "错误: 主机格式不正确"
        } else {
            "Error: Invalid host format"
        }
    }

    pub fn confirm_delete_host(&self, host_name: &str) -> String {
        if self.is_chinese {
            format!("│  确认删除主机: {:30} │", host_name)
        } else {
            format!("│  Confirm delete host: {:30} │", host_name)
        }
    }

    pub fn press_y_to_confirm(&self) -> &str {
        if self.is_chinese {
            "│  按 'y' 确认删除，按 'n' 取消           │"
        } else {
            "│  Press 'y' to confirm, 'n' to cancel     │"
        }
    }

    pub fn confirm_delete(&self) -> &str {
        if self.is_chinese {
            "  y: 确认删除  │  n/Esc: 取消"
        } else {
            "  y: Confirm Delete  │  n/Esc: Cancel"
        }
    }

    pub fn clipboard_parse_error(&self) -> &str {
        if self.is_chinese {
            "错误: 无法解析剪贴板内容为有效的 SSH 命令格式"
        } else {
            "Error: Failed to parse clipboard content as valid SSH command format"
        }
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

