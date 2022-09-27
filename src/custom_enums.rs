    use serde::{Deserialize, Serialize};

    use crate::custom_structs::*;
    use crate::deaths::*;

    #[derive(Clone, Debug, Copy, PartialEq)]
    pub enum PlayerAction {
        TookTurn,
        DidntTakeTurn,
        Exit,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Ai {
        Basic,
        Confused {
            previous_ai: Box<Ai>,
            num_turns: i32,
        },
    }

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeathCallBack {
        Player,
        Monster,
    }

    impl DeathCallBack {
        pub fn callback(self, object: &mut Object, game: &mut Game) {
            use DeathCallBack::*;
            let callback = match self {
                Player => player_death,
                Monster => monster_death,
            };
            callback(object, game);
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Item {
        Heal,
        Lightning,
        Fireball,
        Confuse,
        Sword,
        Shield,
    }

    pub enum UseResult {
        UsedUp,
        UsedAndKept,
        Cancelled,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Slot {
        LeftHand,
        RightHand,
        Head,
    }

    impl std::fmt::Display for Slot {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                Slot::LeftHand => write!(f, "left hand"),
                Slot::RightHand => write!(f, "right hand"),
                Slot::Head => write!(f, "head"),
            }
        }
    }