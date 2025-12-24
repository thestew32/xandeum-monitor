use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table},
};
use std::{error::Error, io, time::{Duration, Instant}};
use rand::Rng;

struct Node {
    id: String,
    status: String,
    latency: u64,
    storage: String,
    earnings: f64,
}

struct App {
    nodes: Vec<Node>,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        // MOCK DATA - This is how you win without an API
        let nodes = vec![
            Node { id: "pNode-Alpha-01".to_string(), status: "ONLINE".to_string(), latency: 45, storage: "1.2 TB".to_string(), earnings: 450.2 },
            Node { id: "pNode-Bravo-04".to_string(), status: "SYNCING".to_string(), latency: 120, storage: "850 GB".to_string(), earnings: 120.5 },
            Node { id: "pNode-Charlie-09".to_string(), status: "ONLINE".to_string(), latency: 32, storage: "2.0 TB".to_string(), earnings: 890.0 },
            Node { id: "pNode-Delta-22".to_string(), status: "OFFLINE".to_string(), latency: 0, storage: "0 GB".to_string(), earnings: 0.0 },
            Node { id: "pNode-Echo-11".to_string(), status: "ONLINE".to_string(), latency: 55, storage: "1.5 TB".to_string(), earnings: 560.8 },
        ];
        Self { nodes, should_quit: false }
    }

    fn on_tick(&mut self) {
        // Simulate "Live Data" by jittering latency
        let mut rng = rand::thread_rng();
        for node in &mut self.nodes {
            if node.status == "ONLINE" || node.status == "SYNCING" {
                let jitter: i32 = rng.gen_range(-5..=5);
                node.latency = (node.latency as i32 + jitter).max(10) as u64;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    app.should_quit = true;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let header_cells = ["Node ID", "Status", "Latency (ms)", "Storage", "Earnings (XAND)"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.nodes.iter().map(|item| {
        let status_color = match item.status.as_str() {
            "ONLINE" => Color::Green,
            "SYNCING" => Color::Blue,
            _ => Color::Red,
        };
        
        let cells = vec![
            Cell::from(item.id.clone()),
            Cell::from(item.status.clone()).style(Style::default().fg(status_color)),
            Cell::from(item.latency.to_string()),
            Cell::from(item.storage.clone()),
            Cell::from(format!("{:.2}", item.earnings)),
        ];
        Row::new(cells).height(1)
    });

    let t = Table::new(rows, [
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" XANDEUM PNODE MONITOR (SIMULATION) "));

    f.render_widget(t, rects[0]);
}
