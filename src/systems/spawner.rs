use bob_engine::runtime::assets::AssetHandle;
use bob_engine::runtime::ecs::SystemBase;
use bob_engine::runtime::ecs::DynamicWorld;
use bob_engine::runtime::ecs::core_components::Transform;
use bob_engine::runtime::math::Float2;
use bob_engine::runtime::rendering::BatchHandle;
use bob_engine::runtime::rendering::PipelineKey;
use bob_engine::runtime::rendering::sprite_rendering::components::Sprite;

use std::sync::Arc;

pub struct Spawner {
    
}

impl SystemBase for Spawner {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        let entity = world.create_entity();
        world.add_component_safe(entity, Transform::new(Float2::ZERO, 0.0));
        world.add_component_safe(entity, BatchHandle::new(AssetHandle::new(0, None), PipelineKey::Sprite));
        world.add_component_safe(entity, Sprite::new(true, [0.0, 0.0], [1.0, 1.0]));
    }
    fn on_update(&mut self, _world: &Arc<DynamicWorld>) {
        
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {
        
    }
}
impl Spawner {
    pub fn new() -> Self { Self {} }
}
    