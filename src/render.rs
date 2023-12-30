pub mod render {
    use tui::{backend::TermionBackend, Terminal};
    use termion::raw::RawTerminal;

    use tui::widgets::Borders;
    use tui::widgets::Paragraph;
    use tui::style::Color;
    use tui::layout::Alignment;
    use tui::style::Style;
    use tui::layout::Constraint;
    use tui::layout::Layout;
    use tui::layout::Direction;
    use tui::widgets::Block;
    use tui::widgets::BorderType;
    use tui::widgets::Row;
    use tui::widgets::Table;

    #[derive(Clone, Copy, Debug)]
    pub enum MenuItem {
        Timetable,
        Settings,
    }


    pub async fn draw(terminal: &mut Terminal<TermionBackend<RawTerminal<std::io::Stdout>>>, timetable: &Vec<Row<'_>>) {
        let tableRows = timetable;


        // Draw our UI onto the terminal
        terminal.draw(|rect| {
            let size = rect.size();
            let table = Table::new(tableRows.clone())
            .header(Row::new(vec!["Line", "Destination", "Departure"])
                    .style(tui::style::Style::default().fg(tui::style::Color::Red).add_modifier(tui::style::Modifier::BOLD)))
            //.block(Block::default().title("Scrollable Table"))
            .widths(&[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
            .column_spacing(1)
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::Magenta).add_modifier(tui::style::Modifier::BOLD))
            .highlight_symbol(">>");
        rect.render_widget(table, size);
        }).unwrap();
    }
}