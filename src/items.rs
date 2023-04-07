use tcod::colors::*;

use crate::custom_enums::*;
use crate::custom_structs::*;
use crate::spells::*;
use crate::equipment::*;

const PLAYER: usize = 0;

pub fn use_item(inventory_id: usize, tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) {
    use Item::*;

    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Heal => cast_heal,
            Lightning => cast_lightning,
            Fireball => cast_fireball,
            Confuse => cast_confuse,
            Sword => toggle_equipment,
            Shield => toggle_equipment,
        };
        match on_use(inventory_id, tcod, game, objects) {
            UseResult::UsedUp => {
                game.inventory.remove(inventory_id);
            }
            UseResult::UsedAndKept => {}
            UseResult::Cancelled => {
                game.messages.add("Cancelled", WHITE);
            }
        }
    } else {
        game.messages.add(
            format!("The {} cannot be used.", game.inventory[inventory_id].name),
            WHITE,
        );
    }
}

pub fn pick_item_up(object_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(format!("Your inventory is full, cannot pickup {}.", objects[object_id].name), RED,);
    }
    else {
        let item = objects.swap_remove(object_id);
        game.messages.add(format!("You picked up a {}!", item.name), GREEN);
        let index = game.inventory.len();
        let slot = item.equipment.map(|e| e.slot);
        game.inventory.push(item);

        if let Some(slot) = slot {
            if get_equipped_in_slot(slot, &game.inventory).is_none() {
                game.inventory[index].equip(&mut game.messages);
            }
        }
    }
}

pub fn drop_item(inventory_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    let mut item = game.inventory.remove(inventory_id);
    if item.equipment.is_some() {
        item.dequip(&mut game.messages);
    }
    item.set_pos(objects[PLAYER].x, objects[PLAYER].y);
    game.messages.add(
        format!("You dropped a {}.", item.name), YELLOW
    );
    objects.push(item);
}