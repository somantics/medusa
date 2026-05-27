
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::{Line, Span, Text, ToSpan}, widgets::{Block, Paragraph, Widget}
};

use crate::game::world::{Size, World};



#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    world: World,
}

impl App {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.world.size = Size {x: 55, y: 10};
        self.world.spawn_test_room();
        self.world.spawn_player();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
       match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            self.handle_key_event(key_event)
        }
        _ => {}
       };
       Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            //KeyCode::Left => self.decrement_counter(),
            //KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Medusa ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        let mut text = Text::raw("Map goes here");
        if let Some(game_map) = self.world.get_map() {
            let mut span_vecs = vec![vec![Span::raw(" "); self.world.size.x]; self.world.size.y];
            for (char, pos, _) in game_map {
                span_vecs[pos.y][pos.x] = Span::raw(String::from(char));
            }

            text = Text::from(span_vecs
                .into_iter()
                .map(|l| Line::from(l))
                .collect::<Vec<_>>());

        }


        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

