

use crate::ecs::storage::{Component, ComponentStorage, Entity};
use crate::error::core::Result;

pub struct World {
    size: Size,
    current_player: Entity,
    components: ComponentStorage,
}

impl World {
    pub fn spawn_player(&mut self) -> Result<()> {
        let player = self.spawn();
        self.add_core_components(player, Position {x: 10, y: 5}, '@')?;
        self.current_player = player;
        Ok(())
    }

    pub fn spawn_floor(&mut self, pos: Position) -> Result<()> {
        let entity = self.spawn();
        self.add_core_components(entity, pos, '.')?;
        Ok(())
    }

    fn spawn(&mut self) -> Entity {
        self.components.new_entity()
    }

    fn add_core_components(&mut self,entity: Entity, pos: Position, char: char) -> Result<()> {
        self.components.add_component(entity, pos)?;
        self.components.add_component(entity, Glyph { char })?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Size { pub x: usize, pub y: usize }

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect { pub pos: Position, pub size: Size }


#[derive(Debug, Default, Clone, Copy)]
pub struct Position { pub x: usize, pub y: usize }
impl Component for Position {}

#[derive(Debug, Default, Clone, Copy)]
pub struct Glyph {pub char: char }
impl Component for Glyph {}