use std::slice::Iter;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{List, ListItem, Paragraph},
};

use crate::{
    handlers::{app::App, config::CompleteConfig},
    ui::components::dashboard::DASHBOARD_TITLE,
    utils::styles::DASHBOARD_TITLE_COLOR,
};

const FIRST_N_CHANNELS: std::ops::Range<u32> = 0..5;

fn render_dashboard_title_widget<T: Backend>(frame: &mut Frame<T>, v_chunks: &mut Iter<Rect>) {
    let w = Paragraph::new(
        DASHBOARD_TITLE
            .iter()
            .map(|&s| Spans::from(vec![Span::raw(s)]))
            .collect::<Vec<Spans>>(),
    )
    .style(DASHBOARD_TITLE_COLOR);

    frame.render_widget(w, *v_chunks.next().unwrap());
}

fn render_channel_selection_widget<T: Backend>(
    frame: &mut Frame<T>,
    v_chunks: &mut Iter<Rect>,
    app: &App,
    current_channel: String,
) {
    let current_channel_title =
        Paragraph::new(Spans::from(vec![Span::raw("Current config channel")]));

    frame.render_widget(current_channel_title, *v_chunks.next().unwrap());

    let current_channel_selection = Paragraph::new(Spans::from(vec![
        Span::raw("["),
        Span::styled(
            "ENTER".to_string(),
            Style::default().fg(Color::LightMagenta),
        ),
        Span::raw("] "),
        Span::raw(current_channel),
    ]));

    frame.render_widget(current_channel_selection, *v_chunks.next().unwrap());

    let channel_selection_title =
        Paragraph::new(Spans::from(vec![Span::raw("Most recent channels")]));

    frame.render_widget(channel_selection_title, *v_chunks.next().unwrap());

    let items_binding = app.storage.get_last_n("channels", 5, true);

    let items = items_binding
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if FIRST_N_CHANNELS.contains(&(i as u32)) {
                Some(ListItem::new(Spans::from(vec![
                    Span::raw("["),
                    Span::styled(i.to_string(), Style::default().fg(Color::LightMagenta)),
                    Span::raw("] "),
                    Span::raw(s),
                ])))
            } else {
                None
            }
        })
        .collect::<Vec<ListItem>>();

    let channel_options = List::new(items)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

    frame.render_widget(channel_options, *v_chunks.next().unwrap());
}

fn render_quit_selection_widget<T: Backend>(frame: &mut Frame<T>, v_chunks: &mut Iter<Rect>) {
    let quit_option = Paragraph::new(Spans::from(vec![
        Span::raw("["),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw("] "),
        Span::raw("Quit"),
    ]));

    frame.render_widget(quit_option, *v_chunks.next().unwrap());
}

pub fn render_dashboard_ui<T: Backend>(
    frame: &mut Frame<T>,
    app: &mut App,
    config: &CompleteConfig,
) {
    let v_chunk_binding = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(DASHBOARD_TITLE.len() as u16 + 1),
            Constraint::Length(2),
            Constraint::Min(2),
            Constraint::Length(2),
            Constraint::Min(6),
            Constraint::Min(1),
        ])
        .margin(2)
        .split(frame.size());

    let mut v_chunks = v_chunk_binding.iter();

    render_dashboard_title_widget(frame, &mut v_chunks);

    render_channel_selection_widget(frame, &mut v_chunks, app, config.twitch.channel.clone());

    render_quit_selection_widget(frame, &mut v_chunks);
}