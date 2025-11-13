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
            ui::Action::Connect(h) => ssh_connect(&h),
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
        } else {
            ssh_args.push(userhost);
        }
    } else {
        ssh_args.push(userhost);
    }

    let mut cmd = if let Some(pw) = &h.password {
        if which("sshpass").is_err() {
            eprintln!("sshpass not found. Cannot auto-login.");
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

    match cmd.spawn() {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                eprintln!("Failed to wait for SSH process: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute SSH: {}", e);
        }
    }
}









