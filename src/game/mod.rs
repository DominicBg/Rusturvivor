use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

use health::HealthPlugin;
use hero::HeroPlugin;
use minions::MinionPlugin;
use spells::SpellPlugin;
use utils::UtilPlugin;

use self::health::components::*;

pub mod health;
pub mod hero;
pub mod minions;
pub mod spells;
pub mod utils;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_camera))
            .insert_resource(ClearColor(Color::hex("#bba1ff").unwrap()))
            .add_event::<DamageEvent>()
            .add_event::<KillEvent>()
            .add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_plugins((
                SpellPlugin,
                HealthPlugin,
                UtilPlugin,
                HeroPlugin,
                MinionPlugin,
            ));
    }
}
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.1),
        projection: OrthographicProjection {
            scale: 0.2,
            ..default()
        },

        ..default()
    });
}



pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {
            color: Color::BLUE,
            //color_texture: Some(asset_server.load("branding/icon.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
   // #[texture(1)]
   // #[sampler(2)]
   // color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

