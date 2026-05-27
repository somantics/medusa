use crate::ui::app::App;


pub mod ui;
pub mod ecs;
pub mod error;
pub mod game;

fn main() -> std::io::Result<()> {
    
    ratatui::run(|terminal| App::new().run(terminal))
}






