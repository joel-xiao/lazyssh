mod config;
mod ui;

use config::{Config, Host};
use std::process::Command;
use which::which;

fn main() {
    ensure_sshpass();

    let mut cfg = Config::load();

    loop {
        let hosts = cfg.hosts.clone();
        ui::Ui::run(hosts, |action| match action {
            ui::Action::Connect(h) => {
                ssh_connect(&h);
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

fn ensure_sshpass() {
    if which("sshpass").is_ok() { return; }
    println!("sshpass not found. Please install it manually for macOS or Linux.");
}


fn ssh_connect(h: &Host) {
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
            return build_ssh_command(h, ssh_args);
        }
    }
    
    ssh_args.push(userhost);
    build_ssh_command(h, ssh_args)
}

fn build_ssh_command(h: &Host, ssh_args: Vec<String>) {
    use std::process::Stdio;

    let mut cmd = if let Some(pw) = &h.password {
        if which("sshpass").is_err() {
            eprintln!("sshpass not found. Cannot auto-login.");
            wait_for_keypress();
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
                            eprintln!("\n❌ SSH 连接失败: {}", userhost);
                            eprintln!("   退出代码: {}", exit_code);
                            eprintln!("   可能的原因：网络问题、主机不可达、认证失败等");
                            eprintln!("\n按回车键返回...");
                            wait_for_keypress();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\n❌ 等待 SSH 进程时出错: {}", e);
                    wait_for_keypress();
                }
            }
        }
        Err(e) => {
            eprintln!("\n❌ 无法执行 SSH 命令: {}", e);
            wait_for_keypress();
        }
    }
}

fn wait_for_keypress() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_line(&mut String::new());
}









