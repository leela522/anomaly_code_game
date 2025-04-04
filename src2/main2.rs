use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Resource)]
struct GameState {
    is_waiting: bool, // 다음 문제 전환 대기 상태
    current_index: usize, // 0은 튜토리얼, 1~8은 문제
    correct_streak: usize, // 연속 정답 수
    problems: Vec<Problem>,
    current_problem: Option<Problem>,
    disappear_timer: Option<Timer>,
}

#[derive(Clone)]
struct Problem {
    code: &'static str,
    is_correct: bool,
    grade: &'static str, // 예: "1-1", "2-1", "3-1" 등 문제 등급 ID
}

#[derive(Component)]
enum AnswerButton {
    LGTM,
    Hmm,
}

#[derive(Component)]
struct CodeText;

#[derive(Component)]
struct ProgressText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            current_index: 0,
            is_waiting: false,
            correct_streak: 0,
            problems: generate_problem_pool(),
            current_problem: Some(generate_problem_pool()[0].clone()),
            disappear_timer: None,
        })
        .insert_resource(FlickerTimer(Timer::from_seconds(0.3, TimerMode::Once)))
        .add_systems(Startup, setup)
        .add_systems(Update, (button_interaction_system, next_problem_ready_system, problem_disappear_system))
        .run();
}

#[derive(Resource)]
struct FlickerTimer(Timer);

fn next_problem_ready_system(
    time: Res<Time>,
    mut flicker_timer: ResMut<FlickerTimer>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text, With<CodeText>>,
    mut commands: Commands,
    query: Query<(Entity, Option<&Name>)>,
) {
    if game_state.is_waiting {
        if flicker_timer.0.tick(time.delta()).just_finished() {
            let mut rng = thread_rng();
            let new_problem = if game_state.current_index == 0 {
                game_state.problems[0].clone()
            } else {
                game_state.problems.choose(&mut rng).unwrap().clone()
            };
            game_state.current_problem = Some(new_problem.clone());

            if let Ok(mut text) = text_query.get_single_mut() {
                text.sections[0].value = new_problem.code.to_string();
            }

            let blackout_entities: Vec<_> = query
                .iter()
                .filter_map(|(e, name)| name.filter(|n| n.as_str() == "Blackout").map(|_| e))
                .collect();
            for e in blackout_entities {
                commands.entity(e).despawn_recursive();
            }

            if game_state.current_index == 0 {
                game_state.current_index = 1;
            }

            game_state.is_waiting = false;
            flicker_timer.0.reset();

            if new_problem.grade == "2-1" {
                game_state.disappear_timer = Some(Timer::from_seconds(5.0, TimerMode::Once));   
            } else {
                game_state.disappear_timer = None;
            }
        }
    }
}

fn problem_disappear_system(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text, With<CodeText>>,
) {
    if let Some(timer) = &mut game_state.disappear_timer {
        if timer.tick(time.delta()).just_finished() {
            if let Ok(mut text) = text_query.get_single_mut() {
                text.sections[0].value = "".to_string();
            }
            // 한 번 사라진 후 다시 실행되지 않도록 타이머 None 처리
            game_state.disappear_timer = None;
        }
    }
}

fn generate_problem_pool() -> Vec<Problem> {
    vec![
        Problem {
            code: "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #0f0;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    1% better every day
</body>
</html>",
            is_correct: true,
            grade: "0",
        },
        Problem {
            code: "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #0f0;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    1% better every day
</body>
</html>",
            is_correct: true,
            grade: "0-1",
        },
        Problem {
            code: "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #0f0;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    1% better every day
</body>
</html>",
            is_correct: true,
            grade: "0-2",
        },
        Problem {
            code: "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #0f0;
            font-family: 'Courier New', monospace;
            display: yolo;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    1% better every day
</body>
</html>",
            is_correct: false,
            grade: "1-1",
        },
        Problem {
            code: "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #0f0;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    100% better every day
</body>
</html>",
            is_correct: false,
            grade: "1-2",
        },
        Problem {
            code: "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #o_o;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    1% better every day
</body>
</html>",
            is_correct: false,
            grade: "1-3",
        },
        Problem {
            code: "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #0f0;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        
    </style>
</head>
<body>
    1% better every day
</body>
</html>",
            is_correct: false,
            grade: "1-4",
        },
        Problem {
            code: r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>1% Better</title>
    <style>
        body {
            background-color: #111;
            color: #0f0;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    1% better every day
</body>
</html>"#,
            is_correct: false,
            grade: "2-1",
        },
    ]
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_state: Res<GameState>) {
    // 프로그레스 표시
    let progress_label = if game_state.current_index == 0 {
        "Platform 0".to_string()
    } else {
        format!("문제 {}/8", game_state.correct_streak + 1)
    };

    // 프로그레스 텍스트를 하단 중앙에 배치
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                progress_label,
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                    font_size: 24.0,
                    color: Color::GRAY,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(80.0), // 하단
                left: Val::Percent(50.0), // 중앙
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 1.0)),
            ..default()
        },
        ProgressText,
    ));

    // 코드 텍스트
    commands.spawn((
        TextBundle::from_section(
            game_state.problems[0].code,
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                font_size: 20.0,
                color: Color::LIME_GREEN,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(30.0)),
            ..default()
        }),
        CodeText,
    ));

    commands.spawn(Camera2dBundle::default());

    // 버튼 2개 UI (LGTM, Hmm)
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(12.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    AnswerButton::LGTM,  // LGTM 버튼
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "LGTM",
                        TextStyle {
                            font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(12.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    AnswerButton::Hmm,  // Hmm 버튼
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Hmm",
                        TextStyle {
                            font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

fn button_interaction_system(
    mut interaction_query: Query<(&Interaction, &AnswerButton), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text, With<CodeText>>, // CodeText만 처리
    mut progress_text_query: Query<&mut Text, (With<ProgressText>, Without<CodeText>)>, // ProgressText만 처리
    asset_server: Res<AssetServer>,  // asset_server 리소스 추가
    mut commands: Commands,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed && !game_state.is_waiting {
            let expected = match button {
                AnswerButton::LGTM => true,
                AnswerButton::Hmm => false,
            };

            let is_correct = if game_state.current_index == 0 {
                // 처음 문제에서는 그냥 1로 초기화
                if let Ok(mut progress_text) = progress_text_query.get_single_mut() {
                    progress_text.sections[0].value = "Platform 1/8".to_string();
                }
                true
            } else if let Some(problem) = &game_state.current_problem {
                problem.is_correct == expected
            } else {
                false
            };

            if is_correct {
                if game_state.current_index == 0 {
                    game_state.current_index = 1;
                } else {
                    game_state.correct_streak += 1;  // 정답을 맞추면 correct_streak 증가
                    game_state.current_index += 1;

                    if game_state.correct_streak >= 8 {
                        println!("🎉 CLEAR!");
                        game_state.correct_streak = 0;
                        game_state.current_index = 0;

                        // 게임 클리어 엔딩 화면 표시
                        commands.spawn((
                            TextBundle::from_section(
                                "CLEAR!",
                                TextStyle {
                                    font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                                    font_size: 48.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                position_type: PositionType::Absolute,
                                top: Val::Px(200.0),
                                left: Val::Percent(50.0),
                                // transform 대신 style 내에서 직접 설정
                                ..default()
                            }),
                        ));
                    }
                }
            } else {
                println!("틀렸습니다! 다시 처음부터");
                game_state.correct_streak = 0;
                game_state.current_index = 1;
            }

            game_state.is_waiting = true;
            commands.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            }).insert(Name::new("Blackout"));

            if let Ok(mut text) = text_query.get_single_mut() {
                text.sections[0].value = "".to_string();
            }

            // 프로그레스 업데이트
            if let Ok(mut progress_text) = progress_text_query.get_single_mut() {
                let label = if game_state.current_index == 0 {
                    "Platform".to_string()
                } else {
                    format!("Platform {}/8", game_state.correct_streak + 1)  // 현재 문제 번호/8
                };
                progress_text.sections[0].value = label;
            }
        }
    }
}