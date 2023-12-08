use bevy::prelude::*;

use crate::my_assets::MyAssets;
use crate::game::resource::*;

use super::component::*;

pub fn setup_ui(
    mut commands: Commands,
    assets: Res<MyAssets>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        // top ui
        parent.spawn(NodeBundle {
            style: Style {
                ..Default::default()
            },
            ..Default::default()
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

pub fn update_exp(
    mut expbar_query: Query<&mut Style, With<ExpBar>>,
    mut experience: ResMut<Experience>,
) {
    let mut needed_exp = 10;
    while experience.0 >= needed_exp {
        experience.0 -= needed_exp;
        needed_exp = 10;
    }
    for mut style in expbar_query.iter_mut() {
        style.width = Val::Percent(100. - 100. * experience.0 as f32 / needed_exp as f32);
    }
}
