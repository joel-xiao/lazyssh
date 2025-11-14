use crate::config::Host;
use crate::i18n::I18n;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use clipboard::ClipboardProvider;

pub enum Action {
    Connect(Host),
    Add(Host),
    Edit(usize, Host),
    Delete(usize),
    Copy,
    Quit,
}

pub struct Ui;

pub struct FormField {
    pub label: String,
    pub value: String,
    pub cursor_pos: usize,
    pub is_multiline: bool,
}

enum AppMode {
    Normal,
    Form {
        fields: Vec<FormField>,
        selected: usize,
        editing_host_idx: Option<usize>,
    },
    ConfirmDelete {
        host_idx: usize,
        host_name: String,
    },
}

pub struct AppState {
    pub hosts: Vec<Host>,
    pub list_index: usize,
    mode: AppMode,
    clipboard: Option<Host>,
}

impl AppState {
    pub fn new(hosts: Vec<Host>) -> Self {
        Self {
            hosts,
            list_index: 0,
            mode: AppMode::Normal,
            clipboard: None,
        }
    }

    pub fn selected_host(&self) -> Option<&Host> {
        self.hosts.get(self.list_index)
    }

    pub fn move_next(&mut self) {
        if self.list_index + 1 < self.hosts.len() { self.list_index += 1; }
    }

    pub fn move_prev(&mut self) {
        if self.list_index > 0 { self.list_index -= 1; }
    }
}

impl Ui {
    fn exit_tui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }

    pub fn normalize_cursor_pos(field: &mut FormField) {
        if field.cursor_pos > field.value.len() {
            field.cursor_pos = field.value.len();
        }
        field.cursor_pos = field.value.char_indices()
            .map(|(i, _)| i)
            .chain(std::iter::once(field.value.len()))
            .find(|&i| i >= field.cursor_pos)
            .unwrap_or(field.value.len());
    }

    fn move_cursor_left(value: &str, pos: usize) -> usize {
        if pos == 0 {
            return 0;
        }
        value.char_indices()
            .map(|(i, _)| i)
            .rev()
            .find(|&i| i < pos)
            .unwrap_or(0)
    }

    fn move_cursor_right(value: &str, pos: usize) -> usize {
        if pos >= value.len() {
            return value.len();
        }
        value.char_indices()
            .map(|(i, _)| i)
            .find(|&i| i > pos)
            .unwrap_or(value.len())
    }

    fn validate_and_exit_on_error(host: &Host, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, i18n_ref: &I18n) -> io::Result<()> {
        if host.user.is_empty() || host.host.is_empty() {
            Self::exit_tui(terminal)?;
            eprintln!("{}", i18n_ref.invalid_host_format());
            std::process::exit(1);
        }
        Ok(())
    }

    fn save_form_and_exit<F>(
        fields: &[FormField],
        editing_host_idx: Option<usize>,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        on_action: &mut F,
    ) -> io::Result<()>
    where
        F: FnMut(Action),
    {
        let host = Self::create_host_from_fields(fields);
        Self::exit_tui(terminal)?;
        if let Some(idx) = editing_host_idx {
            on_action(Action::Edit(idx, host));
        } else {
            on_action(Action::Add(host));
        }
        Ok(())
    }

    pub fn create_host_from_fields(fields: &[FormField]) -> Host {
        Host {
            name: fields[0].value.clone(),
            user: fields[1].value.clone(),
            host: fields[2].value.clone(),
            port: Some(fields[3].value.parse().unwrap_or(22)),
            password: if fields[4].value.is_empty() { None } else { Some(fields[4].value.clone()) },
            command: if fields[5].value.is_empty() { None } else { Some(fields[5].value.clone()) },
        }
    }

    pub fn run<F>(hosts: Vec<Host>, i18n: I18n, mut on_action: F) -> io::Result<()>
    where F: FnMut(Action)
    {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = AppState::new(hosts);

        loop {
            let i18n_ref = &i18n;
            terminal.draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                    .split(size);

                let main_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(45), Constraint::Percentage(55)].as_ref())
                    .split(chunks[0]);

                let items: Vec<ListItem> = app.hosts.iter().enumerate().map(|(idx, h)| {
                    let display = if !app.hosts.is_empty() && idx == app.list_index {
                        format!("▶ {} @ {}", h.name, h.host)
                    } else {
                        format!("  {} @ {}", h.name, h.host)
                    };
                    ListItem::new(display)
                        .style(if idx == app.list_index {
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        })
                }).collect();
                
                let mut list_state = tui::widgets::ListState::default();
                list_state.select(Some(app.list_index));
                
                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Cyan))
                            .title(Spans::from(vec![
                                Span::styled("📡 SSH Hosts", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                            ]))
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::Cyan)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    )
                    .highlight_symbol("▶ ");
                f.render_stateful_widget(list, main_chunks[0], &mut list_state);

                match &app.mode {
                    AppMode::Normal => {
                        if let Some(h) = app.selected_host() {
                            let info_lines = [
                                format!("┌─ Host Information ──────────────────────┐"),
                                format!("│ Name:    {:40} │", truncate(&h.name, 40)),
                                format!("│ User:    {:40} │", truncate(&h.user, 40)),
                                format!("│ Host:    {:40} │", truncate(&h.host, 40)),
                                format!("│ Port:    {:40} │", h.port.unwrap_or(22).to_string()),
                                format!("│ Password: {:39} │", 
                                    if h.password.is_some() {
                                        truncate(h.password.as_ref().unwrap(), 39)
                                    } else {
                                        "(not set)".to_string()
                                    }
                                ),
                                format!("│ Command: {:40} │", 
                                    truncate(&h.command.clone().unwrap_or_else(|| "(none)".to_string()), 40)
                                ),
                                format!("└──────────────────────────────────────────┘"),
                            ]
                            .to_vec();
                            
                            let info_widget = Paragraph::new(info_lines.join("\n"))
                                .style(Style::default().fg(Color::Green))
                                .block(
                                    Block::default()
                                        .borders(Borders::ALL)
                                        .border_style(Style::default().fg(Color::Green))
                                        .title(Spans::from(vec![
                                            Span::styled("ℹ️  Host Details", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                                        ]))
                                );
                        f.render_widget(info_widget, main_chunks[1]);
                        } else {
                            let empty_msg = [
                                "┌──────────────────────────────────────────┐",
                                "│                                          │",
                                "│         No host selected                 │",
                                "│                                          │",
                                "│     Press 'a' to add a new host         │",
                                "│                                          │",
                                "└──────────────────────────────────────────┘",
                            ]
                            .map(|s| s.to_string())
                            .to_vec();
                            let empty_widget = Paragraph::new(empty_msg.join("\n"))
                                .style(Style::default().fg(Color::DarkGray))
                                .block(
                                    Block::default()
                                        .borders(Borders::ALL)
                                        .border_style(Style::default().fg(Color::DarkGray))
                                        .title("ℹ️  Host Details")
                                );
                            f.render_widget(empty_widget, main_chunks[1]);
                        }
                    }
                    AppMode::Form { fields, selected, editing_host_idx } => {
                        let title = if editing_host_idx.is_some() {
                            "✏️  Edit Host"
                        } else {
                            "➕ Add New Host"
                        };
                        
                        let mut form_lines = vec![format!("┌─ {} ────────────────────────────────┐", title)];
                        
                        for (i, field) in fields.iter().enumerate() {
                            let is_selected = i == *selected;
                            let prefix = if is_selected { "│▶" } else { "│ " };
                            let label = format!("{} {:12}: ", prefix, field.label);
                            
                            if field.is_multiline {
                                form_lines.push(label.clone());
                                
                                let cursor_pos = field.cursor_pos.min(field.value.len());
                                let mut char_count = 0;
                                let mut cursor_line_idx = 0;
                                let mut cursor_col = 0;
                                
                                for (line_idx, line) in field.value.lines().enumerate() {
                                    let line_len = line.len();
                                    if char_count <= cursor_pos && cursor_pos <= char_count + line_len {
                                        cursor_line_idx = line_idx;
                                        cursor_col = cursor_pos - char_count;
                                        break;
                                    }
                                    char_count += line_len + 1;
                                }
                                
                                if cursor_pos > char_count {
                                    let value_lines: Vec<&str> = field.value.lines().collect();
                                    if !value_lines.is_empty() {
                                        cursor_line_idx = value_lines.len() - 1;
                                        cursor_col = value_lines.last().unwrap().len();
                                    }
                                }
                                
                                let value_lines: Vec<&str> = field.value.lines().collect();
                                if value_lines.is_empty() || (value_lines.len() == 1 && value_lines[0].is_empty()) {
                                    if is_selected {
                                        form_lines.push("│             ▊".to_string());
                                    } else {
                                        form_lines.push("│             (empty)".to_string());
                                    }
                                } else {
                                    for (line_idx, line) in value_lines.iter().enumerate() {
                                        let is_cursor_line = is_selected && line_idx == cursor_line_idx;
                                        
                                        let line_display = if is_cursor_line {
                                            let cursor_col = cursor_col.min(line.len());
                                            let (before, after) = line.split_at(cursor_col);
                                            format!("{}▊{}", before, after)
                                        } else {
                                            line.to_string()
                                        };
                                        
                                        let max_width = 38;
                                        let display = if line_display.len() > max_width {
                                            format!("{}...", &line_display[..max_width.saturating_sub(3)])
                                        } else {
                                            line_display
                                        };
                                        
                                        let line_num = format!("{:2}", line_idx + 1);
                                        form_lines.push(format!("│ {} │ {}", line_num, display));
                                    }
                                }
                            } else {
                                let value_display = if field.value.is_empty() {
                                    if is_selected {
                                        "▊".to_string()
                                    } else {
                                        "(empty)".to_string()
                                    }
                                } else {
                                    let cursor_pos = field.cursor_pos.min(field.value.len());
                                    let (before, after) = field.value.split_at(cursor_pos);
                                    if is_selected {
                                        format!("{}▊{}", before, after)
                                    } else {
                                        field.value.clone()
                                    }
                                };
                                
                                let max_value_width = 38;
                                let display_value = if value_display.len() > max_value_width {
                                    format!("{}...", &value_display[..max_value_width.saturating_sub(3)])
                                } else {
                                    value_display
                                };
                                
                                form_lines.push(format!("{}{:38} │", label, display_value));
                            }
                            
                            if is_selected && i < fields.len() - 1 {
                                form_lines.push("│  ────────────────────────────────────────│".to_string());
                            }
                        }
                        
                        form_lines.push("└──────────────────────────────────────────┘".to_string());
                        
                        let form_widget = Paragraph::new(form_lines.join("\n"))
                            .style(Style::default().fg(if *selected < fields.len() {
                                Color::Yellow
                            } else {
                                Color::White
                            }))
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(Color::Yellow))
                                    .title(Spans::from(vec![
                                        Span::styled(title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                                    ]))
                            );
                        f.render_widget(form_widget, main_chunks[1]);
                    }
                    AppMode::ConfirmDelete { host_name, .. } => {
                        let host_name_display = i18n_ref.confirm_delete_host(&truncate(host_name, 30));
                        let confirm_lines = vec![
                            "┌──────────────────────────────────────────┐",
                            "│                                          │",
                            host_name_display.as_str(),
                            "│                                          │",
                            i18n_ref.press_y_to_confirm(),
                            "│                                          │",
                            "└──────────────────────────────────────────┘",
                        ];
                        
                        let confirm_widget = Paragraph::new(confirm_lines.join("\n"))
                            .style(Style::default().fg(Color::Red))
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(Color::Red))
                                    .title(Spans::from(vec![
                                        Span::styled("⚠️  删除确认", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                                    ]))
                            );
                        f.render_widget(confirm_widget, main_chunks[1]);
                    }
                }

                let help_text = match &app.mode {
                    AppMode::Normal => {
                        vec![
                            "  ↑/↓/j/k: Navigate  │  Enter: Connect  │  a: Add  │  e: Edit  │  d: Delete  │  y: Copy  │  p: Paste  │  q/Ctrl+C: Quit"
                        ]
                    },
                    AppMode::ConfirmDelete { .. } => {
                        vec![
                            i18n_ref.confirm_delete()
                        ]
                    },
                    AppMode::Form { fields, selected, .. } => {
                        let is_multiline = fields.get(*selected).map(|f| f.is_multiline).unwrap_or(false);
                        if is_multiline {
                            vec![
                                "  ←/→: Move Cursor  │  ↑/↓: Move Line  │  Shift+Enter: New Line  │  Enter: Save  │  Esc: Cancel"
                            ]
                        } else {
                            vec![
                                "  ←/→: Move Cursor  │  Home/End: Jump  │  Tab/↓: Next  │  Shift+Tab/↑: Prev  │  Enter: Save  │  Esc: Cancel"
                            ]
                        }
                    },
                };
                
                let help = Paragraph::new(help_text.join("\n"))
                    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Cyan))
                            .title(Spans::from(vec![
                                Span::styled("⌨️  Keyboard Shortcuts", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                            ]))
                    );
                f.render_widget(help, chunks[1]);
            })?;

            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                    if code == KeyCode::Char('c') && (modifiers.contains(KeyModifiers::CONTROL) || modifiers.contains(KeyModifiers::SUPER)) {
                        Self::exit_tui(&mut terminal)?;
                        on_action(Action::Quit);
                        break;
                    }
                    
                    match &mut app.mode {
                        AppMode::Normal => match code {
                            KeyCode::Up | KeyCode::Char('k') => app.move_prev(),
                            KeyCode::Down | KeyCode::Char('j') => app.move_next(),
                            KeyCode::Char('q') | KeyCode::Esc => {
                                Self::exit_tui(&mut terminal)?;
                                on_action(Action::Quit);
                                break;
                            }
                            KeyCode::Enter => {
                                if let Some(h) = app.selected_host() {
                                    Self::exit_tui(&mut terminal)?;
                                    on_action(Action::Connect(h.clone()));
                                    break;
                                }
                            }
                            KeyCode::Char('y') => {
                                if let Some(h) = app.selected_host() {
                                    let host_clone = h.clone();
                                    app.clipboard = Some(host_clone.clone());
                                    match clipboard::ClipboardContext::new() {
                                        Ok(mut ctx) => {
                                            let ssh_cmd = format!("ssh -p {} {}@{}", 
                                                host_clone.port.unwrap_or(22), host_clone.user, host_clone.host);
                                            let _ = ctx.set_contents(ssh_cmd);
                                        }
                                        Err(_) => {}
                                    }
                                    on_action(Action::Copy);
                                }
                            }
                            KeyCode::Char('p') => {
                                if let Some(clipped_host) = &app.clipboard {
                                    let new_host = clipped_host.clone();
                                    Self::validate_and_exit_on_error(&new_host, &mut terminal, &i18n)?;
                                    Self::exit_tui(&mut terminal)?;
                                    on_action(Action::Add(new_host));
                                    break;
                                } else {
                                    match clipboard::ClipboardContext::new() {
                                        Ok(mut ctx) => {
                                            match ctx.get_contents() {
                                                Ok(content) => {
                                                    if let Some(parsed_host) = Self::parse_ssh_command(&content) {
                                                        Self::validate_and_exit_on_error(&parsed_host, &mut terminal, &i18n)?;
                                                        app.clipboard = Some(parsed_host.clone());
                                                        Self::exit_tui(&mut terminal)?;
                                                        on_action(Action::Add(parsed_host));
                                                        break;
                                                    } else {
                                                        Self::exit_tui(&mut terminal)?;
                                                        eprintln!("{}", i18n.clipboard_parse_error());
                                                        std::process::exit(1);
                                                    }
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                            KeyCode::Char('a') => {
                                let fields = vec![
                                    FormField { label: "Name".into(), value: "".into(), cursor_pos: 0, is_multiline: false },
                                    FormField { label: "User".into(), value: "".into(), cursor_pos: 0, is_multiline: false },
                                    FormField { label: "Host".into(), value: "".into(), cursor_pos: 0, is_multiline: false },
                                    FormField { label: "Port".into(), value: "22".into(), cursor_pos: 2, is_multiline: false },
                                    FormField { label: "Password".into(), value: "".into(), cursor_pos: 0, is_multiline: false },
                                    FormField { label: "Command".into(), value: "".into(), cursor_pos: 0, is_multiline: true },
                                ];
                                app.mode = AppMode::Form { fields, selected: 0, editing_host_idx: None };
                            }
                            KeyCode::Char('e') => {
                                if let Some(idx) = app.hosts.get(app.list_index) {
                                    let name_len = idx.name.len();
                                    let user_len = idx.user.len();
                                    let host_len = idx.host.len();
                                    let port_str = idx.port.unwrap_or(22).to_string();
                                    let port_len = port_str.len();
                                    let password_str = idx.password.clone().unwrap_or_default();
                                    let password_len = password_str.len();
                                    let command_str = idx.command.clone().unwrap_or_default();
                                    let command_len = command_str.len();
                                    
                                    let fields = vec![
                                        FormField { label: "Name".into(), value: idx.name.clone(), cursor_pos: name_len, is_multiline: false },
                                        FormField { label: "User".into(), value: idx.user.clone(), cursor_pos: user_len, is_multiline: false },
                                        FormField { label: "Host".into(), value: idx.host.clone(), cursor_pos: host_len, is_multiline: false },
                                        FormField { label: "Port".into(), value: port_str, cursor_pos: port_len, is_multiline: false },
                                        FormField { label: "Password".into(), value: password_str, cursor_pos: password_len, is_multiline: false },
                                        FormField { label: "Command".into(), value: command_str, cursor_pos: command_len, is_multiline: true },
                                    ];
                                    app.mode = AppMode::Form { fields, selected: 0, editing_host_idx: Some(app.list_index) };
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(h) = app.selected_host() {
                                    let idx = app.list_index;
                                    app.mode = AppMode::ConfirmDelete {
                                        host_idx: idx,
                                        host_name: h.name.clone(),
                                    };
                                }
                            }
                            _ => {}
                        },
                        AppMode::Form { fields, selected, editing_host_idx } => {
                            Self::normalize_cursor_pos(&mut fields[*selected]);
                            let field = &mut fields[*selected];
                            match code {
                                KeyCode::Tab => {
                                    *selected = (*selected + 1) % fields.len();
                                    Self::normalize_cursor_pos(&mut fields[*selected]);
                                }
                                KeyCode::BackTab => {
                                    *selected = if *selected == 0 { fields.len() - 1 } else { *selected - 1 };
                                    Self::normalize_cursor_pos(&mut fields[*selected]);
                                }
                                KeyCode::Up => {
                                    if field.is_multiline {
                                        let cursor_pos = field.cursor_pos.min(field.value.len());
                                        let lines_before: Vec<&str> = field.value[..cursor_pos].lines().collect();
                                        let current_line_idx = lines_before.len().saturating_sub(1);
                                        
                                        if current_line_idx > 0 {
                                            let current_line_start = field.value.lines().take(current_line_idx).map(|l| l.len() + 1).sum::<usize>();
                                            let cursor_in_line = cursor_pos - current_line_start;
                                            
                                            let prev_line_start = field.value.lines().take(current_line_idx - 1).map(|l| l.len() + 1).sum::<usize>();
                                            let prev_line = field.value.lines().nth(current_line_idx - 1).unwrap_or("");
                                            let new_pos = prev_line_start + cursor_in_line.min(prev_line.len());
                                            field.cursor_pos = new_pos;
                                        }
                                    } else {
                                        *selected = if *selected == 0 { fields.len() - 1 } else { *selected - 1 };
                                        Self::normalize_cursor_pos(&mut fields[*selected]);
                                    }
                                }
                                KeyCode::Down => {
                                    if field.is_multiline {
                                        let cursor_pos = field.cursor_pos.min(field.value.len());
                                        let lines_before: Vec<&str> = field.value[..cursor_pos].lines().collect();
                                        let current_line_idx = lines_before.len().saturating_sub(1);
                                        let total_lines: Vec<&str> = field.value.lines().collect();
                                        
                                        if current_line_idx < total_lines.len().saturating_sub(1) {
                                            let current_line_start = field.value.lines().take(current_line_idx).map(|l| l.len() + 1).sum::<usize>();
                                            let cursor_in_line = cursor_pos - current_line_start;
                                            
                                            let next_line_start = field.value.lines().take(current_line_idx + 1).map(|l| l.len() + 1).sum::<usize>();
                                            let next_line = total_lines.get(current_line_idx + 1).unwrap_or(&"");
                                            let new_pos = next_line_start + cursor_in_line.min(next_line.len());
                                            field.cursor_pos = new_pos.min(field.value.len());
                                        }
                                    } else {
                                        *selected = (*selected + 1) % fields.len();
                                        Self::normalize_cursor_pos(&mut fields[*selected]);
                                    }
                                }
                                KeyCode::Left => {
                                    field.cursor_pos = Self::move_cursor_left(&field.value, field.cursor_pos);
                                }
                                KeyCode::Right => {
                                    field.cursor_pos = Self::move_cursor_right(&field.value, field.cursor_pos);
                                }
                                KeyCode::Home => {
                                    field.cursor_pos = 0;
                                }
                                KeyCode::End => {
                                    field.cursor_pos = field.value.len();
                                }
                                KeyCode::Enter => {
                                    if field.is_multiline {
                                        let is_shift = modifiers.contains(crossterm::event::KeyModifiers::SHIFT);
                                        
                                        if is_shift {
                                            field.value.insert(field.cursor_pos, '\n');
                                            field.cursor_pos += 1;
                                        } else {
                                            Self::save_form_and_exit(fields, *editing_host_idx, &mut terminal, &mut on_action)?;
                                            break;
                                        }
                                    } else {
                                        Self::save_form_and_exit(fields, *editing_host_idx, &mut terminal, &mut on_action)?;
                                        break;
                                    }
                                }
                                KeyCode::Esc => {
                                    app.mode = AppMode::Normal;
                                }
                                KeyCode::Backspace => {
                                    if field.cursor_pos > 0 {
                                        let new_pos = Self::move_cursor_left(&field.value, field.cursor_pos);
                                        let char_len = field.cursor_pos - new_pos;
                                        field.value.drain(new_pos..new_pos + char_len);
                                        field.cursor_pos = new_pos;
                                    }
                                }
                                KeyCode::Delete => {
                                    if field.cursor_pos < field.value.len() {
                                        let next_pos = Self::move_cursor_right(&field.value, field.cursor_pos);
                                        let char_len = next_pos - field.cursor_pos;
                                        field.value.drain(field.cursor_pos..field.cursor_pos + char_len);
                                    }
                                }
                                KeyCode::Char(c) => {
                                    let char_len = c.len_utf8();
                                    field.value.insert(field.cursor_pos, c);
                                    field.cursor_pos += char_len;
                                }
                                _ => {}
                            }
                        },
                        AppMode::ConfirmDelete { host_idx, .. } => {
                            match code {
                                KeyCode::Char('y') => {
                                    Self::exit_tui(&mut terminal)?;
                                    on_action(Action::Delete(*host_idx));
                                    break;
                                }
                                KeyCode::Char('n') | KeyCode::Esc => {
                                    app.mode = AppMode::Normal;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn parse_ssh_command(cmd: &str) -> Option<Host> {
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() || parts[0] != "ssh" {
            return None;
        }

        let mut port = 22u16;
        let mut user_host = None;

        let mut i = 1;
        while i < parts.len() {
            if parts[i] == "-p" && i + 1 < parts.len() {
                if let Ok(p) = parts[i + 1].parse::<u16>() {
                    port = p;
                    i += 2;
                    continue;
                }
            }
            if !parts[i].starts_with('-') {
                user_host = Some(parts[i]);
                break;
            }
            i += 1;
        }

        if let Some(uh) = user_host {
            if let Some(at_pos) = uh.find('@') {
                let user = uh[..at_pos].to_string();
                let host = uh[at_pos + 1..].to_string();
                if !user.is_empty() && !host.is_empty() {
                    return Some(Host {
                        name: format!("{}@{}", user, host),
                        user,
                        host,
                        port: Some(port),
                        password: None,
                        command: None,
                    });
                }
            }
        }

        None
    }
}

pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

