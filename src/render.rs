pub mod render {
    use std::io;

    use ratatui::prelude::*;
    use ratatui::widgets::*;

    // #[derive(Clone, Copy, Debug)]
    // pub enum MenuItem {
    //     Timetable,
    //     Settings,
    // }

    pub fn gui(rect: &mut Frame, timetable: &Vec<Row<'_>>, metadata: &Vec<String>) {
        // Divide our vertical layout into chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(10), // Header (Station info, time)
                    Constraint::Percentage(90), // Body (timetable itself)
                ]
                .as_ref()
            )
            .split(rect.size());


        let header = Paragraph::new(metadata[0].to_string())
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Green))
                    .border_type(BorderType::Plain)
            );
        rect.render_widget(header, chunks[0]);


        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(60),
            Constraint::Percentage(30),
        ];
        let table = Table::new(timetable.clone(), widths)
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1)
            // You can set the style of the entire Table.
            .style(Style::new().blue())
            // It has an optional header, which is simply a Row always visible at the top.
            .header(
                Row::new(vec!["Line", "Destination", "Departure"])
                    .style(Style::new().bold().fg(Color::Red))
                    // To add space between the header and the rest of the rows, specify the margin
                    .bottom_margin(1),
            )
            // As any other widget, a Table can be wrapped in a Block.
            .block(Block::default().title("Table"))
            // The selected row and its content can also be styled.
            .highlight_style(Style::new().reversed())
            // ...and potentially show a symbol in front of the selection.
            .highlight_symbol(">>");
        rect.render_widget(table, chunks[1]);
    }


    pub fn draw(terminal: &mut ratatui::prelude::Terminal<CrosstermBackend<io::Stdout>>, timetable: &Vec<Row<'_>>, metadata: &Vec<String>) {
        // Draw our UI onto the terminal
        terminal.draw(|rect| {
            gui(rect, timetable, metadata);
        }).unwrap();
    }
}