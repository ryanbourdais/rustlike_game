use rand::Rng;
use tcod::colors::*;
use std::cmp;

use crate::custom_enums::*;
use crate::custom_structs::*;
use crate::levels::*;
use crate::movement::*;

const PLAYER: usize = 0;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;


pub fn make_map(objects: &mut Vec<Object>, level: u32) -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    assert_eq!(&objects[PLAYER] as *const _, &objects[0] as *const _);
    objects.truncate(1);

    let mut rooms = vec![];

    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            create_room(new_room, &mut map);

            place_objects(new_room, &map, objects, level);

            let(new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                objects[PLAYER].set_pos(new_x, new_y);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }

    let (last_room_x, last_room_y) = rooms[rooms.len() -1].center();
    let mut stairs = Object::new(last_room_x, last_room_y, '<', "stairs", WHITE, false);
    stairs.always_visible = true;
    objects.push(stairs);

    map
}

pub fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1,y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object>, level: u32){
    use rand::distributions::{IndependentSample, Weighted, WeightedChoice};

    let max_monsters = from_dungeon_level(
        &[
            Transition {level: 1, value: 2},
            Transition {level: 4, value: 3},
            Transition {level: 6, value: 5},
        ],
        level,
    );

    let num_monsters = rand::thread_rng().gen_range(0, max_monsters + 1);

    let monster_chances = &mut [
        Weighted {
            weight: 80,
            item: "orc",
        },
        Weighted {
            weight: 20,
            item: "troll",
        },
    ];

    let monster_choice = WeightedChoice::new(monster_chances);

    let max_items = from_dungeon_level(
        &[
            Transition { level: 1, value: 1},
            Transition { level: 4, value: 2},
        ],
        level,
    );

    let item_chances = &mut [
        Weighted {
            weight: from_dungeon_level(&[Transition { level: 4, value: 5}], level),
            item: Item::Sword,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 8,
                    value: 15,
                }],
                level,
            ),
            item: Item::Shield,
        },
        Weighted {
            weight: 35,
            item: Item::Heal,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 4,
                    value: 25,
                }],
                level,
            ),
            item: Item::Lightning,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 6,
                    value: 25,
                }],
                level,
            ),
            item: Item::Fireball,
        },
        Weighted {
            weight: from_dungeon_level(
                &[Transition {
                    level: 2,
                    value: 10,
                }],
                level,
            ),
            item: Item::Confuse,
        },
    ];

    let item_choice = WeightedChoice::new(item_chances);

    let num_items = rand::thread_rng().gen_range(0, max_items + 1);

    for _ in 0 .. num_monsters 
    {
        let x  = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        if !is_blocked(x, y, map, objects){
            
            let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) 
            {
                "orc" => {
                    let mut orc = Object::new(x, y, 'o', "orc", DESATURATED_GREEN, true);
                    orc.fighter = Some(Fighter {
                        base_max_hp: 20,
                        hp: 20, 
                        base_defense: 0, 
                        base_power: 4,
                        xp: 35, 
                        on_death: DeathCallBack::Monster,
                    });
                    orc.ai = Some(Ai::Basic);
                    orc
                }
                "troll" => {
                    let mut troll = Object::new(x, y, 'T', "troll", DARKER_GREEN, true);
                    troll.fighter = Some(Fighter {
                        base_max_hp: 30,
                        hp: 30,
                        base_defense: 2,
                        base_power: 8,
                        xp: 100,
                        on_death: DeathCallBack::Monster,
                    });
                    troll.ai = Some(Ai::Basic);
                    troll
                }
                _ => unreachable!(),
            };
        monster.alive = true;
        objects.push(monster);

        for _ in 0..num_items {
            let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
            let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

            if !is_blocked(x, y, map, objects) {
                let item = match item_choice.ind_sample(&mut rand::thread_rng()) {
                    Item::Shield => {
                        let mut object = Object::new(x, y, '[', "shield", DARKER_ORANGE, false);
                        object.item = Some(Item::Shield);
                        object.equipment = Some(Equipment {
                            equipped: false,
                            slot: Slot::LeftHand,
                            max_hp_bonus: 0,
                            defense_bonus: 1,
                            power_bonus: 0,
                        });
                        object
                    }
                    Item::Sword => {
                        let mut object = Object::new(x, y, '/', "sword", SKY, false);
                        object.item = Some(Item::Sword);
                        object.equipment = Some(Equipment {
                            equipped: false,
                            slot: Slot::RightHand,
                            max_hp_bonus: 0,
                            defense_bonus: 0,
                            power_bonus: 3,
                        });
                        object
                    }
                    Item::Heal => {
                        let mut object = Object::new(x, y, '!', "healing potion", VIOLET, false);
                        object.item = Some(Item::Heal);
                        object
                    }
                    Item::Lightning => {
                        let mut object = Object::new(
                            x,
                            y,
                            '#',
                            "scroll of lightning bolt",
                            LIGHT_YELLOW,
                            false,
                        );
                        object.item = Some(Item::Lightning);
                        object
                    }
                    Item::Fireball => {
                        let mut object = Object::new(x, y, '#', "scroll of fireball", LIGHT_YELLOW, false);
                        object.item = Some(Item::Fireball);
                        object
                    }
                    Item::Confuse => {
                        let mut object = Object::new(
                            x, y, '#', "scroll of confusion", LIGHT_YELLOW, false);
                            object.item = Some(Item::Confuse);
                            object
                    }
            };
                objects.push(item);
                }
            }
        }
    }
}
