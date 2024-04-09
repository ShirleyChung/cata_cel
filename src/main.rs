use crossterm::{
    event::{self, KeyCode, KeyEventKind}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    },
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, prelude::{CrosstermBackend,Terminal}, widgets::{Block, Borders, Paragraph}, Frame
};
use std::{io::{self, Stdout}};
use std::error::Error;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}
fn menu(frame: &mut Frame, rect: Rect) {
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Menu"),
        rect,
    );
}
fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title"),
        main_layout[0],
    );
    let inner_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Max(30),
            Constraint::Length(1),
            Constraint::Min(30),
        ]
    )
    .split(main_layout[1]);
    menu(frame, inner_layout[0]);
    frame.render_widget(
        Paragraph::new("Hello World")
        .block(Block::default().title("Greeting").borders(Borders::ALL)),
         inner_layout[2],
        );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Bottom"),
        main_layout[2],
    );
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('q')
            {
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }
    Ok(())
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
