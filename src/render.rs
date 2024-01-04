pub mod render {
    use std::io;

    use ratatui::prelude::*;
    use ratatui::widgets::*;


    fn gui(rect: &mut Frame, metadata: &String, chunks: &std::rc::Rc<[Rect]>) {

        let header = Paragraph::new(metadata.as_str())
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::LightYellow))
                    .border_type(BorderType::Plain)
            );
        rect.render_widget(header, chunks[0]);
    }

    pub fn departure_table(rect: &mut Frame, timetable: &Vec<Row<'_>>, chunks: &std::rc::Rc<[Rect]>) {
        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(60),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ];
        let table = Table::new(timetable.clone(), widths)
            .column_spacing(1)
            .style(Style::default().fg(Color::White))
            .header(
                Row::new(vec!["Line", "Destination", "Departure", "Delay"])
                    .style(Style::new().fg(Color::Magenta))
                    .bottom_margin(1),
            )
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");
        rect.render_widget(table, chunks[1]);
    }


    pub fn draw(terminal: &mut ratatui::prelude::Terminal<CrosstermBackend<io::Stdout>>, timetable: &Vec<Row<'_>>, metadata: &String) {
        // Draw our UI onto the terminal
        terminal.draw(|rect| {
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

            // Start the drawing of the GUI
            gui(rect, &metadata, &chunks);
            // Draw the table within the GUI
            departure_table(rect, timetable, &chunks);
        }).unwrap();
    }
}