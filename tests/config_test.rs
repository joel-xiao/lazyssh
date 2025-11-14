use lazyssh::config::{Config, Host};

#[test]
fn test_add_host() {
    let mut config = Config { hosts: vec![] };
    let host = Host {
        name: "test".into(),
        user: "user".into(),
        host: "host".into(),
        port: Some(22),
        password: None,
        command: None,
    };
    config.add_host(host);
    assert_eq!(config.hosts.len(), 1);
    assert_eq!(config.hosts[0].name, "test");
}

#[test]
fn test_remove_host() {
    let mut config = Config {
        hosts: vec![
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
                port: Some(22),
                password: None,
                command: None,
            },
        ],
    };
    config.remove_host(0);
    assert_eq!(config.hosts.len(), 1);
    assert_eq!(config.hosts[0].name, "test2");

    config.remove_host(10);
    assert_eq!(config.hosts.len(), 1);
}

#[test]
fn test_update_host() {
    let mut config = Config {
        hosts: vec![
            Host {
                name: "test1".into(),
                user: "user1".into(),
                host: "host1".into(),
                port: Some(22),
                password: None,
                command: None,
            },
        ],
    };
    let updated_host = Host {
        name: "updated".into(),
        user: "user2".into(),
        host: "host2".into(),
        port: Some(2222),
        password: None,
        command: None,
    };
    config.update_host(0, updated_host);
    assert_eq!(config.hosts[0].name, "updated");
    assert_eq!(config.hosts[0].user, "user2");
    assert_eq!(config.hosts[0].port, Some(2222));

    let new_host = Host {
        name: "new".into(),
        user: "user3".into(),
        host: "host3".into(),
        port: Some(22),
        password: None,
        command: None,
    };
    config.update_host(10, new_host);
    assert_eq!(config.hosts.len(), 1);
}

