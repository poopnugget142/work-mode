use chrono::Utc;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, Borders, Paragraph}, Frame
};

use crate::app::{App, WorkStatus};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let status_text = match app.status {
        WorkStatus::Check => Text::styled(
            "Press Space to Begin System Detox",
            Style::default().fg(Color::Red),
        ),
        WorkStatus::Start => Text::styled(
            "Press Space to Begin Working!",
            Style::default().fg(Color::Yellow),
        ),
        WorkStatus::Working => Text::styled(
            "Currently Working...",
            Style::default().fg(Color::Green),
        ),
        WorkStatus::Break => Text::styled(
            "On break.....",
            Style::default().fg(Color::Red),
        ),
        WorkStatus::Complete => Text::styled(
            "YOUR WORK IS COMPLETED FOR TODAY CONGRATS!",
            Style::default().fg(Color::Green).bold(),
        ),
    };

    let title = Paragraph::new(status_text)
    .alignment(Alignment::Center)
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let timer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let timer_text = match app.time_started {
        Some(time_started) => {
            let time_passed = (Utc::now()-time_started).to_std().unwrap();
            let time_remaining = app.work_time.checked_sub(time_passed);

            match time_remaining {
                Some(time_remaining) => {
                    let seconds = time_remaining.as_secs() % 60;
                    let minutes = (time_remaining.as_secs() / 60) % 60;
                    let hours = (time_remaining.as_secs() / 60) / 60;

                    Span::styled(
                        format!("{}:{}:{}", hours, minutes, seconds),
                        Style::default().fg(Color::Green),
                    )
                },
                None => {
                    Span::styled(
                        "Timer finished!",
                        Style::default().fg(Color::Green),
                    )
                },
            }
        }
        None => {
            Span::styled(
                "Time has not started yet",
                Style::default().fg(Color::Red),
            )
        }
    };

    let text = vec![
        Line::from(vec![
            Span::raw("Time Remaining"),
        ]),
        Line::from(vec![
            timer_text,
        ]),
    ];

    let timer = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(timer_block);

    f.render_widget(timer, chunks[1]);
}