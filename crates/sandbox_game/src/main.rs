use engine_core::{
    Cell, CellPos, ChunkPos, MaterialRegistry, SimulationConfig, World, simulate_world,
};

fn main() {
    let registry = MaterialRegistry::core();
    let sand = registry.id("core:sand").expect("core sand material");
    let water = registry.id("core:water").expect("core water material");
    let mut world = World::new(1234, registry);

    for x in -8..=8 {
        world.set_cell(CellPos { x, y: 20 }, Cell::new(sand));
    }
    for x in -3..=3 {
        world.set_cell(CellPos { x, y: 26 }, Cell::liquid(water, 255));
    }

    for _ in 0..12 {
        let stats = simulate_world(
            &mut world,
            SimulationConfig {
                min_chunk: ChunkPos { x: -1, y: -1 },
                max_chunk: ChunkPos { x: 1, y: 1 },
            },
        );
        println!("tick={} moved={}", world.tick, stats.moved_cells);
    }
}
