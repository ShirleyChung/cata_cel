use crossterm::{
    event::{self, KeyCode, KeyEventKind}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    },
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize,Terminal},
    widgets::{Block, Borders, Paragraph}, Frame,
};
use std::io::{self, Stdout};
use std::error::Error;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World")
        .block(Block::default().title("Greeting").borders(Borders::ALL)),
         frame.size(),
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
