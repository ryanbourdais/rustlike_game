use tcod::colors::*;

use crate::custom_structs::*;

pub fn player_death(player: &mut Object, game: &mut Game) {
    game.messages.add("You died!", RED);

    player.char = '%';
    player.color = DARK_RED;
}

pub fn monster_death(monster: &mut Object, game: &mut Game) {
    game.messages.add(
        format!(
            "{} is dead! You gain {} experience points", 
            monster.name, monster.fighter.unwrap().xp), 
        ORANGE);
    monster.char = '%';
    monster.color = DARK_RED;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}