    use serde::{Deserialize, Serialize};
    use tcod::colors::*;
    use tcod::console::*;
    use tcod::map::{Map as FovMap};
    use tcod::input::{Key, Mouse};

    use crate::custom_enums::*;

    pub struct Tcod {
    pub root: Root,
    con: Offscreen,
    panel: Offscreen,
    fov: FovMap,
    key: Key,
    mouse: Mouse,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Messages {
        messages: Vec<(String, Color)>,
    }
    impl Messages {
        pub fn new() -> Self {
            Self{ messages: vec! [] }
        }
        pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
            self.messages.push((message.into(), color));
        }
        pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
            self.messages.iter() 
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Object {
        x: i32,
        y: i32,
        pub char: char,
        pub color: Color,
        pub name: String,
        pub blocks: bool,
        alive: bool,
        pub fighter: Option<Fighter>,
        pub ai: Option<Ai>,
        item: Option<Item>,
        always_visible: bool,
        level: i32,
        equipment: Option<Equipment>, 
    }
    impl Object {
        pub fn new(x:i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Self {
            Object {
                x: x,
                y: y,
                char: char,
                color: color,
                name: name.into(),
                blocks: blocks,
                alive: false,
                fighter: None,
                ai: None,
                item: None,
                always_visible: false,
                level: 1,
                equipment: None,
            }
        }

        pub fn draw(&self, con: &mut dyn Console) {
            con.set_default_foreground(self.color);
            con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
        }

        pub fn pos(&self) -> (i32, i32) {
            (self.x, self.y)
        }

        pub fn set_pos(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
        pub fn distance_to(&self, other: &Object) -> f32 {
            let dx = other.x - self.x;
            let dy = other.y  - self.y;
            ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
        }
        pub fn distance(&self, x: i32, y:i32) -> f32 {
            (((x - self.x).pow(2) + (y - self.y).pow(2)) as f32).sqrt()
        }
        pub fn take_damage(&mut self, damage: i32, game: &mut Game) -> Option<i32> {
            if let Some(fighter) = self.fighter.as_mut() {
                if damage > 0 {
                    fighter.hp -= damage;
                }
            }
            if let Some(fighter) = self.fighter {
                if fighter.hp <= 0 {
                    self.alive = false;
                    fighter.on_death.callback(self, game);
                    return Some(fighter.xp);
                }
            }
            None
        }
        pub fn attack(&mut self, target: &mut Object, game: &mut Game) {
            let damage = self.power(game) - target.defense(game);
            if damage > 0 {
                game.messages.add(format!("{} attacks {} for {} hit points.", self.name, target.name, damage), WHITE);
                if let Some(xp) = target.take_damage(damage, game) {
                    self.fighter.as_mut().unwrap().xp += xp;
                }
            }
            else {
                game.messages.add(format!("{} attacks {} but it has no effect!", self.name, target.name), WHITE);
            }
        }
        pub fn heal(&mut self, amount: i32, game: &Game) {
            let max_hp = self.max_hp(game);
            if let Some(ref mut fighter) = self.fighter {
                fighter.hp += amount;
                if fighter.hp > max_hp {
                    fighter.hp = max_hp;
                }
            }
        }
        pub fn equip(&mut self, messages: &mut Messages) {
            if self.item.is_none() {
                messages.add(
                    format!("Can't equip {:?} because it's not an Item.", self),
                    RED,
                );
                return;
            };
            if let Some(ref mut equipment) = self.equipment {
                if !equipment.equipped {
                    equipment.equipped = true;
                    messages.add(
                        format!("Equipped {} on {}.", self.name, equipment.slot),
                        LIGHT_GREEN,
                    );
                }
            } else {
                messages.add(
                    format!("Can't equip {:?} because it's not an Equipment.", self),
                    RED,
                );
            }
        }
        pub fn dequip(&mut self, messages: &mut Messages) {
            if self.item.is_none() {
                messages.add(
                    format!("Can't dequip {:?} because it's not an Item.", self),
                    RED,
                );
                return;
            };
            if let Some(ref mut equipment) = self.equipment {
                if equipment.equipped {
                    equipment.equipped = false;
                    messages.add(
                        format!("Dequipped {} from {}.", self.name, equipment.slot),
                        LIGHT_YELLOW,
                    );
                }
            } else {
                messages.add(
                    format!("Can't dequip {:?} because it's not an Equipment.", self),
                    RED,
                );
            }
        }
        pub fn get_all_equipped(&self, game: &Game) -> Vec<Equipment> {
            if self.name == "player" {
                game.inventory
                    .iter()
                    .filter(|item| item.equipment.map_or(false, |e| e.equipped))
                    .map(|item| item.equipment.unwrap())
                    .collect()
            } else {
                vec![]
            }
        }
        pub fn power(&self, game: &Game) -> i32 {
            let base_power = self.fighter.map_or(0, |f| f.base_power);
            let bonus: i32 = self
                            .get_all_equipped(game)
                            .iter()
                            .map(|e| e.power_bonus)
                            .sum();
            base_power + bonus
        }
        pub fn defense(&self, game: &Game) -> i32 {
            let base_defense = self.fighter.map_or(0, |f| f.base_defense);
            let bonus: i32 = self  
                            .get_all_equipped(game)
                            .iter()
                            .map(|e| e.defense_bonus)
                            .sum();
            base_defense + bonus
        }
        pub fn max_hp(&self, game: &Game) -> i32 {
            let base_max_hp = self.fighter.map_or(0, |f| f.base_max_hp);
            let bonus: i32 = self
                                .get_all_equipped(game)
                                .iter()
                                .map(|e| e.max_hp_bonus)
                                .sum();
            base_max_hp + bonus
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Fighter {
        base_max_hp: i32,
        hp: i32,
        base_defense: i32,
        base_power: i32,
        pub xp: i32,
        on_death: DeathCallBack,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Equipment {
        slot: Slot,
        equipped: bool,
        power_bonus: i32,
        defense_bonus: i32,
        max_hp_bonus: i32,
    }

    pub struct Transition {
        level: u32,
        value: u32,
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct Tile {
        blocked: bool,
        explored: bool,
        block_sight: bool,
    }
    impl Tile {
        pub fn empty() -> Self {
            Tile {
                blocked: false,
                explored: false,
                block_sight: false,
            }
        }
        pub fn wall() -> Self {
            Tile {
                blocked: true,
                explored: false,
                block_sight: true,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Rect {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    }
    impl Rect {
        pub fn new(x: i32, y:i32, w: i32, h: i32) -> Self {
            Rect {
                x1: x,
                y1: y,
                x2: x + w,
                y2: y + h,
            }
        }
        pub fn center(&self) -> (i32, i32) {
            let center_x = (self.x1 + self.x2) /2;
            let center_y = (self.y1 + self.y2) /2;
            (center_x, center_y)
        }
        pub fn intersects_with(&self, other: &Rect) -> bool {
            (self.x1 <= other.x2) && (self.x2 >= other.x1) && (self.y1 <= other.y2) && (self.y2 >= other.y1)
        }
    }

    type Map = Vec<Vec<Tile>>;

    #[derive(Serialize, Deserialize)]
    pub struct Game {
        map: Map,
        pub messages: Messages,
        inventory: Vec<Object>,
        dungeon_level: u32,
    }
