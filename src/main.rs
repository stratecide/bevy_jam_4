#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod game;
mod my_assets;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{MaterialMesh2dBundle, Material2d, Material2dPlugin};
use bevy_asset_loader::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
        // state
        .add_state::<GameState>()
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Game))
        .add_collection_to_loading_state::<_, my_assets::MyAssets>(GameState::Loading)
        .add_systems(OnEnter(GameState::Game), (
            spawn_background,
        ))
        .add_systems(Update, update_background)
        .run();
}

fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        mesh: meshes.add(shape::Quad::new(Vec2::splat(8000.)).into()).into(),
        material: materials.add(BackgroundMaterial {
            offset: Vec4::splat(0.),
        }),
        ..Default::default()
    });
}

fn update_background(
    mut background_query: Query<(&mut Transform, &Handle<BackgroundMaterial>)>,
    camera_query: Query<&Transform, (With<Camera>, Without<Handle<BackgroundMaterial>>)>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let camera = match camera_query.get_single() {
        Ok(c) => c,
        _ => return,
    };
    for (mut background, material) in background_query.iter_mut() {
        background.translation.x = camera.translation.x;
        background.translation.y = camera.translation.y;
        if let Some(material) = materials.get_mut(material) {
            material.offset.x = camera.translation.x;
            material.offset.y = -camera.translation.y;
        }
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub offset: Vec4,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}