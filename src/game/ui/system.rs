use bevy::prelude::*;

use crate::game::player::component::Upgrade;
use crate::game::player::resource::Upgrades;
use crate::my_assets::MyAssets;
use crate::game::{resource::*, PauseState};

use super::component::*;

const SHOP_OPTION_BORDER_NORMAL: Color = Color::rgb(0.3, 0.3, 0.3);
const SHOP_OPTION_BORDER_HOVERED: Color = Color::rgb(1., 1., 1.);

pub fn setup_ui(
    mut commands: Commands,
    assets: Res<MyAssets>,
    images: Res<Assets<Image>>,
) {
    commands.spawn((
        GuiParent,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        }
    ))
    .with_children(|parent| {
        // top ui
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(8.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // lives
            parent.spawn((
                NodeBundle {
                    style: Style {
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                let life_icon = images.get(&assets.life_icon).unwrap();
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(life_icon.width() as f32),
                            height: Val::Px(life_icon.height() as f32),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    },
                    UiImage::new(assets.life_icon.clone()),
                ));
                parent.spawn((
                    TextBundle {
                        text: Text::from_section("3", TextStyle {
                            font: assets.font.clone(),
                            font_size: 25.,
                            ..Default::default()
                        }),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Label,
                    LifeCounter,
                ));
            });

            // wave timer
            parent.spawn((
                NodeBundle {
                    style: Style {
                        min_width: Val::Px(120.),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle {
                        text: Text::from_sections([
                            TextSection::new("", TextStyle {
                                font: assets.font.clone(),
                                font_size: 25.,
                                ..Default::default()
                            }),
                            TextSection::new("", TextStyle {
                                font: assets.font.clone(),
                                font_size: 16.,
                                ..Default::default()
                            }),
                        ]),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Label,
                    WaveTimerUi,
                ));
            });

            // score
            parent.spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section("Score: ", TextStyle {
                            font: assets.font.clone(),
                            font_size: 16.,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        }),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Label,
                ));
                parent.spawn((
                    TextBundle {
                        text: Text::from_section("", TextStyle {
                            font: assets.font.clone(),
                            font_size: 25.,
                            ..Default::default()
                        }),
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Label,
                    ScoreCounter,
                ));
            });
        });

        // bottom ui
        parent.spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            let exp_margin= 10.;
            let exp_text = Text::from_section("EXP", TextStyle {
                font: assets.font.clone(),
                font_size: 30.,
                ..Default::default()
            });
            parent.spawn((
                TextBundle {
                    text: exp_text.clone(),
                    style: Style {
                        margin: UiRect::right(Val::Px(exp_margin)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Label,
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Px(10.),
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            ..Default::default()
                        },
                        background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                        ..Default::default()
                    },
                    ExpBar,
                ));
            });

            parent.spawn((
                TextBundle {
                    text: exp_text,
                    style: Style {
                        margin: UiRect::left(Val::Px(exp_margin)),
                        ..Default::default()
                    },
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                Label,
            ));
        });
    });
}

pub fn update_expbar(
    mut expbar_query: Query<&mut Style, With<ExpBar>>,
    level: Res<Level>,
    experience: Res<Experience>,
) {
    for mut style in expbar_query.iter_mut() {
        style.width = Val::Percent(100. - 100. * experience.0 as f32 / level.exp_needed_for_next_level() as f32);
    }
}

pub fn update_life_counter(
    mut text_query: Query<&mut Text, With<LifeCounter>>,
    upgrades: Res<Upgrades>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(" {:03}", upgrades.get(Upgrade::ExtraLife));
    }
}

pub fn update_wave_counter(
    mut text_query: Query<&mut Text, With<WaveTimerUi>>,
    time: Res<WaveTimer>,
) {
    let minutes = (time.0 / 60.) as i32;
    let seconds = time.0 as i32 % 60;
    let millis = (time.0 * 100.).floor() % 100.;
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{minutes:02}:{seconds:02}");
        text.sections[1].value = format!(":{millis:03}");
    }
}

pub fn update_score_counter(
    mut text_query: Query<&mut Text, With<ScoreCounter>>,
    score: Res<Score>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{:08}", score.0);
    }
}

pub fn open_shop(
    commands: Commands,
    shop_query: Query<&UpgradeShop>,
    assets: Res<MyAssets>,
    available_upgrades: Res<AvailableUpgrades>,
    upgrades: Res<Upgrades>,
    mut pause_state: ResMut<NextState<PauseState>>,
) {
    if available_upgrades.0 == 0 || !shop_query.is_empty() {
        return;
    }
    if _open_shop(&upgrades, commands, &assets) {
        pause_state.set(PauseState::Shop);
    }
}

fn _open_shop(
    upgrades: &Upgrades,
    mut commands: Commands,
    assets: &MyAssets,
) -> bool {
    let options = upgrades.generate_options();
    if options.len() == 0 {
        return false;
    }
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::rgba(0., 0., 0., 0.2).into(),
            ..Default::default()
        },
        UpgradeShop,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::rgba(0., 0., 0., 0.8).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section("Choose your Upgrade!", TextStyle {
                        font: assets.font.clone(),
                        font_size: 40.,
                        ..Default::default()
                    }),
                    style: Style {
                        margin: UiRect::top(Val::Px(30.)),
                        ..Default::default()
                    },
                    ..Default::default()
                }.with_text_alignment(TextAlignment::Center),
                Label,
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Vw(60.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                for option in options {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(30.)),
                                padding: UiRect::all(Val::Px(5.)),
                                flex_basis: Val::Percent(50.),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            border_color: SHOP_OPTION_BORDER_NORMAL.into(),
                            background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                            ..Default::default()
                        },
                        Interaction::None,
                        option,
                    ))
                    .with_children(|parent| {
                        let (title, description) = if upgrades.get(option) == 0 {
                            (option.unlock_title(), option.unlock_description())
                        } else {
                            (None, None)
                        };
                        let title = title.unwrap_or(option.title());
                        let description = description.unwrap_or(option.description());
                        parent.spawn((
                            TextBundle {
                                text: Text::from_section(title, TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 40.,
                                    ..Default::default()
                                }),
                                style: Style {
                                    margin: UiRect::bottom(Val::Px(10.)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }.with_text_alignment(TextAlignment::Center),
                            Label,
                        ));
                        parent.spawn((
                            TextBundle {
                                text: Text::from_section(description, TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 20.,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            },
                            Label,
                        ));
                    });
                }
            });
        });
    });
    true
}

pub fn update_shop(
    mut commands: Commands,
    shop_query: Query<Entity, With<UpgradeShop>>,
    mut upgrade_query: Query<(&Interaction, &mut BorderColor, &Upgrade), (Changed<Interaction>, Without<UpgradeShop>)>,
    assets: Res<MyAssets>,
    mut available_upgrades: ResMut<AvailableUpgrades>,
    mut pause_state: ResMut<NextState<PauseState>>,
    mut upgrades: ResMut<Upgrades>,
) {
    for (interaction, mut color, option) in upgrade_query.iter_mut() {
        match *interaction {
            Interaction::None => *color = SHOP_OPTION_BORDER_NORMAL.into(),
            Interaction::Hovered => *color = SHOP_OPTION_BORDER_HOVERED.into(),
            Interaction::Pressed => {
                available_upgrades.0 -= 1;
                // apply selected option
                let new_level = upgrades.get(*option) + 1;
                upgrades.0.insert(*option, new_level);
                // rebuild shop or close it
                commands.entity(shop_query.single()).despawn_recursive();
                if available_upgrades.0 == 0 || !_open_shop(&upgrades, commands, &assets) {
                    pause_state.set(PauseState::Unpaused);
                }
                return;
            }
        }
    }
}

pub fn delete_ui(
    mut commands: Commands,
    gui_query: Query<Entity, With<GuiParent>>,
) {
    commands.entity(gui_query.single()).despawn_recursive();
}
