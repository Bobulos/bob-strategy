use bob_engine::runtime::ecs::SystemBase;
use bob_engine::runtime::ecs::DynamicWorld;

use std::sync::Arc;

pub struct Spawner {
    
}

impl SystemBase for Spawner {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {
        
    }
    fn on_update(&mut self, _world: &Arc<DynamicWorld>) {
        
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {
        
    }
}
impl Spawner {
    