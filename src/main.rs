use std::{error::Error, fs::{self, OpenOptions}, io::{self, Write}, str::FromStr};

use app::WorkStatus;
use chrono::{DateTime, Local, Utc};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend}, Terminal
};

mod app;
mod ui;

use crate::{
    app::App,
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.status {
                WorkStatus::Check => {
                    match key.code {
                        KeyCode::Char(' ') => {
                            let mut file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .open("/etc/hosts")
                                .unwrap();

                            // Create gap so that so issues happen on same line
                            let _ = writeln!(file, "");
                            for domain in &app.settings.blocked_sites {
                                let _ = writeln!(file, "127.0.0.1  {}", domain);
                            }

                            app.status = WorkStatus::Start;

                            app.save.detox = true;
                            app.save()
                        }
                        _ => {}
                    }
                }
                WorkStatus::Start => {
                    match key.code {
                        KeyCode::Char(' ') => {
                            match &app.save.time_started {
                                Some(time_started) => {
                                    app.time_started = Some(DateTime::<Local>::from_str(&time_started).expect("Unable to parse string"));
                                }
                                None => {
                                    app.time_started = Some(Local::now());
                                    app.save.time_started = Some(Local::now().to_string());
                                    app.save();
                                }
                            }
                            app.status = WorkStatus::Working;
                        }
                        _ => {}
                    }
                }
                WorkStatus::Working => {
                    // Check if work complete
                    match app.time_started {
                        Some(time_started) => {
                            let time_passed = (Local::now()-time_started).to_std().unwrap();
                            let time_remaining = app.work_time.checked_sub(time_passed);
            
                            // Time remaining when out of bounds!
                            if time_remaining == None {
                                let data = fs::read_to_string("hosts.backup").expect("Unable to read file");
                                fs::write("/etc/hosts", data).expect("Unable to write file");
            
                                app.save.detox = false;
                                app.save.time_started = None;
                                app.save.last_completion = Some(Utc::now().to_string());
                                app.save();
            
                                app.status = WorkStatus::Complete;
                            }
                        }
                        None => {},
                    };
                }

                _ => {}
            }
        }
    }
}