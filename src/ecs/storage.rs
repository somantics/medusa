
use crate::error::core::Result;

pub struct ComponentStorage {
    entity_count: usize,
    components: Vec<Box<dyn ComponentVec>>
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self { entity_count: 0, components: vec![] }
    }

    pub fn new_entity(&mut self) -> usize {
        for vec in self.components.iter_mut() {
            vec.push_none();
        }

        let entity_id = self.entity_count;
        self.entity_count += 1;
        entity_id
    }

    pub fn remove_entity(&mut self, entity: usize) -> Result<()> {
        if entity >= self.entity_count {
            Err("Entity id out of bounds")?
        }

        for vec in &mut self.components {
            vec.set_none(entity);
        }

        // debug!("Removed entity {entity}");
        Ok(())
    }

    pub fn add_component<ComponentType: 'static + Component>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) -> Result<()> {
        if entity >= self.entity_count {
            Err("Entity id out of bounds")?
        }

        for component_vec in self.components.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = Some(component);
                return Ok(());
            }
        }

        self.add_new_component_type::<ComponentType>(Some((entity, component)))?;

        Ok(())
    }

    pub fn remove_component<ComponentType: 'static + Component>(&mut self, entity: usize) -> Result<()> {
        if entity >= self.entity_count {
            Err("Entity id out of bounds")?
        }

        for component_vec in self.components.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = None;
                return Ok(());
            }
        }

        Err("Tried to remove unknown component type".into())
    }

    fn add_new_component_type<ComponentType: 'static + Component>(
        &mut self,
        new_component: Option<(usize, ComponentType)>,
    ) -> Result<()> {
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entity_count);

        for _ in 0..self.entity_count {
            new_component_vec.push_none();
        }

        if let Some((index, component)) = new_component {
            if index >= self.entity_count {
                Err("Entity id out of bounds")?
            }

            new_component_vec[index] = Some(component);
        }
        self.components.push(Box::new(new_component_vec));
        Ok(())
    }

    pub fn borrow_component_vec<ComponentType: 'static + Component>(
        &self,
    ) -> Option<&Vec<Option<ComponentType>>> {
        self.components
            .iter()
            .find_map(|vec| vec.as_any().downcast_ref::<Vec<Option<ComponentType>>>())
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static + Component>(
        &mut self,
    ) -> Option<&mut Vec<Option<ComponentType>>> {
        self.components
            .iter_mut()
            .find_map(|vec| vec.as_any_mut().downcast_mut::<Vec<Option<ComponentType>>>())
    }

    pub fn borrow_entity_component<ComponentType: 'static + Component>(
        &self,
        entity: usize,
    ) -> Option<&ComponentType> {
        let Some(component_vec) = self.borrow_component_vec::<ComponentType>() else {
            return None;
        };

        component_vec[entity].as_ref()
    }

    pub fn borrow_entity_component_mut<ComponentType: 'static + Component>(
        &mut self,
        entity: usize,
    ) -> Option<&mut ComponentType> {
        let Some(component_vec) = self.borrow_component_vec_mut::<ComponentType>() else {
            return None;
        };

        component_vec[entity].as_mut()
    }
}

pub trait Component {}

trait ComponentVec {
    fn push_none(&mut self);
    fn set_none(&mut self, entity: usize);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static + Component> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None);
    }

    fn set_none(&mut self, entity: usize) {
        self[entity] = None;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}