use bevy::prelude::*;
use game::GamePlugin;

pub mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
       // .add_plugin(RustGpuPlugin)
       // .add_plugin(RustGpuMaterialPlugin::<MyRustGpuMaterial>::default())
       // .add_startup_system(setup)
        .add_plugin(GamePlugin)
        .run();
}

/*
fn setup(materials: ResMut<Assets<RustGpu<MyRustGpuMaterial>>>) {
    let shader = asset_server.load::<RustGpuBuilderOutput>(SHADER_PATH);

    // Add it to a RustGpu material, which can be used with bevy's MaterialMeshBundle
    let material = materials.add(RustGpu {
        vertex_shader: Some(shader),
        fragment_shader: Some(shader),
        ..default()
    });

    // Create cube mesh
    let mesh = meshes.add(Cube { size: 1.0 }.into());
    
    // Spawn a mesh with our rust-gpu material
    commands.spawn(MaterialMeshBundle {
        mesh,
        material,
        ..default()
    });
}
*/