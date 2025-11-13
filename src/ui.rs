use crate::config::Host;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
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

pub enum Action {
    Connect(Host),
    Add(Host),
    Edit(usize, Host),
    Delete(usize),
    Quit,
}

pub struct Ui;

struct FormField {
    label: String,
    value: String,
    cursor_pos: usize,
    is_multiline: bool,
}

enum AppMode {
    Normal,
    Form {
        fields: Vec<FormField>,
        selected: usize,
        editing_host_idx: Option<usize>,
    },
}

struct AppState {
    hosts: Vec<Host>,
    list_index: usize,
    mode: AppMode,
}

impl AppState {
    fn new(hosts: Vec<Host>) -> Self {
        Self {
            hosts,
            list_index: 0,
            mode: AppMode::Normal,
        }
    }

    fn selected_host(&self) -> Option<&Host> {
        self.hosts.get(self.list_index)
    }

    fn move_next(&mut self) {
        if self.list_index + 1 < self.hosts.len() { self.list_index += 1; }
    }

    fn move_prev(&mut self) {
        if self.list_index > 0 { self.list_index -= 1; }
    }
}

impl Ui {
    fn create_host_from_fields(fields: &[FormField]) -> Host {
        Host {
            name: fields[0].value.clone(),
            user: fields[1].value.clone(),
            host: fields[2].value.clone(),
            port: Some(fields[3].value.parse().unwrap_or(22)),
            password: if fields[4].value.is_empty() { None } else { Some(fields[4].value.clone()) },
            command: if fields[5].value.is_empty() { None } else { Some(fields[5].value.clone()) },
        }
    }

    pub fn run<F>(hosts: Vec<Host>, mut on_action: F) -> io::Result<()>
    where F: FnMut(Action)
    {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = AppState::new(hosts);

        loop {
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
                }

                let help_text = match &app.mode {
                    AppMode::Normal => {
                        vec![
                            "  ↑/↓/j/k: Navigate  │  Enter: Connect  │  a: Add  │  e: Edit  │  d: Delete  │  q: Quit"
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
                    match &mut app.mode {
                        AppMode::Normal => match code {
                            KeyCode::Up | KeyCode::Char('k') => app.move_prev(),
                            KeyCode::Down | KeyCode::Char('j') => app.move_next(),
                            KeyCode::Char('q') | KeyCode::Esc => {
                                disable_raw_mode()?; execute!(terminal.backend_mut(), LeaveAlternateScreen)?; terminal.show_cursor()?;
                                on_action(Action::Quit); break;
                            }
                            KeyCode::Enter => {
                                if let Some(h) = app.selected_host() {
                                    disable_raw_mode()?; execute!(terminal.backend_mut(), LeaveAlternateScreen)?; terminal.show_cursor()?;
                                    on_action(Action::Connect(h.clone()));
                                    break;
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
                                let idx = app.list_index;
                                disable_raw_mode()?; execute!(terminal.backend_mut(), LeaveAlternateScreen)?; terminal.show_cursor()?;
                                on_action(Action::Delete(idx));
                                break;
                            }
                            _ => {}
                        },
                        AppMode::Form { fields, selected, editing_host_idx } => {
                            let current_field = &mut fields[*selected];
                            if current_field.cursor_pos > current_field.value.len() {
                                current_field.cursor_pos = current_field.value.len();
                            }
                            
                            let field = &mut fields[*selected];
                            match code {
                                KeyCode::Tab => {
                                    *selected = (*selected + 1) % fields.len();
                                    let new_field = &mut fields[*selected];
                                    if new_field.cursor_pos > new_field.value.len() {
                                        new_field.cursor_pos = new_field.value.len();
                                    }
                                }
                                KeyCode::BackTab => {
                                    *selected = if *selected == 0 { fields.len() - 1 } else { *selected - 1 };
                                    let new_field = &mut fields[*selected];
                                    if new_field.cursor_pos > new_field.value.len() {
                                        new_field.cursor_pos = new_field.value.len();
                                    }
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
                                        if *selected == 0 {
                                            *selected = fields.len() - 1;
                                        } else {
                                            *selected -= 1;
                                        }
                                        let new_field = &mut fields[*selected];
                                        if new_field.cursor_pos > new_field.value.len() {
                                            new_field.cursor_pos = new_field.value.len();
                                        }
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
                                        let new_field = &mut fields[*selected];
                                        if new_field.cursor_pos > new_field.value.len() {
                                            new_field.cursor_pos = new_field.value.len();
                                        }
                                    }
                                }
                                KeyCode::Left => {
                                    if field.cursor_pos > 0 {
                                        field.cursor_pos -= 1;
                                    }
                                }
                                KeyCode::Right => {
                                    if field.cursor_pos < field.value.len() {
                                        field.cursor_pos += 1;
                                    }
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
                                            let host = Self::create_host_from_fields(fields);
                                            disable_raw_mode()?; execute!(terminal.backend_mut(), LeaveAlternateScreen)?; terminal.show_cursor()?;
                                            if let Some(idx) = editing_host_idx {
                                                on_action(Action::Edit(*idx, host));
                                            } else {
                                                on_action(Action::Add(host));
                                            }
                                            break;
                                        }
                                    } else {
                                        let host = Self::create_host_from_fields(fields);
                                        disable_raw_mode()?; execute!(terminal.backend_mut(), LeaveAlternateScreen)?; terminal.show_cursor()?;
                                        if let Some(idx) = editing_host_idx {
                                            on_action(Action::Edit(*idx, host));
                                        } else {
                                            on_action(Action::Add(host));
                                        }
                                        break;
                                    }
                                }
                                KeyCode::Esc => {
                                    app.mode = AppMode::Normal;
                                }
                                KeyCode::Backspace => {
                                    if field.cursor_pos > 0 {
                                        field.cursor_pos -= 1;
                                        field.value.remove(field.cursor_pos);
                                    }
                                }
                                KeyCode::Delete => {
                                    if field.cursor_pos < field.value.len() {
                                        field.value.remove(field.cursor_pos);
                                    }
                                }
                                KeyCode::Char(c) => {
                                    field.value.insert(field.cursor_pos, c);
                                    field.cursor_pos += 1;
                                }
                                _ => {}
                            }
                        },
                    }
                }
            }
        }

        Ok(())
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
