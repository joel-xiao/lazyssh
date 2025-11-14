mod config;
mod ui;
mod i18n;

use config::{Config, Host};
use std::process::Command;
use std::env;
use which::which;
use i18n::I18n;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lang_override: Option<String> = None;
    let mut i = 1;
    
    while i < args.len() {
        match args[i].as_str() {
            "--version" | "-V" | "-v" => {
                println!("lazyssh {}", VERSION);
                return;
            }
            "--help" | "-h" => {
                let i18n = I18n::with_lang(lang_override.as_deref());
                print_help(&i18n);
                return;
            }
            "--lang" | "-l" => {
                if i + 1 < args.len() {
                    lang_override = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --lang requires a language code (e.g., zh, en)");
                    std::process::exit(1);
                }
            }
            _ => {
                let i18n = I18n::with_lang(lang_override.as_deref());
                eprintln!("{}", i18n.unknown_arg(&args[i]));
                eprintln!("{}", i18n.use_help());
                std::process::exit(1);
            }
        }
    }
    
    let i18n = I18n::with_lang(lang_override.as_deref());
    ensure_sshpass(&i18n);

    let mut cfg = Config::load();

    loop {
        let hosts = cfg.hosts.clone();
        let i18n_clone = I18n::with_lang(lang_override.as_deref());
        ui::Ui::run(hosts, i18n_clone, |action| match action {
            ui::Action::Connect(h) => {
                ssh_connect(&h, &i18n);
            }
            ui::Action::Add(h) => {
                cfg.add_host(h);
                cfg.save();
            }
            ui::Action::Edit(idx, h) => {
                if idx < cfg.hosts.len() {
                    cfg.update_host(idx, h);
                    cfg.save();
                }
            }
            ui::Action::Delete(idx) => {
                cfg.remove_host(idx);
                cfg.save();
            }
            ui::Action::Copy => {}
            ui::Action::Quit => std::process::exit(0),
        }).ok();
    }
}

fn print_help(i18n: &I18n) {
    println!("{}", i18n.help_title());
    println!();
    println!("{}", i18n.help_usage());
    println!();
    println!("{}", i18n.help_options());
    println!("  -V, --version    {}", i18n.help_version());
    println!("  -h, --help       {}", i18n.help_help());
    println!("  -l, --lang CODE  {}", i18n.help_lang());
    println!();
    println!("{}", i18n.help_no_args());
    println!();
    if i18n.is_chinese {
        println!("语言设置优先级：命令行参数 > 环境变量 LAZYSSH_LANG > 系统语言");
    } else {
        println!("Language priority: command line argument > LAZYSSH_LANG env > system language");
    }
}

fn ensure_sshpass(i18n: &I18n) {
    if which("sshpass").is_ok() { return; }
    println!("{}", i18n.sshpass_not_found());
}


fn ssh_connect(h: &Host, i18n: &I18n) {
    let mut ssh_args = vec!["-t".to_string()];
    
    ssh_args.push("-o".to_string());
    ssh_args.push("ConnectTimeout=30".to_string());
    
    ssh_args.push("-o".to_string());
    ssh_args.push("StrictHostKeyChecking=accept-new".to_string());
    
    if let Some(port) = h.port {
        ssh_args.push("-p".to_string());
        ssh_args.push(port.to_string());
    }

    let userhost = format!("{}@{}", h.user, h.host);

    if let Some(cmd) = &h.command {
        let commands: Vec<&str> = cmd.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();
        
        if !commands.is_empty() {
            let combined_cmd = commands.join("; ");
            ssh_args.push(userhost);
            ssh_args.push(format!("{}; exec $SHELL -l", combined_cmd));
            return build_ssh_command(h, ssh_args, i18n);
        }
    }
    
    ssh_args.push(userhost);
    build_ssh_command(h, ssh_args, i18n)
}

fn build_ssh_command(h: &Host, ssh_args: Vec<String>, i18n: &I18n) {
    use std::process::Stdio;

    let mut cmd = if let Some(pw) = &h.password {
        if which("sshpass").is_err() {
            eprintln!("{}", i18n.sshpass_cannot_login());
            wait_for_keypress(i18n);
            return;
        }
        let mut sshpass_cmd = Command::new("sshpass");
        sshpass_cmd.arg("-p").arg(pw).arg("ssh");
        sshpass_cmd.args(&ssh_args);
        sshpass_cmd
    } else {
        let mut ssh_cmd = Command::new("ssh");
        ssh_cmd.args(&ssh_args);
        ssh_cmd
    };

    cmd.stdin(Stdio::inherit())
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());

    match cmd.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    if let Some(exit_code) = status.code() {
                        if exit_code == 255 {
                            let userhost = format!("{}@{}", h.user, h.host);
                            eprintln!("{}", i18n.ssh_connection_failed(&userhost));
                            eprintln!("{}{}", i18n.exit_code(), exit_code);
                            eprintln!("{}", i18n.possible_reasons());
                            eprintln!("{}", i18n.press_enter_to_return());
                            wait_for_keypress(i18n);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", i18n.wait_ssh_process_error(&e.to_string()));
                    wait_for_keypress(i18n);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", i18n.execute_ssh_error(&e.to_string()));
            wait_for_keypress(i18n);
        }
    }
}

fn wait_for_keypress(_i18n: &I18n) {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_line(&mut String::new());
}









