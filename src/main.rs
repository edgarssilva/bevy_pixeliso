mod animation;
mod controller;
mod follow;
mod helper;
mod stats;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use animation::*;
use controller::*;
use follow::*;
use helper::*;
use stats::*;
pub struct KeyMaps {
    walk_up: KeyCode,
    walk_left: KeyCode,
    walk_down: KeyCode,
    walk_right: KeyCode,
}

impl Default for KeyMaps {
    fn default() -> Self {
        KeyMaps {
            walk_up: KeyCode::W,
            walk_left: KeyCode::A,
            walk_down: KeyCode::S,
            walk_right: KeyCode::D,
        }
    }
}

pub const PLAYER_Z: f32 = 39.;
pub const MAP_Z: f32 = 36.;
pub const BACKGROUND_Z: f32 = 1.;

/* #[derive(PhysicsLayer)]
enum CollisionLayer {
    Player,
    PlayerWeapon,
    Enemy,
} */

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(48. / 255., 44. / 255., 46. / 255.)))
        .insert_resource(KeyMaps::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_system(set_texture_filters_to_nearest.system())
        .add_system(helper_camera_controller.system())
        .add_system(sprite_animation.system())
        .add_system(player_controller.system())
        .add_system(follow_entity_system.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_on_camera_move.system())
                .with_system(parallax_system.system()),
        )
        .add_system(shake_system.system())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //Map Creation
    let map_id = commands.spawn().id();
    let map_handle = asset_server.load("map.tmx");

    commands.entity(map_id).insert_bundle(TiledMapBundle {
        tiled_map: map_handle,
        map: Map::new(0u16, map_id),
        transform: Transform::from_xyz(0.0, 40.0, MAP_Z), //TODO: Find a way to center the map
        ..Default::default()
    });

    //Player Creation
    let player_size = Vec2::new(16., 17.);

    //Load the textures
    let texture_handle = asset_server.load("IsometricTRPGAssetPack_Entities.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, player_size, 4, 33);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_size = player_size / 2.; //Player actual size is half of his sprite (whitespace)

    let player_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0., 0., PLAYER_Z),
            ..Default::default()
        })
        .insert(PlayerControlled)
        .insert(Timer::from_seconds(0.1, true))
        .insert(Stats::new(100, 20, 50))
        .with_children(|children| {
            let offset = player_size.x;
            let half_width = player_size.x / 3.;

            let positions = [
                Vec2::Y * offset,
                Vec2::Y * -offset,
                Vec2::X * -offset,
                Vec2::X * offset,
            ];

            //Add attack sensors
            for pos in positions {
                children.spawn();
            }
        })
        .id();

    //Add enemy
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(asset_server.load("old/char/iddle_l1.png").into()),
            transform: Transform {
                translation: Vec3::new(30., 5., PLAYER_Z),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Stats::new(100, 20, 50));

    //Add Camera after so we can give it the player entity
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 0.15;
    commands
        .spawn_bundle(camera_bundle)
        .insert(Follow {
            target: FollowTarget::Entity(player_entity),
            speed: 5.,
        })
       /*  .insert(Shake {
            strength: 15.,
            duration: 10.,
        }) */;

    //Add parallax planet
    commands
        .spawn()
        .insert(Transform::from_xyz(-75., 30., 0.))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                        asset_server.load("earth2.png"),
                        Vec2::splat(100.),
                        50,
                        50,
                    )),
                    transform: Transform {
                        translation: Vec3::new(0., 0., BACKGROUND_Z + 20.),
                        scale: Vec3::new(0.5, 0.5, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Timer::from_seconds(0.050, true))
                .insert(Parallax);
        });

    //Add space layers with parallax
    let names = vec![
        "background_4.png",
        "background_3.png",
        "background_2.png",
        "background_1.png",
    ];

    for i in 1..5 {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(asset_server.load(names[i - 1]).into()),
                transform: Transform::from_xyz(0., 0., BACKGROUND_Z + (i * 10) as f32),
                ..Default::default()
            })
            .insert(Parallax);
    }
}
