use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create the Initial App Layout
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_footer(f, app, chunks[2]);

    match app.tabs.index {
        0 => profiles_table(f, app, chunks[1]),
        _ => {}
    };
}

pub fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Max(40)].as_ref())
        .split(area);

    draw_tabs(f, app, chunks[0]);
    draw_search(f, app, chunks[1])
}

pub fn draw_footer<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let footer =
        Paragraph::new("").block(Block::default().borders(Borders::ALL).title("Shortcuts"));

    f.render_widget(footer, area)
}

pub fn draw_tabs<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::LightBlue))
        .select(app.tabs.index);

    f.render_widget(tabs, area);
}

pub fn draw_search<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let search = Paragraph::new("").block(Block::default().borders(Borders::ALL).title("Search"));

    f.render_widget(search, area)
}

fn profiles_table<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Styling
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let header_value_style = Style::default().fg(Color::Yellow);
    let header_row_style = Style::default().add_modifier(Modifier::BOLD);

    // Table Setup
    let header_cells = ["ID", "Name", "Status", "Server Address", "Client Address"]
        .iter()
        .map(|h| Cell::from(*h).style(header_value_style));
    let header = Row::new(header_cells)
        .height(1)
        .bottom_margin(1)
        .style(header_row_style);

    // Table Values
    let rows = app.profiles.items.iter().map(|p| {
        Row::new(vec![
            p.id,
            p.name,
            p.status,
            p.server_address,
            p.client_address,
        ])
    });

    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Pritunl Profiles"),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Ratio(2, 8),
            Constraint::Ratio(2, 8),
            Constraint::Ratio(1, 8),
            Constraint::Ratio(1, 8),
            Constraint::Ratio(1, 8),
        ]);

    f.render_stateful_widget(table, area, &mut app.profiles.state);
}
