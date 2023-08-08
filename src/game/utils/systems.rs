use super::components::*;
use bevy::prelude::*;

pub fn rotate_objects_system(
    mut rotator_query: Query<(&mut RotateOverTime, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut rotate_over_time, mut transform) in rotator_query.iter_mut() {
        rotate_over_time.angle_degree += rotate_over_time.speed_deg * dt;
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotate_over_time.angle_degree.to_radians());
    }
}

pub fn update_mana_each_second(mut mana_query: Query<&mut Mana>, time: Res<Time>) {
    let dt: f32 = time.delta_seconds();
    let current_time = time.elapsed_seconds();
    let previous_time = current_time - dt;

    if current_time.floor() > previous_time.floor() {
        for mut mana in mana_query.iter_mut() {
            mana.current_mana += mana.mana_regen_per_sec;
            mana.current_mana = mana.current_mana.clamp(0, mana.max_mana);
        }
    }
}

pub fn create_sprite_bundle(
    asset: &str,
    position: Vec3,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: position,
            ..default()
        },
        texture: asset_server.load(asset),
        ..default()
    }
}