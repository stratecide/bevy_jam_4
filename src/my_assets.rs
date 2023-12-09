use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "img/playerShip1_green.png")]
    pub player: Handle<Image>,

    #[asset(path = "img/laserBlue06.png")]
    pub player_bullet: Handle<Image>,
    #[asset(path = "img/laserRed06.png")]
    pub enemy_bullet: Handle<Image>,
    #[asset(path = "img/star2.png")]
    pub bullet_explosion: Handle<Image>,

    #[asset(path = "img/asteroid", collection(typed))]
    pub asteroids: Vec<Handle<Image>>,
    #[asset(path = "img/simple_enemy/enemyRed1.png")]
    pub simple_enemy: Handle<Image>,

    #[asset(path = "img/star_silver.png")]
    pub exp: Handle<Image>,

    #[asset(path = "sound/sfx_laser2.ogg")]
    pub shooting: Handle<AudioSource>,

    #[asset(path = "kenvector_future_thin.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "img/playerLife1_green.png")]
    pub life_icon: Handle<Image>,
}
