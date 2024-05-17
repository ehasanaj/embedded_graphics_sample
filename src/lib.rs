use embedded_graphics::{
    geometry::Point,
    prelude::*,
    text::{Alignment, Text},
};
use manager::Manager;
use utils::get_time;

mod manager;
mod renderer;
mod utils;

pub type InfalliableResult = Result<(), std::convert::Infallible>;

pub fn run() -> InfalliableResult {
    let mut manager = Manager::new("Clock", 128, 64);
    manager.run(|manager: &mut Manager| draw_timer(manager))?;
    Ok(())
}

fn draw_timer(manager: &mut Manager) -> InfalliableResult {
    let time = get_time();
    let position = manager.bounding_box().center() + Point::new(0, 3);
    Text::with_alignment(
        &time,
        position,
        manager.character_style(),
        Alignment::Center,
    )
    .draw(manager.display())?;
    Ok(())
}