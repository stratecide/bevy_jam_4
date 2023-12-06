use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "img/playerShip1_green.png")]
    pub player: Handle<Image>,
    #[asset(path = "img/laserBlue06.png")]
    pub player_bullet: Handle<Image>,
    #[asset(path = "img/star2.png")]
    pub bullet_explosion: Handle<Image>,
    #[asset(path = "img/asteroid", collection(typed))]
    pub asteroids: Vec<Handle<Image>>,
    #[asset(path = "sound/sfx_laser2.ogg")]
    pub shooting: Handle<AudioSource>,
}
