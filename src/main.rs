use bevy::prelude::*;
use bevy::ui::Val::Px;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb_u8(48, 44, 44)))
        .insert_resource(WindowDescriptor {
            title: "Gerrymandering".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(Selected { pos: Vec::new() })
        .insert_resource(Selection {
            selections: Vec::new(),
        })
        .add_startup_system(startup)
        .add_system(mouse_click)
        .run();
}

#[derive(Component)]
struct Points {
    pos: (u16, u16), // this will be like x and y so it starts at the bottom left
    color: u8,       // 0: blue, 1: red
}

#[derive(Component)]
struct Selected {
    pos: Vec<(u8, u8)>,
}

#[derive(Component)]
struct Selection {
    selections: Vec<(Vec<(u8, u8)>, u8)>,
}

#[derive(Component)]
struct Lines;

#[derive(Component)]
struct DeleteText;

#[derive(Component)]
struct PermLines;

static mut CURRENT_XY: (u8, u8) = (0, 0);
static MAX: u16 = 600;
static LINE_THINKNESS: f32 = 5.0;
static mut LEVEL: u8 = 1;
static mut GO_FOR: u8 = 0;

fn startup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-650.0, 0.0, 0.0),
            scale: Vec3::new(10.0, 750.0, 1.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(650.0, 0.0, 0.0),
            scale: Vec3::new(10.0, 750.0, 1.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 370.0, 0.0),
            scale: Vec3::new(1310.0, 10.0, 1.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -370.0, 0.0),
            scale: Vec3::new(1310.0, 10.0, 1.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Px(110.0),
                        left: Px(225.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Done",
                    TextStyle {
                        font: server.load("font/troika.otf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            });
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-320.0, -200.0, 0.0),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                texture: server.load("button_back.png"),
                ..Default::default()
            });
    unsafe {
        CURRENT_XY = (((LEVEL / 2) * 2) + 3, ((LEVEL / 2) * 2) + 3);
    }
    spawn_points(make_points(), commands, server);
}

fn mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    commands1: Commands,
    mut selected: ResMut<Selected>,
    mut selection: ResMut<Selection>,
    lines: Query<Entity, With<Lines>>,
    perm_lines: Query<Entity, With<PermLines>>,
    points_entity: Query<Entity, With<Points>>,
    points: Query<&Points>,
    server: Res<AssetServer>,
    delete_text: Query<Entity, With<DeleteText>>,
) {
    unsafe {
        let win = windows.get_primary().expect("no primary window");
        let mut redraw_selected = false;
        let mut redraw_selection = false;
        let size = (
            MAX as f32 / (CURRENT_XY.0) as f32,
            MAX as f32 / (CURRENT_XY.1) as f32,
        );
        if mouse_input.pressed(MouseButton::Left) {
            let pos =
                win.cursor_position().unwrap() - Vec2::new(win.width() / 2.0, win.height() / 2.0);
                if selection.selections.len() as u8 == CURRENT_XY.1 && pos.x > -570.0 && pos.x < -70.0 && pos.y > -350.0 && pos.y < -50.0 {
                    let mut colors = 0;
                    for x in &selection.selections {
                        colors += x.1;
                    }
                    if (GO_FOR == 0 && (colors as f32) < (CURRENT_XY.1 as f32) / 2.0)
                        || (GO_FOR == 1 && (colors as f32) > (CURRENT_XY.1 as f32) / 2.0)
                    {
                        for i in perm_lines.iter() {
                            commands.entity(i).despawn();
                        }
                        for i in lines.iter() {
                            commands.entity(i).despawn();
                        }
                        for i in points_entity.iter() {
                            commands.entity(i).despawn();
                        }
                        for i in delete_text.iter() {
                            commands.entity(i).despawn();
                        }
                        selected.pos = Vec::new();
                        selection.selections = Vec::new();
                        LEVEL += 1;
                        CURRENT_XY = (((LEVEL / 2) * 2) + 3, ((LEVEL / 2) * 2) + 3);
                        spawn_points(make_points(), commands1, server);
                        redraw_selected = true;
                        redraw_selection = true;
                    }
                }
            let mut has_selected_that_can_not = false;
            for i in &selection.selections {
                if i.0.contains(&(
                    ((pos.x as f32 + 20.0) / size.0).ceil() as u8,
                    ((pos.y as f32 + 300.0) / size.1).ceil() as u8,
                )) {
                    has_selected_that_can_not = true;
                }
            }
            if ((pos.x as f32 + 20.0) / size.0).ceil() <= CURRENT_XY.0 as f32
                && ((pos.x as f32 + 20.0) / size.0).ceil() > 0.0
                && ((pos.y as f32 + 300.0) / size.1).ceil() <= CURRENT_XY.1 as f32
                && ((pos.y as f32 + 300.0) / size.1).ceil() > 0.0
                && !selected.pos.contains(&(
                    ((pos.x as f32 + 20.0) / size.0).ceil() as u8,
                    ((pos.y as f32 + 300.0) / size.1).ceil() as u8,
                ))
                && selected.pos.len() < CURRENT_XY.1 as usize
                && !has_selected_that_can_not
            {
                selected.pos.push((
                    ((pos.x as f32 + 20.0) / size.0).ceil() as u8,
                    ((pos.y as f32 + 300.0) / size.1).ceil() as u8,
                ));
                redraw_selected = true;
            }
            if selected.pos.clone().len() == CURRENT_XY.1 as usize {
                let mut ammount_of_red = 0;
                for i in points.iter() {
                    if selected
                        .pos
                        .contains(&(i.pos.0 as u8 + 1, i.pos.1 as u8 + 1))
                    {
                        ammount_of_red += i.color;
                    }
                }
                selection.selections.push((
                    selected.pos.clone(),
                    if ammount_of_red as f32 > selected.pos.len() as f32 / 2.0 {
                        1
                    } else {
                        0
                    },
                ));
                selected.pos = Vec::new();
                redraw_selection = true;
            }
        }
        if mouse_input.pressed(MouseButton::Right) {
            let pos =
                win.cursor_position().unwrap() - Vec2::new(win.width() / 2.0, win.height() / 2.0);
            if ((pos.x as f32 + 20.0) / size.0).ceil() <= CURRENT_XY.0 as f32
                && ((pos.x as f32 + 20.0) / size.0).ceil() > 0.0
                && ((pos.y as f32 + 300.0) / size.1).ceil() <= CURRENT_XY.1 as f32
                && ((pos.y as f32 + 300.0) / size.1).ceil() > 0.0
            {
                let mut stop = false;
                if selected.pos.contains(&(
                    ((pos.x as f32 + 20.0) / size.0).ceil() as u8,
                    ((pos.y as f32 + 300.0) / size.1).ceil() as u8,
                )) {
                    for i in 0..selected.pos.len() {
                        if !stop {
                            if selected.pos.get(i).unwrap()
                                == &(
                                    ((pos.x as f32 + 20.0) / size.0).ceil() as u8,
                                    ((pos.y as f32 + 300.0) / size.1).ceil() as u8,
                                )
                            {
                                selected.pos.remove(i);
                                redraw_selected = true;
                                stop = true;
                            }
                        }
                    }
                }
                stop = false;
                for i in 0..selection.selections.len() {
                    if !stop {
                        if selection.selections.get(i).unwrap().0.contains(&(
                            ((pos.x as f32 + 20.0) / size.0).ceil() as u8,
                            ((pos.y as f32 + 300.0) / size.1).ceil() as u8,
                        )) {
                            selection.selections.remove(i);
                            redraw_selection = true;
                            stop = true;
                        }
                    }
                }
            }
        }

        if redraw_selected {
            for i in lines.iter() {
                commands.entity(i).despawn();
            }
            for i in &selected.pos {
                if !selected.pos.contains(&(i.0 - 1, i.1)) {
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    (i.0 as f32 - 1.0) * size.0 - 20.0,
                                    (i.1 as f32 - 1.0) * size.1 + size.1 / 2.0 - 300.0,
                                    0.0,
                                ),
                                scale: Vec3::new(
                                    LINE_THINKNESS,
                                    size.1 + LINE_THINKNESS / 2.0,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Lines);
                }
                if !selected.pos.contains(&(i.0 + 1, i.1)) {
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    (i.0 as f32 - 1.0) * size.0 + size.0 - 20.0,
                                    (i.1 as f32 - 1.0) * size.1 + size.1 / 2.0 - 300.0,
                                    0.0,
                                ),
                                scale: Vec3::new(
                                    LINE_THINKNESS,
                                    size.1 + LINE_THINKNESS / 2.0,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Lines);
                }
                if !selected.pos.contains(&(i.0, i.1 - 1)) {
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    (i.0 as f32 - 1.0) * size.0 + size.0 / 2.0 - 20.0,
                                    (i.1 as f32 - 1.0) * size.1 - 300.0,
                                    0.0,
                                ),
                                scale: Vec3::new(
                                    size.0 + LINE_THINKNESS / 2.0,
                                    LINE_THINKNESS,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Lines);
                }
                if !selected.pos.contains(&(i.0, i.1 + 1)) {
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    (i.0 as f32 - 1.0) * size.0 + size.0 / 2.0 - 20.0,
                                    (i.1 as f32 - 1.0) * size.1 + size.1 - 300.0,
                                    0.0,
                                ),
                                scale: Vec3::new(
                                    size.0 + LINE_THINKNESS / 2.0,
                                    LINE_THINKNESS,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Lines);
                }
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                (i.0 as f32 - 1.0) * size.0 + size.0 / 2.0 - 20.0,
                                (i.1 as f32 - 1.0) * size.1 + size.1 / 2.0 - 300.0,
                                1.0,
                            ),
                            scale: Vec3::new(size.0, size.1, 0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.01),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Lines);
            }
        }

        if redraw_selection {
            for i in perm_lines.iter() {
                commands.entity(i).despawn();
            }
            for i in &selection.selections {
                for x in &i.0 {
                    if !i.0.contains(&(x.0 - 1, x.1)) {
                        commands
                            .spawn_bundle(SpriteBundle {
                                transform: Transform {
                                    translation: Vec3::new(
                                        (x.0 as f32 - 1.0) * size.0 - 20.0,
                                        (x.1 as f32 - 1.0) * size.1 + size.1 / 2.0 - 300.0,
                                        0.0,
                                    ),
                                    scale: Vec3::new(
                                        LINE_THINKNESS,
                                        size.1 + LINE_THINKNESS / 2.0,
                                        0.0,
                                    ),
                                    ..Default::default()
                                },
                                sprite: Sprite {
                                    color: Color::rgb_u8(169, 169, 169),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PermLines);
                    }
                    if !i.0.contains(&(x.0 + 1, x.1)) {
                        commands
                            .spawn_bundle(SpriteBundle {
                                transform: Transform {
                                    translation: Vec3::new(
                                        (x.0 as f32 - 1.0) * size.0 + size.0 - 20.0,
                                        (x.1 as f32 - 1.0) * size.1 + size.1 / 2.0 - 300.0,
                                        0.0,
                                    ),
                                    scale: Vec3::new(
                                        LINE_THINKNESS,
                                        size.1 + LINE_THINKNESS / 2.0,
                                        0.0,
                                    ),
                                    ..Default::default()
                                },
                                sprite: Sprite {
                                    color: Color::rgb_u8(169, 169, 169),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PermLines);
                    }
                    if !i.0.contains(&(x.0, x.1 - 1)) {
                        commands
                            .spawn_bundle(SpriteBundle {
                                transform: Transform {
                                    translation: Vec3::new(
                                        (x.0 as f32 - 1.0) * size.0 + size.0 / 2.0 - 20.0,
                                        (x.1 as f32 - 1.0) * size.1 - 300.0,
                                        0.0,
                                    ),
                                    scale: Vec3::new(
                                        size.0 + LINE_THINKNESS / 2.0,
                                        LINE_THINKNESS,
                                        0.0,
                                    ),
                                    ..Default::default()
                                },
                                sprite: Sprite {
                                    color: Color::rgb_u8(169, 169, 169),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PermLines);
                    }
                    if !i.0.contains(&(x.0, x.1 + 1)) {
                        commands
                            .spawn_bundle(SpriteBundle {
                                transform: Transform {
                                    translation: Vec3::new(
                                        (x.0 as f32 - 1.0) * size.0 + size.0 / 2.0 - 20.0,
                                        (x.1 as f32 - 1.0) * size.1 + size.1 - 300.0,
                                        0.0,
                                    ),
                                    scale: Vec3::new(
                                        size.0 + LINE_THINKNESS / 2.0,
                                        LINE_THINKNESS,
                                        0.0,
                                    ),
                                    ..Default::default()
                                },
                                sprite: Sprite {
                                    color: Color::rgb_u8(169, 169, 169),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(PermLines);
                    }
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    (x.0 as f32 - 1.0) * size.0 + size.0 / 2.0 - 20.0,
                                    (x.1 as f32 - 1.0) * size.1 + size.1 / 2.0 - 300.0,
                                    1.0,
                                ),
                                scale: Vec3::new(size.0, size.1, 0.0),
                                ..Default::default()
                            },
                            sprite: match i.1 {
                                0 => Sprite {
                                    color: Color::rgba_u8(33, 91, 166, 128),
                                    ..Default::default()
                                },
                                _ => Sprite {
                                    color: Color::rgba_u8(166, 36, 47, 128),
                                    ..Default::default()
                                },
                            },
                            ..Default::default()
                        })
                        .insert(PermLines);
                }
            }
        }
    }
}

fn make_points() -> Vec<Points> {
    unsafe {
        // this will be better but you know idc
        let mut output: Vec<Points> = Vec::new();
        for x in 0..CURRENT_XY.0 {
            for y in 0..CURRENT_XY.1 {
                output.push(Points {
                    pos: (x.into(), y.into()),
                    color: (rand::random::<f32>() * 2.0).floor() as u8,
                })
            }
        }
        let mut ammount_of_red = 0;
        for i in &output {
            ammount_of_red += i.color;
        }
        while (ammount_of_red as f32) > (CURRENT_XY.0 * CURRENT_XY.1) as f32 / 5.0 * 3.0
            || (ammount_of_red as f32) < (CURRENT_XY.0 * CURRENT_XY.1) as f32 / 5.0 * 2.0
        {
            output = Vec::new();
            for x in 0..CURRENT_XY.0 {
                for y in 0..CURRENT_XY.1 {
                    output.push(Points {
                        pos: (x.into(), y.into()),
                        color: (rand::random::<f32>() * 2.0).floor() as u8,
                    })
                }
            }
            ammount_of_red = 0;
            for i in &output {
                ammount_of_red += i.color;
            }
        }
        GO_FOR = if (ammount_of_red as f32) > (CURRENT_XY.0 * CURRENT_XY.1) as f32 / 2.0 {
            0
        } else if (ammount_of_red as f32) < (CURRENT_XY.0 * CURRENT_XY.1) as f32 / 2.0 {
            1
        } else {
            (rand::random::<f32>() * 2.0).floor() as u8
        };
        output
    }
}

fn spawn_points(
    x: Vec<Points>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    unsafe {
        let size = (
            MAX as f32 / (CURRENT_XY.0) as f32,
            MAX as f32 / (CURRENT_XY.1) as f32,
        );
        for i in &x {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            i.pos.0 as f32 * size.0 + size.0 / 2.0 - 20.0,
                            i.pos.1 as f32 * size.1 + size.1 / 2.0 - 300.0,
                            0.0,
                        ),
                        scale: Vec3::new(size.0 * 0.8 / 1024.0, size.1 * 0.8 / 1024.0, 1.0),
                        ..Default::default()
                    },
                    texture: match i.color {
                        0 => server.load("blue.png"),
                        1 => server.load("red.png"),
                        _ => server.load("cant happen so lest error it out"),
                    },
                    ..Default::default()
                })
                .insert(Points {
                    pos: i.pos,
                    color: i.color,
                });
        }
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Px(50.0),
                        left: Px(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    format!("Level: {}", LEVEL),
                    TextStyle {
                        font: server.load("font/troika.otf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            })
            .insert(DeleteText);
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Px(175.0),
                        left: Px(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    match GO_FOR {
                        0 => "Try and make\n blue win",
                        _ => "Try and make\n red win",
                    },
                    TextStyle {
                        font: server.load("font/troika.otf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            })
            .insert(DeleteText);
    }
}
