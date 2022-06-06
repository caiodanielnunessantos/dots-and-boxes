

use std::{fs::File, io::Read};

use crate::colors::{COLOR_NAMES, self};
use crate::app::Player;

const DEFAULT_CONFIG_FILE: &str = "config.ini";

pub fn get_options() {
    let mut window_size = (1200, 700);
    let mut line_size = 50;
    let mut background_color = colors::LIGHT_GRAY;
    let mut dot_color = colors::BLACK;
    let mut board_size = (10, 8);
    let mut hover_color = colors::BLACK;
    let players = [Player(colors::LIME_GREEN), Player(colors::BLUE), Player(colors::RED)];
    let mut custom_players = Vec::new();

    if let Ok(mut file) = File::open(DEFAULT_CONFIG_FILE) {
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        buf.lines().for_each(|line: &str| {
            if line.starts_with('#') {()}
            if line.contains('=') {
                if let Some((start, end)) = line.split_once('=') {
                    let (mut start, mut end) = (
                        start.to_lowercase(),
                        end.trim().to_lowercase()
                    );
                    start.remove_matches(|character: char| {
                        !character.is_ascii_lowercase()
                    });
                    match &*start {
                        "windowsize" => {
                            let numbers: Vec<i32> = end.split(|character: char| {
                                !character.is_ascii_digit()
                            }).filter_map(|part: &str| {
                                if let Ok(val) = i32::from_str_radix(part, 10) {
                                    Some(val)
                                } else { None }
                            }).collect();
                            if numbers.len() == 2 {
                                window_size = (numbers[0], numbers[1]);
                            }
                        },
                        "linesize" => {
                            let numbers: Vec<i32> = end.split(|character: char| {
                                !character.is_ascii_digit()
                            }).filter_map(|part: &str| {
                                if let Ok(val) = i32::from_str_radix(part, 10) {
                                    Some(val)
                                } else { None }
                            }).collect();
                            if numbers.len() == 1 {
                                line_size = numbers[0];
                            }
                        },
                        "boardsize" => {
                            let numbers: Vec<i32> = end.split(|character: char| {
                                !character.is_ascii_digit()
                            }).filter_map(|part: &str| {
                                if let Ok(val) = i32::from_str_radix(part, 10) {
                                    Some(val)
                                } else { None }
                            }).collect();
                            if numbers.len() == 2 {
                                board_size = (numbers[0], numbers[1]);
                            }
                        },
                        "backgroundcolor" => {
                            end.remove_matches(|character: char| {
                                !character.is_ascii_lowercase()
                            });
                            if let Some(color) = colors::COLOR_NAMES.into_iter().find_map(|el: &(&str, sdl2::pixels::Color)| {
                                let mut color = el.0.to_ascii_lowercase();
                                color.remove_matches(|character: char| {
                                    !character.is_ascii_lowercase()
                                });
                                if end == color { Some(el.1) }
                                else { None }
                            }) {
                                background_color = color;
                            };
                        },
                        "dotcolor" => {
                            end.remove_matches(|character: char| {
                                !character.is_ascii_lowercase()
                            });
                            if let Some(color) = colors::COLOR_NAMES.into_iter().find_map(|el: &(&str, sdl2::pixels::Color)| {
                                let mut color = el.0.to_ascii_lowercase();
                                color.remove_matches(|character: char| {
                                    !character.is_ascii_lowercase()
                                });
                                if end == color { Some(el.1) }
                                else { None }
                            }) {
                                dot_color = color;
                            };
                        },
                        "hovercolor" => {
                            end.remove_matches(|character: char| {
                                !character.is_ascii_lowercase()
                            });
                            if let Some(color) = colors::COLOR_NAMES.into_iter().find_map(|el: &(&str, sdl2::pixels::Color)| {
                                let mut color = el.0.to_ascii_lowercase();
                                color.remove_matches(|character: char| {
                                    !character.is_ascii_lowercase()
                                });
                                if end == color { Some(el.1) }
                                else { None }
                            }) {
                                hover_color = color;
                            };
                        },
                        "playercolor" => {
                            end.remove_matches(|character: char| {
                                !character.is_ascii_lowercase()
                            });
                            if let Some(color) = colors::COLOR_NAMES.into_iter().find_map(|el: &(&str, sdl2::pixels::Color)| {
                                let mut color = el.0.to_ascii_lowercase();
                                color.remove_matches(|character: char| {
                                    !character.is_ascii_lowercase()
                                });
                                if end == color { Some(el.1) }
                                else { None }
                            }) {
                                custom_players.push(crate::app::Player(color));
                            };
                        },
                        _ => ()
                    }
                };
            }
        });
    }
    if custom_players.len() >= 2 {
        let mut application = crate::app::App::new(
            window_size,
            line_size,
            background_color,
            dot_color,
            board_size,
            hover_color,
            &custom_players
        );
        application.init();
    } else {
        let mut application = crate::app::App::new(
            window_size,
            line_size,
            background_color,
            dot_color,
            board_size,
            hover_color,
            &players
        );
        application.init();
    }
}