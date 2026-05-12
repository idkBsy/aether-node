use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, BorderType, Sparkline, List, ListItem, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration, sync::Arc};
use tokio::sync::Mutex;
use std::collections::VecDeque;

pub struct MonitorState {
    pub entropy_data: VecDeque<u64>,
    pub hardware_telemetry: Vec<String>,
    pub attribution_feed: Vec<String>,
    pub running: bool,
}

impl MonitorState {
    pub fn new() -> Self {
        Self {
            entropy_data: VecDeque::from(vec![0; 120]),
            hardware_telemetry: vec![
                "[*] SYNC: GPU Compute Engine (ROCm/HIP)".to_string(),
                "[*] SYNC: Zero-Copy Network Ingestion (libpcap)".to_string(),
                "[+] NIC : wlp3s0 (Promiscuous Mode)".to_string(),
                "[+] HOST: Linux Kernel 5.15+ detected".to_string(),
            ],
            attribution_feed: vec![
                "SYSTEM INIT: AETHER-NODE v0.1.0".to_string(),
                "AUTHOR: idkBsy".to_string(),
                "STATUS: Awaiting hardware signatures...".to_string(),
            ],
            running: true,
        }
    }

    pub fn log_attribution(&mut self, msg: String) {
        // Use system time since chrono isn't present
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let formatted_time = format!("{:02}:{:02}:{:02}", (secs / 3600) % 24, (secs / 60) % 60, secs % 60);
        self.attribution_feed.push(format!("[{}] {}", formatted_time, msg));
        if self.attribution_feed.len() > 50 {
            self.attribution_feed.remove(0);
        }
    }
}

pub async fn run_monitor(state: Arc<Mutex<MonitorState>>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let mut lock = state.lock().await;
        if !lock.running { break; }
        
        terminal.draw(|f| {
            let size = f.size();

            // Main Background Block
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(Span::styled(
                    " AETHER-NODE COMMAND CENTER ",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ));
            f.render_widget(block, size);

            // Layout
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Percentage(40), // Sparkline
                    Constraint::Percentage(60), // Split Bottom
                ].as_ref())
                .split(size);

            // 1. Header Area
            let header = Paragraph::new(Line::from(vec![
                Span::styled("STATUS: ", Style::default().fg(Color::White)),
                Span::styled("ACTIVE", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::styled("MODE: ", Style::default().fg(Color::White)),
                Span::styled("SILICON FINGERPRINTING", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::styled("AUTHOR: ", Style::default().fg(Color::White)),
                Span::styled("idkBsy", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" | Press 'q' to Exit"),
            ]))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(Color::DarkGray)));
            f.render_widget(header, main_chunks[0]);

            // 2. Entropy Sparkline (Middle)
            let entropy_vec: Vec<u64> = lock.entropy_data.iter().cloned().collect();
            let sparkline = Sparkline::default()
                .block(Block::default()
                    .title(" Real-time Entropy Stream (Clock-Skew \u{0394} Variance) ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(Color::DarkGray)))
                .data(&entropy_vec)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(sparkline, main_chunks[1]);

            // Split the bottom area into Telemetry and Logs
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Percentage(60),
                ].as_ref())
                .split(main_chunks[2]);

            // 3. Hardware Telemetry (Bottom Left)
            let telemetry_items: Vec<ListItem> = lock
                .hardware_telemetry
                .iter()
                .map(|i| ListItem::new(Span::styled(i.clone(), Style::default().fg(Color::LightBlue))))
                .collect();
            let telemetry_list = List::new(telemetry_items)
                .block(Block::default()
                    .title(" Hardware Telemetry ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)));
            f.render_widget(telemetry_list, bottom_chunks[0]);

            // 4. Attribution Console (Bottom Right)
            let attribution_items: Vec<ListItem> = lock
                .attribution_feed
                .iter()
                .map(|i| {
                    let style = if i.contains("ERROR") {
                        Style::default().fg(Color::Red)
                    } else if i.contains("LOCK") || i.contains("Confirmed") {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::LightMagenta)
                    };
                    ListItem::new(Span::styled(i.clone(), style))
                })
                .collect();
            let attribution_list = List::new(attribution_items)
                .block(Block::default()
                    .title(" Attribution & OSINT Console ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)));
            f.render_widget(attribution_list, bottom_chunks[1]);
        })?;

        // Non-blocking input handling
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c')) {
                    lock.running = false;
                    break;
                }
            }
        }
        
        // Simulating data updates for the TUI graph (flowing effect)
        lock.entropy_data.pop_front();
        let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let wave = ((time as f64 / 200.0).sin() * 20.0) as u64 + 20;
        let noise = (time % 15) as u64;
        lock.entropy_data.push_back(wave + noise);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
