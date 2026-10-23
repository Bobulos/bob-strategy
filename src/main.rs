use bob_engine::runtime::assets::AssetStore;
use bob_engine::runtime::Engine;
use bob_engine::runtime::ecs::{SystemGroup, system_group};
use bob_engine::*;

pub mod systems;
pub use systems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bob_engine running...");
    let event_loop = EventLoop::new()?;
    
    // Bootstrap
    let mut engine = Engine::new();
    bootstrap_gameplay_systems(&mut engine);
    // Do this last
    let mut app = App::new(engine, AppContext::new((800,600), false), build_asset_store());
    event_loop.run_app(&mut app)?;
    Ok(())
}
const GAMEPLAY_LOGIC: &str = "gameplay_logic";
fn bootstrap_gameplay_systems(engine: &mut Engine) {
    let world = engine.entities.get_world(bob_engine::constants::MAIN_WORLD).unwrap();
    let group = engine.entities.add_system_group(
        GAMEPLAY_LOGIC,
        SystemGroup::new(&world, system_group::SystemGroupThreading::Main
    )).unwrap();
    
    //let group = engine.entities.get_system_group_mut(GAMEPLAY_LOGIC).unwrap();
    
}
fn build_asset_store() -> AssetStore {
    let mut a: AssetStore = AssetStore::new();
    include_asset!(&mut a, "soldier.png");
    include_asset!(&mut a, "font.png");
    include_asset!(&mut a, "grass.png");
    //include_asset!(&mut a, "test")
    a
}