use core::time;

use chrono::{Datelike, Local, TimeDelta, Weekday};
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
        WorkStatus::Weekend => Text::styled(
            "WEEKEND TODAY ENJOY BREAK",
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
            let time_passed = (Local::now()-time_started).to_std().unwrap();
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
                "0:0:0",
                Style::default().fg(Color::Red),
            )
        }
    };

    let weekday = Local::now().weekday();
    let (weekday_span, weekday_span2) = match weekday {
        Weekday::Sun | Weekday::Sat => (
            Span::styled(
                weekday.to_string(),
                Style::default().fg(Color::Green)
            ),
            Span::styled(
                "Weekend",
                Style::default().fg(Color::Green)
            ),
        ),
        _ => (
            Span::styled(
                weekday.to_string(),
                Style::default().fg(Color::Red)
            ),
            Span::styled(
                format!("Days Until Weekend: {}", 5-weekday.num_days_from_sunday()),
                Style::default().fg(Color::Red)
            ),
        )
    };

    let mut text = vec![
        Line::from(vec![
            weekday_span,
        ]),
        Line::from(vec![
            weekday_span2,
        ]),
        Line::from(vec![
            Span::raw(format!("{}", app.start_time)),
        ]),
    ];

    if app.status != WorkStatus::Complete && app.status != WorkStatus::Weekend {
        text.insert(0, Line::from(vec![
            timer_text,
        ]));
    }

    match app.time_started {
        Some(time_started) => {
            let latetime = (time_started-Local::now().with_time(app.start_time).unwrap()).num_minutes();
            if latetime > 0 {
                text.push(Line::from(vec![
                    Span::styled(
                        format!(" You were {} minutes late for work today ... do better next time!", latetime),
                        Style::default().fg(Color::Red)
                    )
                ]));
            } else {
                text.push(Line::from(vec![
                    Span::styled(
                        format!(" You were {} minutes early for work today congrats!", latetime),
                        Style::default().fg(Color::Red)
                    )
                ]));
            }
        }
        None => {}
    }

    let timer = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(timer_block);

    f.render_widget(timer, chunks[1]);
}