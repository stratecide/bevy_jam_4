use bevy::prelude::*;

use crate::GameState;
use crate::game::PauseState;
use crate::my_assets::MyAssets;

use super::resource::*;
use super::component::*;

const SHOP_OPTION_BORDER_NORMAL: Color = Color::rgb(0.3, 0.3, 0.3);
const SHOP_OPTION_BORDER_HOVERED: Color = Color::rgb(1., 1., 1.);

pub fn setup_ui(
    mut commands: Commands,
    assets: Res<MyAssets>,
    //images: Res<Assets<Image>>,
    highscores: Res<HighScores>,
    curent_score: Res<CurrentScoreIndex>,
) {
    commands.spawn((
        MenuNode,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        },
    ))
    .with_children(|parent| {
        // play button
        parent.spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(20.)),
                    margin: UiRect::all(Val::Px(20.)),
                    ..Default::default()
                },
                border_color: SHOP_OPTION_BORDER_NORMAL.into(),
                background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                ..Default::default()
            },
            Interaction::None,
            PlayButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section("Play", TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.,
                        ..Default::default()
                    }),
                    style: Style {
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Label,
            ));
        });

        // highscore table
        parent.spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            Interaction::None,
            PlayButton,
        ))
        .with_children(|parent| {
            if highscores.0.len() > 0 {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section("Your Highscores", TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.,
                            ..Default::default()
                        }),
                        style: Style {
                            margin: UiRect::all(Val::Px(20.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Label,
                ));

                let mut scores = Vec::new();
                for (i, score) in highscores.0.iter().enumerate() {
                    let color = if Some(i) == curent_score.0 {
                        Color::GOLD
                    } else {
                        Color::WHITE
                    };
                    scores.push(TextSection::new(format!("{score:08}\n"), TextStyle {
                        font: assets.font.clone(),
                        font_size: 30.,
                        color,
                        ..Default::default()
                    }));
                }
                parent.spawn((
                    TextBundle {
                        text: Text::from_sections(scores),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Label,
                ));
            }
        });
    });
}

pub fn start_game(
    mut commands: Commands,
    menu_query: Query<Entity, With<MenuNode>>,
    mut button_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<PlayButton>, Without<MenuNode>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<NextState<PauseState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::None => *color = SHOP_OPTION_BORDER_NORMAL.into(),
            Interaction::Hovered => *color = SHOP_OPTION_BORDER_HOVERED.into(),
            Interaction::Pressed => {
                commands.entity(menu_query.single()).despawn_recursive();
                pause_state.set(PauseState::Unpaused);
                game_state.set(GameState::Game);
                return;
            }
        }
    }
}
