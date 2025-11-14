use lazyssh::i18n::I18n;
use std::env;

#[test]
fn test_with_lang_zh() {
    let i18n = I18n::with_lang(Some("zh"));
    assert!(i18n.is_chinese);
}

#[test]
fn test_with_lang_zh_cn() {
    let i18n = I18n::with_lang(Some("zh_CN"));
    assert!(i18n.is_chinese);
}

#[test]
fn test_with_lang_zh_tw() {
    let i18n = I18n::with_lang(Some("zh_TW"));
    assert!(i18n.is_chinese);
}

#[test]
fn test_with_lang_cn() {
    let i18n = I18n::with_lang(Some("cn"));
    assert!(i18n.is_chinese);
}

#[test]
fn test_with_lang_en() {
    let i18n = I18n::with_lang(Some("en"));
    assert!(!i18n.is_chinese);
}

#[test]
fn test_with_lang_en_us() {
    let i18n = I18n::with_lang(Some("en_US"));
    assert!(!i18n.is_chinese);
}

#[test]
fn test_with_lang_none_uses_env() {
    env::set_var("LAZYSSH_LANG", "zh");
    let i18n = I18n::with_lang(None);
    assert!(i18n.is_chinese);
    env::remove_var("LAZYSSH_LANG");
}

#[test]
fn test_unknown_arg_chinese() {
    let i18n = I18n::with_lang(Some("zh"));
    let msg = i18n.unknown_arg("test");
    assert!(msg.contains("未知参数"));
}

#[test]
fn test_unknown_arg_english() {
    let i18n = I18n::with_lang(Some("en"));
    let msg = i18n.unknown_arg("test");
    assert!(msg.contains("Unknown argument"));
}

#[test]
fn test_help_texts_chinese() {
    let i18n = I18n::with_lang(Some("zh"));
    assert_eq!(i18n.help_usage(), "用法: lazyssh [选项]");
    assert_eq!(i18n.help_options(), "选项:");
    assert_eq!(i18n.help_version(), "显示版本信息");
    assert_eq!(i18n.help_help(), "显示帮助信息");
    assert_eq!(i18n.help_lang(), "指定语言 (zh/en)");
}

#[test]
fn test_help_texts_english() {
    let i18n = I18n::with_lang(Some("en"));
    assert_eq!(i18n.help_usage(), "Usage: lazyssh [OPTIONS]");
    assert_eq!(i18n.help_options(), "Options:");
    assert_eq!(i18n.help_version(), "Show version information");
    assert_eq!(i18n.help_help(), "Show help information");
    assert_eq!(i18n.help_lang(), "Specify language (zh/en)");
}

#[test]
fn test_ssh_messages_chinese() {
    let i18n = I18n::with_lang(Some("zh"));
    let msg = i18n.ssh_connection_failed("user@host");
    assert!(msg.contains("SSH 连接失败"));
    assert!(msg.contains("user@host"));
    
    assert_eq!(i18n.exit_code(), "   退出代码:");
    assert!(i18n.possible_reasons().contains("网络问题"));
    assert_eq!(i18n.press_enter_to_return(), "\n按回车键返回...");
}

#[test]
fn test_ssh_messages_english() {
    let i18n = I18n::with_lang(Some("en"));
    let msg = i18n.ssh_connection_failed("user@host");
    assert!(msg.contains("SSH connection failed"));
    assert!(msg.contains("user@host"));
    
    assert_eq!(i18n.exit_code(), "   Exit code:");
    assert!(i18n.possible_reasons().contains("network issues"));
    assert_eq!(i18n.press_enter_to_return(), "\nPress Enter to return...");
}

#[test]
fn test_ui_messages_chinese() {
    let i18n = I18n::with_lang(Some("zh"));
    assert_eq!(i18n.invalid_host_format(), "错误: 主机格式不正确");
    assert!(i18n.confirm_delete_host("test").contains("确认删除主机"));
    assert!(i18n.press_y_to_confirm().contains("确认删除"));
    assert!(i18n.confirm_delete().contains("确认删除"));
    assert!(i18n.clipboard_parse_error().contains("无法解析"));
}

#[test]
fn test_ui_messages_english() {
    let i18n = I18n::with_lang(Some("en"));
    assert_eq!(i18n.invalid_host_format(), "Error: Invalid host format");
    assert!(i18n.confirm_delete_host("test").contains("Confirm delete host"));
    assert!(i18n.press_y_to_confirm().contains("confirm"));
    assert!(i18n.confirm_delete().contains("Confirm Delete"));
    assert!(i18n.clipboard_parse_error().contains("Failed to parse"));
}

#[test]
fn test_chinese_variants() {
    let variants = vec!["zh", "zh_CN", "zh_TW", "zh-Hans", "zh-Hant", "cn", "zh_CN.UTF-8"];
    for variant in variants {
        let i18n = I18n::with_lang(Some(variant));
        assert!(i18n.is_chinese, "Failed for variant: {}", variant);
    }
}

#[test]
fn test_english_variants() {
    let variants = vec!["en", "en_US", "en_GB", "en_US.UTF-8", "C", "POSIX"];
    for variant in variants {
        let i18n = I18n::with_lang(Some(variant));
        assert!(!i18n.is_chinese, "Failed for variant: {}", variant);
    }
}

