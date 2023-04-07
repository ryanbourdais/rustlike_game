use tcod::colors::*;
use tcod::console::*;
use tcod::input::{self, Event};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use crate::custom_structs::*;
use crate::custom_enums::*;
use crate::rendering::*;
use crate::map::*;
use crate::player_controller::*;
use crate::level_up::*;
use crate::ai::*;

const PLAYER: usize = 0;

pub fn new_game(tcod: &mut Tcod) -> (Game, Vec<Object>) {
    let mut player = Object::new(0, 0, '@', "player", WHITE, true);
    player.alive = true;
    player.fighter = Some(Fighter {
        base_max_hp: 100, 
        hp: 100, 
        base_defense: 1, 
        base_power: 2, 
        xp: 0,
        on_death: DeathCallBack::Player,
    });

    let mut objects = vec![player];

    let mut game = Game {
         map: make_map(&mut objects, 1),
        messages: Messages::new(),
        inventory: vec![],
        dungeon_level: 1,
    };

    let mut dagger = Object::new(0, 0, '-', "dagger", SKY, false);
    dagger.item = Some(Item::Sword);
    dagger.equipment = Some(Equipment {
        equipped: true,
        slot: Slot::LeftHand,
        max_hp_bonus: 0,
        defense_bonus: 0,
        power_bonus: 2,
    });
    game.inventory.push(dagger);

    initialise_fov(tcod, &game.map);

    game.messages.add("Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.", RED,);

    (game, objects)
}

pub fn save_game(game: &Game, objects: &[Object]) -> Result<(), Box<dyn Error>> {
    let save_data = serde_json::to_string(&(game, objects))?;
    let mut file = File::create("savegame")?;
    file.write_all(save_data.as_bytes())?;
    Ok(())
}

pub fn load_game() -> Result<(Game, Vec<Object>), Box<dyn Error>> {
    let mut json_save_state = String::new();
    let mut file = File::open("savegame")?;
    file.read_to_string(&mut json_save_state)?;
    let result = serde_json::from_str::<(Game, Vec<Object>)>(&json_save_state)?;
    Ok(result)
}

pub fn play_game(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) {
    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        tcod.con.clear();

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        }

        let fov_recompute = previous_player_position != (objects[PLAYER].pos());
        render_all(tcod, game, &objects, fov_recompute);

        tcod.root.flush();

        level_up(tcod, game, objects);

        previous_player_position = objects[PLAYER].pos();
        let player_action = handle_keys(tcod, game, objects);
        if player_action == PlayerAction::Exit {
            save_game(game, objects).unwrap();
            break;
        }
        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &tcod, game, objects);
                }
            }
        }
    }
}