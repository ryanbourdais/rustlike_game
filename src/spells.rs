use tcod::colors::*;

use crate::custom_enums::*;
use crate::custom_structs::*;
use crate::ai::target_tile;

const PLAYER: usize = 0;
const HEAL_AMOUNT: i32 = 40;
const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5; 
const FIREBALL_RADIUS: i32 = 3;
const FIREBALL_DAMAGE: i32 = 25;
const CONFUSE_RANGE: i32 = 8;
const CONFUSE_NUM_TURNS: i32 = 10;

pub fn cast_heal(
    _inventory_id: usize,
    _tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],) -> UseResult {
    let player = &mut objects[PLAYER];
    if let Some(fighter) = player.fighter {
        if fighter.hp == player.max_hp(game) {
            game.messages.add("You are already at full health.", RED);
            return UseResult::Cancelled;
        }
        game.messages.add("Your wounds start to feel better!", LIGHT_VIOLET);
        objects[PLAYER].heal(HEAL_AMOUNT, game);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

pub fn cast_lightning(
    _inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],) -> UseResult {
    let monster_id = closest_monster(tcod, objects, LIGHTNING_RANGE);
    if let Some(monster_id) = monster_id {
        game.messages.add(
            format!(
                "A lightning bolt strikes the {} with a loud thunder! \
                The damage is {} hit points.",
                objects[monster_id].name, LIGHTNING_DAMAGE
            ),
            LIGHT_BLUE,
        );
        objects[monster_id].take_damage(LIGHTNING_DAMAGE, game);
        UseResult::UsedUp
    } else {
        game.messages.add("No enemy is close enough to strike.", RED);
        UseResult::Cancelled
    }
}

pub fn cast_fireball(_inventory_id: usize, tcod: &mut Tcod, game: &mut Game, objects: &mut [Object],) -> UseResult {
    game.messages.add("Left-click a target tile for a fireball, or right-click to cancel.", LIGHT_CYAN,);
    let (x,y) = match target_tile(tcod, game, objects, None) {
        Some(tile_pos) => tile_pos,
        None => return UseResult::Cancelled,
    };
    game.messages.add(
        format!("The fireball explodes, burning everything within {} tiles!", FIREBALL_RADIUS),
        ORANGE,
    );
    let mut xp_to_gain = 0;
    for (id, obj) in objects.iter_mut().enumerate() {
        if obj.distance(x, y) <= FIREBALL_RADIUS as f32 && obj.fighter.is_some() && id != PLAYER {
            game.messages.add(
                format!("The {} gets burned for {} hit points.", obj.name, FIREBALL_DAMAGE),
                ORANGE,
            );
            if let Some(xp) = obj.take_damage(FIREBALL_DAMAGE, game) {
                if id != PLAYER {
                    xp_to_gain += xp;
                }
            }
        }
    }
    objects[PLAYER].fighter.as_mut().unwrap().xp += xp_to_gain;
    UseResult::UsedUp
}

pub fn cast_confuse(_inventory_id: usize, tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    let monster_id = target_monster(tcod, game, objects, Some(CONFUSE_RANGE as f32));
    if let Some(monster_id) = monster_id {
        let old_ai = objects[monster_id].ai.take().unwrap_or(Ai::Basic);
        objects[monster_id].ai = Some(Ai::Confused {
            previous_ai: Box::new(old_ai),
            num_turns: CONFUSE_NUM_TURNS,
        });
        game.messages.add(format!("The eyes of {} look vacant as he starts to stunfkw around", objects[monster_id].name), LIGHT_GREEN);
        UseResult::UsedUp
    } else {
        game.messages.add("No enemy is close enough to strike.", RED );
        UseResult::Cancelled
    }
}

fn target_monster(tcod: &mut Tcod, game: &mut Game, objects: &[Object], max_range: Option<f32>,) -> Option<usize> {
    loop {
        match target_tile(tcod, game, objects, max_range) {
            Some((x, y)) => {
                for (id, obj) in objects.iter().enumerate() {
                    if obj.pos() == (x, y) && obj.fighter.is_some() && id != PLAYER {
                        return Some(id);
                    }
                }
            }
            None => return None,
        }
    }
}

fn closest_monster(tcod: &Tcod, objects: &[Object], max_range: i32) -> Option<usize> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32;

    for (id, object) in objects.iter().enumerate() {
        if (id != PLAYER)
            && object.fighter.is_some()
            && object.ai.is_some()
            && tcod.fov.is_in_fov(object.x, object.y)
            {
                let dist = objects[PLAYER].distance_to(object);
                if dist < closest_dist {
                    closest_enemy = Some(id);
                    closest_dist = dist;
                }
            }
    }
    closest_enemy
}