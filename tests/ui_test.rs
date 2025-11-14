use lazyssh::config::Host;
use lazyssh::ui::{Ui, FormField, AppState};

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[test]
fn test_truncate() {
    assert_eq!(truncate("hello", 10), "hello");
    assert_eq!(truncate("hello world", 5), "he...");
    assert_eq!(truncate("test", 4), "test");
    assert_eq!(truncate("very long string", 10), "very lo...");
}

#[test]
fn test_parse_ssh_command() {
    let host = Ui::parse_ssh_command("ssh user@host").unwrap();
    assert_eq!(host.user, "user");
    assert_eq!(host.host, "host");
    assert_eq!(host.port, Some(22));
    assert_eq!(host.name, "user@host");

    let host = Ui::parse_ssh_command("ssh -p 2222 user@host").unwrap();
    assert_eq!(host.user, "user");
    assert_eq!(host.host, "host");
    assert_eq!(host.port, Some(2222));

    let host = Ui::parse_ssh_command("ssh user@example.com").unwrap();
    assert_eq!(host.user, "user");
    assert_eq!(host.host, "example.com");

    assert!(Ui::parse_ssh_command("not ssh command").is_none());
    assert!(Ui::parse_ssh_command("ssh").is_none());
    assert!(Ui::parse_ssh_command("ssh @host").is_none());
    assert!(Ui::parse_ssh_command("ssh user@").is_none());
}

#[test]
fn test_create_host_from_fields() {
    let fields = vec![
        FormField { label: "Name".into(), value: "test".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "User".into(), value: "user".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Host".into(), value: "host".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Port".into(), value: "2222".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Password".into(), value: "pass".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Command".into(), value: "ls".into(), cursor_pos: 0, is_multiline: true },
    ];
    let host = Ui::create_host_from_fields(&fields);
    assert_eq!(host.name, "test");
    assert_eq!(host.user, "user");
    assert_eq!(host.host, "host");
    assert_eq!(host.port, Some(2222));
    assert_eq!(host.password, Some("pass".to_string()));
    assert_eq!(host.command, Some("ls".to_string()));

    let fields = vec![
        FormField { label: "Name".into(), value: "test2".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "User".into(), value: "user2".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Host".into(), value: "host2".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Port".into(), value: "invalid".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Password".into(), value: "".into(), cursor_pos: 0, is_multiline: false },
        FormField { label: "Command".into(), value: "".into(), cursor_pos: 0, is_multiline: true },
    ];
    let host = Ui::create_host_from_fields(&fields);
    assert_eq!(host.port, Some(22));
    assert_eq!(host.password, None);
    assert_eq!(host.command, None);
}

#[test]
fn test_normalize_cursor_pos() {
    let mut field = FormField {
        label: "Test".into(),
        value: "hello".into(),
        cursor_pos: 10,
        is_multiline: false,
    };
    Ui::normalize_cursor_pos(&mut field);
    assert_eq!(field.cursor_pos, 5);

    let mut field = FormField {
        label: "Test".into(),
        value: "hello".into(),
        cursor_pos: 3,
        is_multiline: false,
    };
    Ui::normalize_cursor_pos(&mut field);
    assert_eq!(field.cursor_pos, 3);
}

#[test]
fn test_app_state() {
    let hosts = vec![
        Host {
            name: "test1".into(),
            user: "user1".into(),
            host: "host1".into(),
            port: Some(22),
            password: None,
            command: None,
        },
        Host {
            name: "test2".into(),
            user: "user2".into(),
            host: "host2".into(),
            port: Some(2222),
            password: None,
            command: None,
        },
    ];
    let mut app = AppState::new(hosts);
    assert_eq!(app.list_index, 0);
    assert!(app.selected_host().is_some());
    assert_eq!(app.selected_host().unwrap().name, "test1");

    app.move_next();
    assert_eq!(app.list_index, 1);
    assert_eq!(app.selected_host().unwrap().name, "test2");

    app.move_prev();
    assert_eq!(app.list_index, 0);

    app.move_prev();
    assert_eq!(app.list_index, 0);
}

