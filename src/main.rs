use tcod::console::*;
use tcod::map::{Map as FovMap};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const PANEL_HEIGHT: i32 = 7;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

const LIMIT_FPS: i32 = 20;


mod ai;
mod custom_enums;
mod custom_structs;
mod deaths;
mod equipment;
mod game_states;
mod get_names;
mod items;
mod level_up;
mod levels;
mod map;
mod menu;
mod message_box;
mod movement;
mod player_controller;
mod rendering;
mod spells;


fn main() {

    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("RustLike Game")
        .init();

    let mut tcod = custom_structs::Tcod { 
        root, 
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT), 
        panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT), 
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        key: Default::default(),
        mouse: Default::default(),
    }; 

    menu::main_menu(&mut tcod);

}