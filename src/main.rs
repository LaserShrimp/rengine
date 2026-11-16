use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize, Color},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    widgets::canvas::{Canvas, Map, MapResolution, Points, Rectangle},
    DefaultTerminal, Frame,
};
use color_eyre::Result;
use crate::vectors::vectors::*;
use crate::curves::curves::*;

pub mod vectors;
pub mod curves;

#[derive(Default, Debug)]
pub struct App {
    points_array: Vec<Vector3D>,
    camera_position: Vector3D,
    phi: f64,
    exit: bool
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.phi+= 0.1;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // frame.render_widget(self, frame.area());
        let canvas = Canvas::default()
            .block(Block::bordered().title("canvas ?"))
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                for point in self.points_array.iter() {
                    let current_coo = find_coordinates_on_screen_from_two_vectors(&self.camera_position, point, 0.0);
                    ctx.draw(&Points{
                        coords: &[(current_coo.x, current_coo.y),],
                        color: Color::Red,
                    });
                }
  
                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[0], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[3], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });

                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[1], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[4], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });

                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[2], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[5], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });

                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[0], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[1], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });
                
                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[1], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[2], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });

                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[2], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[0], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });

                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[3], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[4], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });

                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[4], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[5], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });

                let v_1 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[5], 0.0);
                let v_2 = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &self.points_array[3], 0.0);
                ctx.draw(&ratatui::widgets::canvas::Line{
                    x1: v_1.x,
                    y1: v_1.y,
                    x2: v_2.x,
                    y2: v_2.y,
                    color: Color::Green,
                });
                let mut t = -100.0;
                while t <= 100.0 {
                    let vec = spiral(t, self.phi);
                    let mut vec = find_coordinates_on_screen_from_two_vectors(&self.camera_position, &vec, 0.0);
                    vec.z+= 50.0;
                    ctx.draw(&Points{
                        coords: &[(vec.x, vec.y),],
                        color: Color::Blue
                    });
                    t+= 0.01;
                }
            });

        frame.render_widget(canvas, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.camera_position.x-= 2.0,
            KeyCode::Right => self.camera_position.x+= 2.0,
            KeyCode::Up => self.camera_position.y+= 2.0,
            KeyCode::Down => self.camera_position.y-= 2.0,
            KeyCode::Char('a') => self.camera_position.z+= 2.0,
            KeyCode::Char('z') => self.camera_position.z-= 2.0,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }


}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("3D renderer what a f#cking style".bold());
        let instructions = Line::from(vec![
            " Step forward ".into(),
            "^".blue().bold(),
            " Step backwaard ".into(),
            "v".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Paragraph::new("hey")
            .centered()
            .block(block)
            .render(area, buf); 
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::default();
    app.camera_position.x = 10.0;
    app.camera_position.y = 10.0;
    app.camera_position.z = -10.0;
 
    for i in 1..6 {
        app.points_array.push(Vector3D{x: 0.0, y: 10.0, z: 10.0*(i as f64)});
        app.points_array.push(Vector3D{x: 60.0, y: 10.0, z: 10.0*(i as f64)});
        app.points_array.push(Vector3D{x: 30.0, y: 60.0, z: 10.0*(i as f64)}); 
    }

    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
