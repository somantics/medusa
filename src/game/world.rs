

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
        self.add_core_components(player, Position {x: 10, y: 5}, '@', Some(DisplayLayer::Unit))?;
        self.current_player = player;
        Ok(())
    }

    pub fn spawn_floor(&mut self, pos: Position) -> Result<()> {
        let entity = self.spawn();
        self.add_core_components(entity, pos, '.', None)?;
        Ok(())
    }

    pub fn get_map(&self) -> Option<Vec<(char, Position, DisplayLayer)>> {
        let map = self.components.get_component_iter::<Glyph>()?
            .filter_map(Self::filter_missing_content)
            .filter_map(|(entity, glyph)| {
                Some((
                    glyph.char,
                    *self.components.borrow_entity_component::<Position>(entity)?,
                    glyph.layer,
                ))
            })
            .collect();
        Some(map)
    }

    fn filter_missing_content<T>(item: (Entity, Option<&T>)) -> Option<(Entity, &T)> {
        let (entity, option) = item;
        match option {
            Some(value) => Some((entity, value)),
            None => None,
        }
    }

    fn spawn(&mut self) -> Entity {
        self.components.new_entity()
    }

    fn add_core_components(&mut self,entity: Entity, pos: Position, char: char, layer: Option<DisplayLayer>) -> Result<()> {
        self.components.add_component(entity, pos)?;
        self.components.add_component(entity, Glyph { char, layer: layer.unwrap_or_default() })?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Size { pub x: usize, pub y: usize }

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect { pub pos: Position, pub size: Size }

// Components below
#[derive(Debug, Default, Clone, Copy)]
pub struct Position { pub x: usize, pub y: usize }
impl Component for Position {}

#[derive(Debug, Default, Clone, Copy)]
pub struct Glyph {pub char: char, pub layer: DisplayLayer }
impl Component for Glyph {}


#[derive(Debug, Default, Clone, Copy)]
pub enum DisplayLayer {
    Unit,
    #[default]
    Floor
}