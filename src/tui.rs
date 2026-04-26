use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Table,
        TableState,
    },
    Terminal,
};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::monitor::{ForwardingStatus, Monitor};

pub struct App {
    monitor: Arc<Mutex<Monitor>>,
    should_quit: bool,
    table_state: TableState,
}

impl App {
    pub fn new(monitor: Arc<Mutex<Monitor>>) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            monitor,
            should_quit: false,
            table_state,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(250);

        while !self.should_quit {
            terminal.draw(|f| self.ui(f))?;

            if crossterm::event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous();
            }
            _ => {}
        }
    }

    fn get_total_rows(&self) -> usize {
        let monitor = self.monitor.lock().unwrap();
        let mut count = 0;
        for server in monitor.iter_servers() {
            count += 1 + server.forwardings.len();
        }
        count
    }

    fn next(&mut self) {
        let count = self.get_total_rows();
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        let count = self.get_total_rows();
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn ui(&mut self, f: &mut ratatui::Frame) {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(size);

        self.render_header(f, chunks[0]);
        self.render_forwarding_table(f, chunks[1]);
        self.render_footer(f, chunks[2]);
    }

    fn render_header(&self, f: &mut ratatui::Frame, area: Rect) {
        let monitor = self.monitor.lock().unwrap();
        let server_count = monitor.iter_servers().count();
        let total_forwardings: usize = monitor.iter_servers()
            .map(|s| s.forwardings.len())
            .sum();

        let title = Line::from(vec![
            Span::styled(" Portico ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::styled(" - SSH Port Forwarding Monitor ", Style::default().fg(Color::Gray)),
        ]);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let info = Line::from(vec![
            Span::styled("Servers: ", Style::default().fg(Color::White)),
            Span::styled(server_count.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("  |  Forwardings: ", Style::default().fg(Color::White)),
            Span::styled(total_forwardings.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("  |  Press 'q' to quit", Style::default().fg(Color::Yellow)),
        ]);

        let paragraph = Paragraph::new(info)
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }

    fn render_forwarding_table(&mut self, f: &mut ratatui::Frame, area: Rect) {
        let monitor = self.monitor.lock().unwrap();

        let header_cells = ["Status", "Local Port", "Remote", "Process", "Traffic", "Uptime"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));

        let header = Row::new(header_cells)
            .height(1)
            .bottom_margin(1);

        let mut rows = Vec::new();
        
        for server in monitor.iter_servers() {
            // Server header row - created inline to avoid borrow issues
            let server_info = format!(
                "{} | {}@{}:{} | {} forwardings",
                server.name,
                server.username,
                server.host,
                server.port,
                server.forwardings.len()
            );
            
            let server_cell = Cell::from(server_info)
                .style(Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD));
            
            rows.push(Row::new(vec![server_cell]).height(1));
            
            // Forwarding rows for this server
            for forwarding in &server.forwardings {
                let status_style = match forwarding.status {
                    ForwardingStatus::Active => Style::default().fg(Color::Green),
                    ForwardingStatus::Error => Style::default().fg(Color::Red),
                    ForwardingStatus::Unknown => Style::default().fg(Color::Yellow),
                };

                let status_str = match forwarding.status {
                    ForwardingStatus::Active => "  ● Active",
                    ForwardingStatus::Error => "  ✗ Error",
                    ForwardingStatus::Unknown => "  ? Unknown",
                };

                let process = monitor.get_process_for_port(forwarding.local_port)
                    .map(|s| s.as_str())
                    .unwrap_or("-");

                let traffic = monitor.get_traffic_for_port(forwarding.local_port)
                    .map(|t| format!("↑{} ↓{}", format_bytes(t.bytes_sent), format_bytes(t.bytes_received)))
                    .unwrap_or("-".to_string());

                let uptime = format_duration(monitor.get_uptime(forwarding.created_at));

                let cells = vec![
                    Cell::from(status_str).style(status_style),
                    Cell::from(forwarding.local_port.to_string()).style(Style::default().fg(Color::Blue)),
                    Cell::from(format!("{}:{}", forwarding.remote_host, forwarding.remote_port)),
                    Cell::from(process),
                    Cell::from(traffic),
                    Cell::from(uptime),
                ];

                rows.push(Row::new(cells).height(1));
            }
        }

        let table = Table::new(
            rows,
            [
                Constraint::Length(12),
                Constraint::Length(11),
                Constraint::Length(25),
                Constraint::Length(15),
                Constraint::Length(18),
                Constraint::Length(10),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Servers & Forwardings ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

        f.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render_footer(&self, f: &mut ratatui::Frame, area: Rect) {
        let help_text = Line::from(vec![
            Span::styled("↑/↓ or j/k: Navigate  ", Style::default().fg(Color::Gray)),
            Span::styled("q/Esc: Quit", Style::default().fg(Color::Yellow)),
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let paragraph = Paragraph::new(help_text)
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let hours = mins / 24;
    let days = hours / 24;

    if days > 0 {
        format!("{}d{}h", days, hours % 24)
    } else if hours > 0 {
        format!("{}h{}m", hours, mins % 60)
    } else if mins > 0 {
        format!("{}m{}s", mins, secs % 60)
    } else {
        format!("{}s", secs)
    }
}