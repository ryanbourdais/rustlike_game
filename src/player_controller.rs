use tcod::input::{Key};

use crate::custom_enums::*;
use crate::custom_structs::*;
use crate::movement::*;
use crate::items::*;
use crate::equipment::*;
use crate::levels::*;
use crate::message_box::*;

const PLAYER: usize = 0;
const LEVEL_UP_BASE: i32 = 200;
const LEVEL_UP_FACTOR: i32 = 150;
const CHARACTER_SCREEN_WIDTH: i32 = 30;

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> PlayerAction {
    use PlayerAction::*;
    use tcod::input::KeyCode::*;

    let player_alive = objects[PLAYER].alive;
    match (tcod.key, tcod.key.text(), player_alive) {
        (Key { code: Up, .. }, _, true ) => {player_move_or_attack(0, -1, game, objects);
        TookTurn }
        (Key { code: Down, .. }, _, true ) => {player_move_or_attack(0, 1, game, objects);
            TookTurn }
        (Key { code: Left, .. }, _, true ) => {player_move_or_attack(-1, 0, game, objects);
        TookTurn }
        (Key { code: Right, .. }, _, true ) => { player_move_or_attack(1, 0, game, objects);
        TookTurn }
        (Key { code: Text, .. }, "g", true) => {
            let item_id = objects
                .iter()
                .position(|object| object.pos() == objects[PLAYER].pos() && object.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
            DidntTakeTurn
        }
        (Key { code: Text, ..}, "d", true) => {
            let inventory_index = inventory_menu(
                &game.inventory,
                "Press the key next to an item to drop it, or any other to cancel. \n'",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                drop_item(inventory_index, game, objects);
            }
            DidntTakeTurn
        }
        (Key { code: Text, .. }, "i", true) => {
            let inventory_index = inventory_menu(
                &game.inventory,
                "Press the key next to an item to use it, or any other to cancel.\n", 
                &mut tcod.root
            );
            if let Some(inventory_index) = inventory_index {
                use_item(inventory_index, tcod, game, objects);
            }
            DidntTakeTurn
        }
        (Key { code: Enter, alt: true, .. }, _ , _) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        },
        (Key { code: Escape, .. }, _ , _  )=> Exit,
        (Key { code: Text, ..}, "<", true) => {
            let player_on_stairs = objects
            .iter()
            .any(|object| object.pos() == objects[PLAYER].pos() && object.name == "stairs");
            if player_on_stairs {
                next_level(tcod, game, objects);
            }
            DidntTakeTurn
        },
        (Key { code: Text, ..}, "c", true) => {
            let player = &objects[PLAYER];
            let level = player.level;
            let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
            if let Some(fighter) = player.fighter.as_ref() {
                let msg = format!(
                    "Character information
                    
                    Level: {}
                    Experience: {}
                    Experience to level up:{}
                    
                    Maximum HP: {}
                    Attack: {}
                    Defense: {}",
                    level, fighter.xp, level_up_xp, player.max_hp(game), player.power(game), player.defense(game),
                );
                msgbox(&msg, CHARACTER_SCREEN_WIDTH, &mut tcod.root);
            }
            DidntTakeTurn
        }
        _ => { DidntTakeTurn}
    }
}