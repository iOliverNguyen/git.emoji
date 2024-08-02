use std::collections::HashMap;

pub struct Emojis {
    all: Option<Vec<&'static str>>,
    map: Option<HashMap<&'static str, ()>>,
}

impl Emojis {
    pub fn new() -> Self {
        Self {
            all: None,
            map: None,
        }
    }
    fn init(&mut self) {
        if let Some(_) = self.all {
            return;
        }
        let all: Vec<&str> = EMOJI_STR.split('|').map(|x| x.trim()).collect();
        let map = all.iter().map(|&s| (s, ())).collect();
        self.all = Some(all);
        self.map = Some(map);
    }
    pub fn all(&mut self) -> Vec<&'static str> {
        self.init();
        self.all.clone().unwrap()
    }
    pub fn get(&mut self, emoji: &str) -> bool {
        self.init();
        self.map.clone().unwrap().contains_key(emoji)
    }
}

// https://github.com/sweaver2112/Regex-combined-emojis?tab=readme-ov-file#the-pattern-compact-version
const EMOJI_STR: &str = r#"🧑🏻‍❤️‍💋‍🧑🏼|🧑🏻‍❤️‍💋‍🧑🏽|🧑🏻‍❤️‍💋‍🧑🏾|🧑🏻‍❤️‍💋‍🧑🏿|🧑🏼‍❤️‍💋‍🧑🏻|🧑🏼‍❤️‍💋‍🧑🏽|🧑🏼‍❤️‍💋‍🧑🏾|🧑🏼‍❤️‍💋‍🧑🏿|🧑🏽‍❤️‍💋‍🧑🏻|🧑🏽‍❤️‍💋‍🧑🏼|🧑🏽‍❤️‍💋‍🧑🏾|🧑🏽‍❤️‍💋‍🧑🏿|🧑🏾‍❤️‍💋‍🧑🏻|🧑🏾‍❤️‍💋‍🧑🏼|🧑🏾‍❤️‍💋‍🧑🏽|🧑🏾‍❤️‍💋‍🧑🏿|🧑🏿‍❤️‍💋‍🧑🏻|🧑🏿‍❤️‍💋‍🧑🏼|🧑🏿‍❤️‍💋‍🧑🏽|🧑🏿‍❤️‍💋‍🧑🏾|👩🏻‍❤️‍💋‍👨🏻|👩🏻‍❤️‍💋‍👨🏼|👩🏻‍❤️‍💋‍👨🏽|👩🏻‍❤️‍💋‍👨🏾|👩🏻‍❤️‍💋‍👨🏿|👩🏼‍❤️‍💋‍👨🏻|👩🏼‍❤️‍💋‍👨🏼|👩🏼‍❤️‍💋‍👨🏽|👩🏼‍❤️‍💋‍👨🏾|👩🏼‍❤️‍💋‍👨🏿|👩🏽‍❤️‍💋‍👨🏻|👩🏽‍❤️‍💋‍👨🏼|👩🏽‍❤️‍💋‍👨🏽|👩🏽‍❤️‍💋‍👨🏾|👩🏽‍❤️‍💋‍👨🏿|👩🏾‍❤️‍💋‍👨🏻|👩🏾‍❤️‍💋‍👨🏼|👩🏾‍❤️‍💋‍👨🏽|👩🏾‍❤️‍💋‍👨🏾|👩🏾‍❤️‍💋‍👨🏿|👩🏿‍❤️‍💋‍👨🏻|👩🏿‍❤️‍💋‍👨🏼|👩🏿‍❤️‍💋‍👨🏽|👩🏿‍❤️‍💋‍👨🏾|👩🏿‍❤️‍💋‍👨🏿|👨🏻‍❤️‍💋‍👨🏻|👨🏻‍❤️‍💋‍👨🏼|👨🏻‍❤️‍💋‍👨🏽|👨🏻‍❤️‍💋‍👨🏾|👨🏻‍❤️‍💋‍👨🏿|👨🏼‍❤️‍💋‍👨🏻|👨🏼‍❤️‍💋‍👨🏼|👨🏼‍❤️‍💋‍👨🏽|👨🏼‍❤️‍💋‍👨🏾|👨🏼‍❤️‍💋‍👨🏿|👨🏽‍❤️‍💋‍👨🏻|👨🏽‍❤️‍💋‍👨🏼|👨🏽‍❤️‍💋‍👨🏽|👨🏽‍❤️‍💋‍👨🏾|👨🏽‍❤️‍💋‍👨🏿|👨🏾‍❤️‍💋‍👨🏻|👨🏾‍❤️‍💋‍👨🏼|👨🏾‍❤️‍💋‍👨🏽|👨🏾‍❤️‍💋‍👨🏾|👨🏾‍❤️‍💋‍👨🏿|👨🏿‍❤️‍💋‍👨🏻|👨🏿‍❤️‍💋‍👨🏼|👨🏿‍❤️‍💋‍👨🏽|👨🏿‍❤️‍💋‍👨🏾|👨🏿‍❤️‍💋‍👨🏿|👩🏻‍❤️‍💋‍👩🏻|👩🏻‍❤️‍💋‍👩🏼|👩🏻‍❤️‍💋‍👩🏽|👩🏻‍❤️‍💋‍👩🏾|👩🏻‍❤️‍💋‍👩🏿|👩🏼‍❤️‍💋‍👩🏻|👩🏼‍❤️‍💋‍👩🏼|👩🏼‍❤️‍💋‍👩🏽|👩🏼‍❤️‍💋‍👩🏾|👩🏼‍❤️‍💋‍👩🏿|👩🏽‍❤️‍💋‍👩🏻|👩🏽‍❤️‍💋‍👩🏼|👩🏽‍❤️‍💋‍👩🏽|👩🏽‍❤️‍💋‍👩🏾|👩🏽‍❤️‍💋‍👩🏿|👩🏾‍❤️‍💋‍👩🏻|👩🏾‍❤️‍💋‍👩🏼|👩🏾‍❤️‍💋‍👩🏽|👩🏾‍❤️‍💋‍👩🏾|👩🏾‍❤️‍💋‍👩🏿|👩🏿‍❤️‍💋‍👩🏻|👩🏿‍❤️‍💋‍👩🏼|👩🏿‍❤️‍💋‍👩🏽|👩🏿‍❤️‍💋‍👩🏾|👩🏿‍❤️‍💋‍👩🏿|🏴󠁧󠁢󠁥󠁮󠁧󠁿|🏴󠁧󠁢󠁳󠁣󠁴󠁿|🏴󠁧󠁢󠁷󠁬󠁳󠁿|🧑🏻‍🤝‍🧑🏻|🧑🏻‍🤝‍🧑🏼|🧑🏻‍🤝‍🧑🏽|🧑🏻‍🤝‍🧑🏾|🧑🏻‍🤝‍🧑🏿|🧑🏼‍🤝‍🧑🏻|🧑🏼‍🤝‍🧑🏼|🧑🏼‍🤝‍🧑🏽|🧑🏼‍🤝‍🧑🏾|🧑🏼‍🤝‍🧑🏿|🧑🏽‍🤝‍🧑🏻|🧑🏽‍🤝‍🧑🏼|🧑🏽‍🤝‍🧑🏽|🧑🏽‍🤝‍🧑🏾|🧑🏽‍🤝‍🧑🏿|🧑🏾‍🤝‍🧑🏻|🧑🏾‍🤝‍🧑🏼|🧑🏾‍🤝‍🧑🏽|🧑🏾‍🤝‍🧑🏾|🧑🏾‍🤝‍🧑🏿|🧑🏿‍🤝‍🧑🏻|🧑🏿‍🤝‍🧑🏼|🧑🏿‍🤝‍🧑🏽|🧑🏿‍🤝‍🧑🏾|🧑🏿‍🤝‍🧑🏿|👩🏻‍🤝‍👩🏼|👩🏻‍🤝‍👩🏽|👩🏻‍🤝‍👩🏾|👩🏻‍🤝‍👩🏿|👩🏼‍🤝‍👩🏻|👩🏼‍🤝‍👩🏽|👩🏼‍🤝‍👩🏾|👩🏼‍🤝‍👩🏿|👩🏽‍🤝‍👩🏻|👩🏽‍🤝‍👩🏼|👩🏽‍🤝‍👩🏾|👩🏽‍🤝‍👩🏿|👩🏾‍🤝‍👩🏻|👩🏾‍🤝‍👩🏼|👩🏾‍🤝‍👩🏽|👩🏾‍🤝‍👩🏿|👩🏿‍🤝‍👩🏻|👩🏿‍🤝‍👩🏼|👩🏿‍🤝‍👩🏽|👩🏿‍🤝‍👩🏾|👩🏻‍🤝‍👨🏼|👩🏻‍🤝‍👨🏽|👩🏻‍🤝‍👨🏾|👩🏻‍🤝‍👨🏿|👩🏼‍🤝‍👨🏻|👩🏼‍🤝‍👨🏽|👩🏼‍🤝‍👨🏾|👩🏼‍🤝‍👨🏿|👩🏽‍🤝‍👨🏻|👩🏽‍🤝‍👨🏼|👩🏽‍🤝‍👨🏾|👩🏽‍🤝‍👨🏿|👩🏾‍🤝‍👨🏻|👩🏾‍🤝‍👨🏼|👩🏾‍🤝‍👨🏽|👩🏾‍🤝‍👨🏿|👩🏿‍🤝‍👨🏻|👩🏿‍🤝‍👨🏼|👩🏿‍🤝‍👨🏽|👩🏿‍🤝‍👨🏾|👨🏻‍🤝‍👨🏼|👨🏻‍🤝‍👨🏽|👨🏻‍🤝‍👨🏾|👨🏻‍🤝‍👨🏿|👨🏼‍🤝‍👨🏻|👨🏼‍🤝‍👨🏽|👨🏼‍🤝‍👨🏾|👨🏼‍🤝‍👨🏿|👨🏽‍🤝‍👨🏻|👨🏽‍🤝‍👨🏼|👨🏽‍🤝‍👨🏾|👨🏽‍🤝‍👨🏿|👨🏾‍🤝‍👨🏻|👨🏾‍🤝‍👨🏼|👨🏾‍🤝‍👨🏽|👨🏾‍🤝‍👨🏿|👨🏿‍🤝‍👨🏻|👨🏿‍🤝‍👨🏼|👨🏿‍🤝‍👨🏽|👨🏿‍🤝‍👨🏾|🧑🏻‍❤️‍🧑🏼|🧑🏻‍❤️‍🧑🏽|🧑🏻‍❤️‍🧑🏾|🧑🏻‍❤️‍🧑🏿|🧑🏼‍❤️‍🧑🏻|🧑🏼‍❤️‍🧑🏽|🧑🏼‍❤️‍🧑🏾|🧑🏼‍❤️‍🧑🏿|🧑🏽‍❤️‍🧑🏻|🧑🏽‍❤️‍🧑🏼|🧑🏽‍❤️‍🧑🏾|🧑🏽‍❤️‍🧑🏿|🧑🏾‍❤️‍🧑🏻|🧑🏾‍❤️‍🧑🏼|🧑🏾‍❤️‍🧑🏽|🧑🏾‍❤️‍🧑🏿|🧑🏿‍❤️‍🧑🏻|🧑🏿‍❤️‍🧑🏼|🧑🏿‍❤️‍🧑🏽|🧑🏿‍❤️‍🧑🏾|👩🏻‍❤️‍👨🏻|👩🏻‍❤️‍👨🏼|👩🏻‍❤️‍👨🏽|👩🏻‍❤️‍👨🏾|👩🏻‍❤️‍👨🏿|👩🏼‍❤️‍👨🏻|👩🏼‍❤️‍👨🏼|👩🏼‍❤️‍👨🏽|👩🏼‍❤️‍👨🏾|👩🏼‍❤️‍👨🏿|👩🏽‍❤️‍👨🏻|👩🏽‍❤️‍👨🏼|👩🏽‍❤️‍👨🏽|👩🏽‍❤️‍👨🏾|👩🏽‍❤️‍👨🏿|👩🏾‍❤️‍👨🏻|👩🏾‍❤️‍👨🏼|👩🏾‍❤️‍👨🏽|👩🏾‍❤️‍👨🏾|👩🏾‍❤️‍👨🏿|👩🏿‍❤️‍👨🏻|👩🏿‍❤️‍👨🏼|👩🏿‍❤️‍👨🏽|👩🏿‍❤️‍👨🏾|👩🏿‍❤️‍👨🏿|👨🏻‍❤️‍👨🏻|👨🏻‍❤️‍👨🏼|👨🏻‍❤️‍👨🏽|👨🏻‍❤️‍👨🏾|👨🏻‍❤️‍👨🏿|👨🏼‍❤️‍👨🏻|👨🏼‍❤️‍👨🏼|👨🏼‍❤️‍👨🏽|👨🏼‍❤️‍👨🏾|👨🏼‍❤️‍👨🏿|👨🏽‍❤️‍👨🏻|👨🏽‍❤️‍👨🏼|👨🏽‍❤️‍👨🏽|👨🏽‍❤️‍👨🏾|👨🏽‍❤️‍👨🏿|👨🏾‍❤️‍👨🏻|👨🏾‍❤️‍👨🏼|👨🏾‍❤️‍👨🏽|👨🏾‍❤️‍👨🏾|👨🏾‍❤️‍👨🏿|👨🏿‍❤️‍👨🏻|👨🏿‍❤️‍👨🏼|👨🏿‍❤️‍👨🏽|👨🏿‍❤️‍👨🏾|👨🏿‍❤️‍👨🏿|👩🏻‍❤️‍👩🏻|👩🏻‍❤️‍👩🏼|👩🏻‍❤️‍👩🏽|👩🏻‍❤️‍👩🏾|👩🏻‍❤️‍👩🏿|👩🏼‍❤️‍👩🏻|👩🏼‍❤️‍👩🏼|👩🏼‍❤️‍👩🏽|👩🏼‍❤️‍👩🏾|👩🏼‍❤️‍👩🏿|👩🏽‍❤️‍👩🏻|👩🏽‍❤️‍👩🏼|👩🏽‍❤️‍👩🏽|👩🏽‍❤️‍👩🏾|👩🏽‍❤️‍👩🏿|👩🏾‍❤️‍👩🏻|👩🏾‍❤️‍👩🏼|👩🏾‍❤️‍👩🏽|👩🏾‍❤️‍👩🏾|👩🏾‍❤️‍👩🏿|👩🏿‍❤️‍👩🏻|👩🏿‍❤️‍👩🏼|👩🏿‍❤️‍👩🏽|👩🏿‍❤️‍👩🏾|👩🏿‍❤️‍👩🏿|👩‍❤️‍💋‍👨|👨‍❤️‍💋‍👨|👩‍❤️‍💋‍👩|👨‍👩‍👧‍👦|👨‍👩‍👦‍👦|👨‍👩‍👧‍👧|👨‍👨‍👧‍👦|👨‍👨‍👦‍👦|👨‍👨‍👧‍👧|👩‍👩‍👧‍👦|👩‍👩‍👦‍👦|👩‍👩‍👧‍👧|🧑‍🧑‍🧒‍🧒|🚶🏻‍♀️‍➡️|🚶🏼‍♀️‍➡️|🚶🏽‍♀️‍➡️|🚶🏾‍♀️‍➡️|🚶🏿‍♀️‍➡️|🚶🏻‍♂️‍➡️|🚶🏼‍♂️‍➡️|🚶🏽‍♂️‍➡️|🚶🏾‍♂️‍➡️|🚶🏿‍♂️‍➡️|🧎🏻‍♀️‍➡️|🧎🏼‍♀️‍➡️|🧎🏽‍♀️‍➡️|🧎🏾‍♀️‍➡️|🧎🏿‍♀️‍➡️|🧎🏻‍♂️‍➡️|🧎🏼‍♂️‍➡️|🧎🏽‍♂️‍➡️|🧎🏾‍♂️‍➡️|🧎🏿‍♂️‍➡️|🧑🏻‍🦯‍➡️|🧑🏼‍🦯‍➡️|🧑🏽‍🦯‍➡️|🧑🏾‍🦯‍➡️|🧑🏿‍🦯‍➡️|👨🏻‍🦯‍➡️|👨🏼‍🦯‍➡️|👨🏽‍🦯‍➡️|👨🏾‍🦯‍➡️|👨🏿‍🦯‍➡️|👩🏻‍🦯‍➡️|👩🏼‍🦯‍➡️|👩🏽‍🦯‍➡️|👩🏾‍🦯‍➡️|👩🏿‍🦯‍➡️|🧑🏻‍🦼‍➡️|🧑🏼‍🦼‍➡️|🧑🏽‍🦼‍➡️|🧑🏾‍🦼‍➡️|🧑🏿‍🦼‍➡️|👨🏻‍🦼‍➡️|👨🏼‍🦼‍➡️|👨🏽‍🦼‍➡️|👨🏾‍🦼‍➡️|👨🏿‍🦼‍➡️|👩🏻‍🦼‍➡️|👩🏼‍🦼‍➡️|👩🏽‍🦼‍➡️|👩🏾‍🦼‍➡️|👩🏿‍🦼‍➡️|🧑🏻‍🦽‍➡️|🧑🏼‍🦽‍➡️|🧑🏽‍🦽‍➡️|🧑🏾‍🦽‍➡️|🧑🏿‍🦽‍➡️|👨🏻‍🦽‍➡️|👨🏼‍🦽‍➡️|👨🏽‍🦽‍➡️|👨🏾‍🦽‍➡️|👨🏿‍🦽‍➡️|👩🏻‍🦽‍➡️|👩🏼‍🦽‍➡️|👩🏽‍🦽‍➡️|👩🏾‍🦽‍➡️|👩🏿‍🦽‍➡️|🏃🏻‍♀️‍➡️|🏃🏼‍♀️‍➡️|🏃🏽‍♀️‍➡️|🏃🏾‍♀️‍➡️|🏃🏿‍♀️‍➡️|🏃🏻‍♂️‍➡️|🏃🏼‍♂️‍➡️|🏃🏽‍♂️‍➡️|🏃🏾‍♂️‍➡️|🏃🏿‍♂️‍➡️|🫱🏻‍🫲🏼|🫱🏻‍🫲🏽|🫱🏻‍🫲🏾|🫱🏻‍🫲🏿|🫱🏼‍🫲🏻|🫱🏼‍🫲🏽|🫱🏼‍🫲🏾|🫱🏼‍🫲🏿|🫱🏽‍🫲🏻|🫱🏽‍🫲🏼|🫱🏽‍🫲🏾|🫱🏽‍🫲🏿|🫱🏾‍🫲🏻|🫱🏾‍🫲🏼|🫱🏾‍🫲🏽|🫱🏾‍🫲🏿|🫱🏿‍🫲🏻|🫱🏿‍🫲🏼|🫱🏿‍🫲🏽|🫱🏿‍🫲🏾|🚶‍♀️‍➡️|🚶‍♂️‍➡️|🧎‍♀️‍➡️|🧎‍♂️‍➡️|🧑‍🦯‍➡️|👨‍🦯‍➡️|👩‍🦯‍➡️|🧑‍🦼‍➡️|👨‍🦼‍➡️|👩‍🦼‍➡️|🧑‍🦽‍➡️|👨‍🦽‍➡️|👩‍🦽‍➡️|🏃‍♀️‍➡️|🏃‍♂️‍➡️|🧑‍🤝‍🧑|👩‍❤️‍👨|👨‍❤️‍👨|👩‍❤️‍👩|👨‍👩‍👦|👨‍👩‍👧|👨‍👨‍👦|👨‍👨‍👧|👩‍👩‍👦|👩‍👩‍👧|👨‍👦‍👦|👨‍👧‍👦|👨‍👧‍👧|👩‍👦‍👦|👩‍👧‍👦|👩‍👧‍👧|🧑‍🧑‍🧒|🧑‍🧒‍🧒|👁️‍🗨️|🧔🏻‍♂️|🧔🏼‍♂️|🧔🏽‍♂️|🧔🏾‍♂️|🧔🏿‍♂️|🧔🏻‍♀️|🧔🏼‍♀️|🧔🏽‍♀️|🧔🏾‍♀️|🧔🏿‍♀️|👨🏻‍🦰|👨🏼‍🦰|👨🏽‍🦰|👨🏾‍🦰|👨🏿‍🦰|👨🏻‍🦱|👨🏼‍🦱|👨🏽‍🦱|👨🏾‍🦱|👨🏿‍🦱|👨🏻‍🦳|👨🏼‍🦳|👨🏽‍🦳|👨🏾‍🦳|👨🏿‍🦳|👨🏻‍🦲|👨🏼‍🦲|👨🏽‍🦲|👨🏾‍🦲|👨🏿‍🦲|👩🏻‍🦰|👩🏼‍🦰|👩🏽‍🦰|👩🏾‍🦰|👩🏿‍🦰|🧑🏻‍🦰|🧑🏼‍🦰|🧑🏽‍🦰|🧑🏾‍🦰|🧑🏿‍🦰|👩🏻‍🦱|👩🏼‍🦱|👩🏽‍🦱|👩🏾‍🦱|👩🏿‍🦱|🧑🏻‍🦱|🧑🏼‍🦱|🧑🏽‍🦱|🧑🏾‍🦱|🧑🏿‍🦱|👩🏻‍🦳|👩🏼‍🦳|👩🏽‍🦳|👩🏾‍🦳|👩🏿‍🦳|🧑🏻‍🦳|🧑🏼‍🦳|🧑🏽‍🦳|🧑🏾‍🦳|🧑🏿‍🦳|👩🏻‍🦲|👩🏼‍🦲|👩🏽‍🦲|👩🏾‍🦲|👩🏿‍🦲|🧑🏻‍🦲|🧑🏼‍🦲|🧑🏽‍🦲|🧑🏾‍🦲|🧑🏿‍🦲|👱🏻‍♀️|👱🏼‍♀️|👱🏽‍♀️|👱🏾‍♀️|👱🏿‍♀️|👱🏻‍♂️|👱🏼‍♂️|👱🏽‍♂️|👱🏾‍♂️|👱🏿‍♂️|🙍🏻‍♂️|🙍🏼‍♂️|🙍🏽‍♂️|🙍🏾‍♂️|🙍🏿‍♂️|🙍🏻‍♀️|🙍🏼‍♀️|🙍🏽‍♀️|🙍🏾‍♀️|🙍🏿‍♀️|🙎🏻‍♂️|🙎🏼‍♂️|🙎🏽‍♂️|🙎🏾‍♂️|🙎🏿‍♂️|🙎🏻‍♀️|🙎🏼‍♀️|🙎🏽‍♀️|🙎🏾‍♀️|🙎🏿‍♀️|🙅🏻‍♂️|🙅🏼‍♂️|🙅🏽‍♂️|🙅🏾‍♂️|🙅🏿‍♂️|🙅🏻‍♀️|🙅🏼‍♀️|🙅🏽‍♀️|🙅🏾‍♀️|🙅🏿‍♀️|🙆🏻‍♂️|🙆🏼‍♂️|🙆🏽‍♂️|🙆🏾‍♂️|🙆🏿‍♂️|🙆🏻‍♀️|🙆🏼‍♀️|🙆🏽‍♀️|🙆🏾‍♀️|🙆🏿‍♀️|💁🏻‍♂️|💁🏼‍♂️|💁🏽‍♂️|💁🏾‍♂️|💁🏿‍♂️|💁🏻‍♀️|💁🏼‍♀️|💁🏽‍♀️|💁🏾‍♀️|💁🏿‍♀️|🙋🏻‍♂️|🙋🏼‍♂️|🙋🏽‍♂️|🙋🏾‍♂️|🙋🏿‍♂️|🙋🏻‍♀️|🙋🏼‍♀️|🙋🏽‍♀️|🙋🏾‍♀️|🙋🏿‍♀️|🧏🏻‍♂️|🧏🏼‍♂️|🧏🏽‍♂️|🧏🏾‍♂️|🧏🏿‍♂️|🧏🏻‍♀️|🧏🏼‍♀️|🧏🏽‍♀️|🧏🏾‍♀️|🧏🏿‍♀️|🙇🏻‍♂️|🙇🏼‍♂️|🙇🏽‍♂️|🙇🏾‍♂️|🙇🏿‍♂️|🙇🏻‍♀️|🙇🏼‍♀️|🙇🏽‍♀️|🙇🏾‍♀️|🙇🏿‍♀️|🤦🏻‍♂️|🤦🏼‍♂️|🤦🏽‍♂️|🤦🏾‍♂️|🤦🏿‍♂️|🤦🏻‍♀️|🤦🏼‍♀️|🤦🏽‍♀️|🤦🏾‍♀️|🤦🏿‍♀️|🤷🏻‍♂️|🤷🏼‍♂️|🤷🏽‍♂️|🤷🏾‍♂️|🤷🏿‍♂️|🤷🏻‍♀️|🤷🏼‍♀️|🤷🏽‍♀️|🤷🏾‍♀️|🤷🏿‍♀️|🧑🏻‍⚕️|🧑🏼‍⚕️|🧑🏽‍⚕️|🧑🏾‍⚕️|🧑🏿‍⚕️|👨🏻‍⚕️|👨🏼‍⚕️|👨🏽‍⚕️|👨🏾‍⚕️|👨🏿‍⚕️|👩🏻‍⚕️|👩🏼‍⚕️|👩🏽‍⚕️|👩🏾‍⚕️|👩🏿‍⚕️|🧑🏻‍🎓|🧑🏼‍🎓|🧑🏽‍🎓|🧑🏾‍🎓|🧑🏿‍🎓|👨🏻‍🎓|👨🏼‍🎓|👨🏽‍🎓|👨🏾‍🎓|👨🏿‍🎓|👩🏻‍🎓|👩🏼‍🎓|👩🏽‍🎓|👩🏾‍🎓|👩🏿‍🎓|🧑🏻‍🏫|🧑🏼‍🏫|🧑🏽‍🏫|🧑🏾‍🏫|🧑🏿‍🏫|👨🏻‍🏫|👨🏼‍🏫|👨🏽‍🏫|👨🏾‍🏫|👨🏿‍🏫|👩🏻‍🏫|👩🏼‍🏫|👩🏽‍🏫|👩🏾‍🏫|👩🏿‍🏫|🧑🏻‍⚖️|🧑🏼‍⚖️|🧑🏽‍⚖️|🧑🏾‍⚖️|🧑🏿‍⚖️|👨🏻‍⚖️|👨🏼‍⚖️|👨🏽‍⚖️|👨🏾‍⚖️|👨🏿‍⚖️|👩🏻‍⚖️|👩🏼‍⚖️|👩🏽‍⚖️|👩🏾‍⚖️|👩🏿‍⚖️|🧑🏻‍🌾|🧑🏼‍🌾|🧑🏽‍🌾|🧑🏾‍🌾|🧑🏿‍🌾|👨🏻‍🌾|👨🏼‍🌾|👨🏽‍🌾|👨🏾‍🌾|👨🏿‍🌾|👩🏻‍🌾|👩🏼‍🌾|👩🏽‍🌾|👩🏾‍🌾|👩🏿‍🌾|🧑🏻‍🍳|🧑🏼‍🍳|🧑🏽‍🍳|🧑🏾‍🍳|🧑🏿‍🍳|👨🏻‍🍳|👨🏼‍🍳|👨🏽‍🍳|👨🏾‍🍳|👨🏿‍🍳|👩🏻‍🍳|👩🏼‍🍳|👩🏽‍🍳|👩🏾‍🍳|👩🏿‍🍳|🧑🏻‍🔧|🧑🏼‍🔧|🧑🏽‍🔧|🧑🏾‍🔧|🧑🏿‍🔧|👨🏻‍🔧|👨🏼‍🔧|👨🏽‍🔧|👨🏾‍🔧|👨🏿‍🔧|👩🏻‍🔧|👩🏼‍🔧|👩🏽‍🔧|👩🏾‍🔧|👩🏿‍🔧|🧑🏻‍🏭|🧑🏼‍🏭|🧑🏽‍🏭|🧑🏾‍🏭|🧑🏿‍🏭|👨🏻‍🏭|👨🏼‍🏭|👨🏽‍🏭|👨🏾‍🏭|👨🏿‍🏭|👩🏻‍🏭|👩🏼‍🏭|👩🏽‍🏭|👩🏾‍🏭|👩🏿‍🏭|🧑🏻‍💼|🧑🏼‍💼|🧑🏽‍💼|🧑🏾‍💼|🧑🏿‍💼|👨🏻‍💼|👨🏼‍💼|👨🏽‍💼|👨🏾‍💼|👨🏿‍💼|👩🏻‍💼|👩🏼‍💼|👩🏽‍💼|👩🏾‍💼|👩🏿‍💼|🧑🏻‍🔬|🧑🏼‍🔬|🧑🏽‍🔬|🧑🏾‍🔬|🧑🏿‍🔬|👨🏻‍🔬|👨🏼‍🔬|👨🏽‍🔬|👨🏾‍🔬|👨🏿‍🔬|👩🏻‍🔬|👩🏼‍🔬|👩🏽‍🔬|👩🏾‍🔬|👩🏿‍🔬|🧑🏻‍💻|🧑🏼‍💻|🧑🏽‍💻|🧑🏾‍💻|🧑🏿‍💻|👨🏻‍💻|👨🏼‍💻|👨🏽‍💻|👨🏾‍💻|👨🏿‍💻|👩🏻‍💻|👩🏼‍💻|👩🏽‍💻|👩🏾‍💻|👩🏿‍💻|🧑🏻‍🎤|🧑🏼‍🎤|🧑🏽‍🎤|🧑🏾‍🎤|🧑🏿‍🎤|👨🏻‍🎤|👨🏼‍🎤|👨🏽‍🎤|👨🏾‍🎤|👨🏿‍🎤|👩🏻‍🎤|👩🏼‍🎤|👩🏽‍🎤|👩🏾‍🎤|👩🏿‍🎤|🧑🏻‍🎨|🧑🏼‍🎨|🧑🏽‍🎨|🧑🏾‍🎨|🧑🏿‍🎨|👨🏻‍🎨|👨🏼‍🎨|👨🏽‍🎨|👨🏾‍🎨|👨🏿‍🎨|👩🏻‍🎨|👩🏼‍🎨|👩🏽‍🎨|👩🏾‍🎨|👩🏿‍🎨|🧑🏻‍✈️|🧑🏼‍✈️|🧑🏽‍✈️|🧑🏾‍✈️|🧑🏿‍✈️|👨🏻‍✈️|👨🏼‍✈️|👨🏽‍✈️|👨🏾‍✈️|👨🏿‍✈️|👩🏻‍✈️|👩🏼‍✈️|👩🏽‍✈️|👩🏾‍✈️|👩🏿‍✈️|🧑🏻‍🚀|🧑🏼‍🚀|🧑🏽‍🚀|🧑🏾‍🚀|🧑🏿‍🚀|👨🏻‍🚀|👨🏼‍🚀|👨🏽‍🚀|👨🏾‍🚀|👨🏿‍🚀|👩🏻‍🚀|👩🏼‍🚀|👩🏽‍🚀|👩🏾‍🚀|👩🏿‍🚀|🧑🏻‍🚒|🧑🏼‍🚒|🧑🏽‍🚒|🧑🏾‍🚒|🧑🏿‍🚒|👨🏻‍🚒|👨🏼‍🚒|👨🏽‍🚒|👨🏾‍🚒|👨🏿‍🚒|👩🏻‍🚒|👩🏼‍🚒|👩🏽‍🚒|👩🏾‍🚒|👩🏿‍🚒|👮🏻‍♂️|👮🏼‍♂️|👮🏽‍♂️|👮🏾‍♂️|👮🏿‍♂️|👮🏻‍♀️|👮🏼‍♀️|👮🏽‍♀️|👮🏾‍♀️|👮🏿‍♀️|🕵🏻‍♂️|🕵🏼‍♂️|🕵🏽‍♂️|🕵🏾‍♂️|🕵🏿‍♂️|🕵🏻‍♀️|🕵🏼‍♀️|🕵🏽‍♀️|🕵🏾‍♀️|🕵🏿‍♀️|💂🏻‍♂️|💂🏼‍♂️|💂🏽‍♂️|💂🏾‍♂️|💂🏿‍♂️|💂🏻‍♀️|💂🏼‍♀️|💂🏽‍♀️|💂🏾‍♀️|💂🏿‍♀️|👷🏻‍♂️|👷🏼‍♂️|👷🏽‍♂️|👷🏾‍♂️|👷🏿‍♂️|👷🏻‍♀️|👷🏼‍♀️|👷🏽‍♀️|👷🏾‍♀️|👷🏿‍♀️|👳🏻‍♂️|👳🏼‍♂️|👳🏽‍♂️|👳🏾‍♂️|👳🏿‍♂️|👳🏻‍♀️|👳🏼‍♀️|👳🏽‍♀️|👳🏾‍♀️|👳🏿‍♀️|🤵🏻‍♂️|🤵🏼‍♂️|🤵🏽‍♂️|🤵🏾‍♂️|🤵🏿‍♂️|🤵🏻‍♀️|🤵🏼‍♀️|🤵🏽‍♀️|🤵🏾‍♀️|🤵🏿‍♀️|👰🏻‍♂️|👰🏼‍♂️|👰🏽‍♂️|👰🏾‍♂️|👰🏿‍♂️|👰🏻‍♀️|👰🏼‍♀️|👰🏽‍♀️|👰🏾‍♀️|👰🏿‍♀️|👩🏻‍🍼|👩🏼‍🍼|👩🏽‍🍼|👩🏾‍🍼|👩🏿‍🍼|👨🏻‍🍼|👨🏼‍🍼|👨🏽‍🍼|👨🏾‍🍼|👨🏿‍🍼|🧑🏻‍🍼|🧑🏼‍🍼|🧑🏽‍🍼|🧑🏾‍🍼|🧑🏿‍🍼|🧑🏻‍🎄|🧑🏼‍🎄|🧑🏽‍🎄|🧑🏾‍🎄|🧑🏿‍🎄|🦸🏻‍♂️|🦸🏼‍♂️|🦸🏽‍♂️|🦸🏾‍♂️|🦸🏿‍♂️|🦸🏻‍♀️|🦸🏼‍♀️|🦸🏽‍♀️|🦸🏾‍♀️|🦸🏿‍♀️|🦹🏻‍♂️|🦹🏼‍♂️|🦹🏽‍♂️|🦹🏾‍♂️|🦹🏿‍♂️|🦹🏻‍♀️|🦹🏼‍♀️|🦹🏽‍♀️|🦹🏾‍♀️|🦹🏿‍♀️|🧙🏻‍♂️|🧙🏼‍♂️|🧙🏽‍♂️|🧙🏾‍♂️|🧙🏿‍♂️|🧙🏻‍♀️|🧙🏼‍♀️|🧙🏽‍♀️|🧙🏾‍♀️|🧙🏿‍♀️|🧚🏻‍♂️|🧚🏼‍♂️|🧚🏽‍♂️|🧚🏾‍♂️|🧚🏿‍♂️|🧚🏻‍♀️|🧚🏼‍♀️|🧚🏽‍♀️|🧚🏾‍♀️|🧚🏿‍♀️|🧛🏻‍♂️|🧛🏼‍♂️|🧛🏽‍♂️|🧛🏾‍♂️|🧛🏿‍♂️|🧛🏻‍♀️|🧛🏼‍♀️|🧛🏽‍♀️|🧛🏾‍♀️|🧛🏿‍♀️|🧜🏻‍♂️|🧜🏼‍♂️|🧜🏽‍♂️|🧜🏾‍♂️|🧜🏿‍♂️|🧜🏻‍♀️|🧜🏼‍♀️|🧜🏽‍♀️|🧜🏾‍♀️|🧜🏿‍♀️|🧝🏻‍♂️|🧝🏼‍♂️|🧝🏽‍♂️|🧝🏾‍♂️|🧝🏿‍♂️|🧝🏻‍♀️|🧝🏼‍♀️|🧝🏽‍♀️|🧝🏾‍♀️|🧝🏿‍♀️|💆🏻‍♂️|💆🏼‍♂️|💆🏽‍♂️|💆🏾‍♂️|💆🏿‍♂️|💆🏻‍♀️|💆🏼‍♀️|💆🏽‍♀️|💆🏾‍♀️|💆🏿‍♀️|💇🏻‍♂️|💇🏼‍♂️|💇🏽‍♂️|💇🏾‍♂️|💇🏿‍♂️|💇🏻‍♀️|💇🏼‍♀️|💇🏽‍♀️|💇🏾‍♀️|💇🏿‍♀️|🚶🏻‍♂️|🚶🏼‍♂️|🚶🏽‍♂️|🚶🏾‍♂️|🚶🏿‍♂️|🚶🏻‍♀️|🚶🏼‍♀️|🚶🏽‍♀️|🚶🏾‍♀️|🚶🏿‍♀️|🚶🏻‍➡️|🚶🏼‍➡️|🚶🏽‍➡️|🚶🏾‍➡️|🚶🏿‍➡️|🧍🏻‍♂️|🧍🏼‍♂️|🧍🏽‍♂️|🧍🏾‍♂️|🧍🏿‍♂️|🧍🏻‍♀️|🧍🏼‍♀️|🧍🏽‍♀️|🧍🏾‍♀️|🧍🏿‍♀️|🧎🏻‍♂️|🧎🏼‍♂️|🧎🏽‍♂️|🧎🏾‍♂️|🧎🏿‍♂️|🧎🏻‍♀️|🧎🏼‍♀️|🧎🏽‍♀️|🧎🏾‍♀️|🧎🏿‍♀️|🧎🏻‍➡️|🧎🏼‍➡️|🧎🏽‍➡️|🧎🏾‍➡️|🧎🏿‍➡️|🧑🏻‍🦯|🧑🏼‍🦯|🧑🏽‍🦯|🧑🏾‍🦯|🧑🏿‍🦯|👨🏻‍🦯|👨🏼‍🦯|👨🏽‍🦯|👨🏾‍🦯|👨🏿‍🦯|👩🏻‍🦯|👩🏼‍🦯|👩🏽‍🦯|👩🏾‍🦯|👩🏿‍🦯|🧑🏻‍🦼|🧑🏼‍🦼|🧑🏽‍🦼|🧑🏾‍🦼|🧑🏿‍🦼|👨🏻‍🦼|👨🏼‍🦼|👨🏽‍🦼|👨🏾‍🦼|👨🏿‍🦼|👩🏻‍🦼|👩🏼‍🦼|👩🏽‍🦼|👩🏾‍🦼|👩🏿‍🦼|🧑🏻‍🦽|🧑🏼‍🦽|🧑🏽‍🦽|🧑🏾‍🦽|🧑🏿‍🦽|👨🏻‍🦽|👨🏼‍🦽|👨🏽‍🦽|👨🏾‍🦽|👨🏿‍🦽|👩🏻‍🦽|👩🏼‍🦽|👩🏽‍🦽|👩🏾‍🦽|👩🏿‍🦽|🏃🏻‍♂️|🏃🏼‍♂️|🏃🏽‍♂️|🏃🏾‍♂️|🏃🏿‍♂️|🏃🏻‍♀️|🏃🏼‍♀️|🏃🏽‍♀️|🏃🏾‍♀️|🏃🏿‍♀️|🏃🏻‍➡️|🏃🏼‍➡️|🏃🏽‍➡️|🏃🏾‍➡️|🏃🏿‍➡️|🧖🏻‍♂️|🧖🏼‍♂️|🧖🏽‍♂️|🧖🏾‍♂️|🧖🏿‍♂️|🧖🏻‍♀️|🧖🏼‍♀️|🧖🏽‍♀️|🧖🏾‍♀️|🧖🏿‍♀️|🧗🏻‍♂️|🧗🏼‍♂️|🧗🏽‍♂️|🧗🏾‍♂️|🧗🏿‍♂️|🧗🏻‍♀️|🧗🏼‍♀️|🧗🏽‍♀️|🧗🏾‍♀️|🧗🏿‍♀️|🏌🏻‍♂️|🏌🏼‍♂️|🏌🏽‍♂️|🏌🏾‍♂️|🏌🏿‍♂️|🏌🏻‍♀️|🏌🏼‍♀️|🏌🏽‍♀️|🏌🏾‍♀️|🏌🏿‍♀️|🏄🏻‍♂️|🏄🏼‍♂️|🏄🏽‍♂️|🏄🏾‍♂️|🏄🏿‍♂️|🏄🏻‍♀️|🏄🏼‍♀️|🏄🏽‍♀️|🏄🏾‍♀️|🏄🏿‍♀️|🚣🏻‍♂️|🚣🏼‍♂️|🚣🏽‍♂️|🚣🏾‍♂️|🚣🏿‍♂️|🚣🏻‍♀️|🚣🏼‍♀️|🚣🏽‍♀️|🚣🏾‍♀️|🚣🏿‍♀️|🏊🏻‍♂️|🏊🏼‍♂️|🏊🏽‍♂️|🏊🏾‍♂️|🏊🏿‍♂️|🏊🏻‍♀️|🏊🏼‍♀️|🏊🏽‍♀️|🏊🏾‍♀️|🏊🏿‍♀️|🏋🏻‍♂️|🏋🏼‍♂️|🏋🏽‍♂️|🏋🏾‍♂️|🏋🏿‍♂️|🏋🏻‍♀️|🏋🏼‍♀️|🏋🏽‍♀️|🏋🏾‍♀️|🏋🏿‍♀️|🚴🏻‍♂️|🚴🏼‍♂️|🚴🏽‍♂️|🚴🏾‍♂️|🚴🏿‍♂️|🚴🏻‍♀️|🚴🏼‍♀️|🚴🏽‍♀️|🚴🏾‍♀️|🚴🏿‍♀️|🚵🏻‍♂️|🚵🏼‍♂️|🚵🏽‍♂️|🚵🏾‍♂️|🚵🏿‍♂️|🚵🏻‍♀️|🚵🏼‍♀️|🚵🏽‍♀️|🚵🏾‍♀️|🚵🏿‍♀️|🤸🏻‍♂️|🤸🏼‍♂️|🤸🏽‍♂️|🤸🏾‍♂️|🤸🏿‍♂️|🤸🏻‍♀️|🤸🏼‍♀️|🤸🏽‍♀️|🤸🏾‍♀️|🤸🏿‍♀️|🤽🏻‍♂️|🤽🏼‍♂️|🤽🏽‍♂️|🤽🏾‍♂️|🤽🏿‍♂️|🤽🏻‍♀️|🤽🏼‍♀️|🤽🏽‍♀️|🤽🏾‍♀️|🤽🏿‍♀️|🤾🏻‍♂️|🤾🏼‍♂️|🤾🏽‍♂️|🤾🏾‍♂️|🤾🏿‍♂️|🤾🏻‍♀️|🤾🏼‍♀️|🤾🏽‍♀️|🤾🏾‍♀️|🤾🏿‍♀️|🤹🏻‍♂️|🤹🏼‍♂️|🤹🏽‍♂️|🤹🏾‍♂️|🤹🏿‍♂️|🤹🏻‍♀️|🤹🏼‍♀️|🤹🏽‍♀️|🤹🏾‍♀️|🤹🏿‍♀️|🧘🏻‍♂️|🧘🏼‍♂️|🧘🏽‍♂️|🧘🏾‍♂️|🧘🏿‍♂️|🧘🏻‍♀️|🧘🏼‍♀️|🧘🏽‍♀️|🧘🏾‍♀️|🧘🏿‍♀️|😶‍🌫️|🕵️‍♂️|🕵️‍♀️|🏌️‍♂️|🏌️‍♀️|🏋️‍♂️|🏋️‍♀️|🏳️‍🌈|🏳️‍⚧️|⛹🏻‍♂️|⛹🏼‍♂️|⛹🏽‍♂️|⛹🏾‍♂️|⛹🏿‍♂️|⛹🏻‍♀️|⛹🏼‍♀️|⛹🏽‍♀️|⛹🏾‍♀️|⛹🏿‍♀️|😮‍💨|🙂‍↔️|🙂‍↕️|😵‍💫|❤️‍🔥|❤️‍🩹|🧔‍♂️|🧔‍♀️|👨‍🦰|👨‍🦱|👨‍🦳|👨‍🦲|👩‍🦰|🧑‍🦰|👩‍🦱|🧑‍🦱|👩‍🦳|🧑‍🦳|👩‍🦲|🧑‍🦲|👱‍♀️|👱‍♂️|🙍‍♂️|🙍‍♀️|🙎‍♂️|🙎‍♀️|🙅‍♂️|🙅‍♀️|🙆‍♂️|🙆‍♀️|💁‍♂️|💁‍♀️|🙋‍♂️|🙋‍♀️|🧏‍♂️|🧏‍♀️|🙇‍♂️|🙇‍♀️|🤦‍♂️|🤦‍♀️|🤷‍♂️|🤷‍♀️|🧑‍⚕️|👨‍⚕️|👩‍⚕️|🧑‍🎓|👨‍🎓|👩‍🎓|🧑‍🏫|👨‍🏫|👩‍🏫|🧑‍⚖️|👨‍⚖️|👩‍⚖️|🧑‍🌾|👨‍🌾|👩‍🌾|🧑‍🍳|👨‍🍳|👩‍🍳|🧑‍🔧|👨‍🔧|👩‍🔧|🧑‍🏭|👨‍🏭|👩‍🏭|🧑‍💼|👨‍💼|👩‍💼|🧑‍🔬|👨‍🔬|👩‍🔬|🧑‍💻|👨‍💻|👩‍💻|🧑‍🎤|👨‍🎤|👩‍🎤|🧑‍🎨|👨‍🎨|👩‍🎨|🧑‍✈️|👨‍✈️|👩‍✈️|🧑‍🚀|👨‍🚀|👩‍🚀|🧑‍🚒|👨‍🚒|👩‍🚒|👮‍♂️|👮‍♀️|💂‍♂️|💂‍♀️|👷‍♂️|👷‍♀️|👳‍♂️|👳‍♀️|🤵‍♂️|🤵‍♀️|👰‍♂️|👰‍♀️|👩‍🍼|👨‍🍼|🧑‍🍼|🧑‍🎄|🦸‍♂️|🦸‍♀️|🦹‍♂️|🦹‍♀️|🧙‍♂️|🧙‍♀️|🧚‍♂️|🧚‍♀️|🧛‍♂️|🧛‍♀️|🧜‍♂️|🧜‍♀️|🧝‍♂️|🧝‍♀️|🧞‍♂️|🧞‍♀️|🧟‍♂️|🧟‍♀️|💆‍♂️|💆‍♀️|💇‍♂️|💇‍♀️|🚶‍♂️|🚶‍♀️|🚶‍➡️|🧍‍♂️|🧍‍♀️|🧎‍♂️|🧎‍♀️|🧎‍➡️|🧑‍🦯|👨‍🦯|👩‍🦯|🧑‍🦼|👨‍🦼|👩‍🦼|🧑‍🦽|👨‍🦽|👩‍🦽|🏃‍♂️|🏃‍♀️|🏃‍➡️|👯‍♂️|👯‍♀️|🧖‍♂️|🧖‍♀️|🧗‍♂️|🧗‍♀️|🏄‍♂️|🏄‍♀️|🚣‍♂️|🚣‍♀️|🏊‍♂️|🏊‍♀️|⛹️‍♂️|⛹️‍♀️|🚴‍♂️|🚴‍♀️|🚵‍♂️|🚵‍♀️|🤸‍♂️|🤸‍♀️|🤼‍♂️|🤼‍♀️|🤽‍♂️|🤽‍♀️|🤾‍♂️|🤾‍♀️|🤹‍♂️|🤹‍♀️|🧘‍♂️|🧘‍♀️|👨‍👦|👨‍👧|👩‍👦|👩‍👧|🧑‍🧒|🐕‍🦺|🐻‍❄️|🐦‍🔥|🍋‍🟩|🍄‍🟫|⛓️‍💥|🏴‍☠️|🐈‍⬛|🐦‍⬛|🇦🇨|🇦🇩|🇦🇪|🇦🇫|🇦🇬|🇦🇮|🇦🇱|🇦🇲|🇦🇴|🇦🇶|🇦🇷|🇦🇸|🇦🇹|🇦🇺|🇦🇼|🇦🇽|🇦🇿|🇧🇦|🇧🇧|🇧🇩|🇧🇪|🇧🇫|🇧🇬|🇧🇭|🇧🇮|🇧🇯|🇧🇱|🇧🇲|🇧🇳|🇧🇴|🇧🇶|🇧🇷|🇧🇸|🇧🇹|🇧🇻|🇧🇼|🇧🇾|🇧🇿|🇨🇦|🇨🇨|🇨🇩|🇨🇫|🇨🇬|🇨🇭|🇨🇮|🇨🇰|🇨🇱|🇨🇲|🇨🇳|🇨🇴|🇨🇵|🇨🇷|🇨🇺|🇨🇻|🇨🇼|🇨🇽|🇨🇾|🇨🇿|🇩🇪|🇩🇬|🇩🇯|🇩🇰|🇩🇲|🇩🇴|🇩🇿|🇪🇦|🇪🇨|🇪🇪|🇪🇬|🇪🇭|🇪🇷|🇪🇸|🇪🇹|🇪🇺|🇫🇮|🇫🇯|🇫🇰|🇫🇲|🇫🇴|🇫🇷|🇬🇦|🇬🇧|🇬🇩|🇬🇪|🇬🇫|🇬🇬|🇬🇭|🇬🇮|🇬🇱|🇬🇲|🇬🇳|🇬🇵|🇬🇶|🇬🇷|🇬🇸|🇬🇹|🇬🇺|🇬🇼|🇬🇾|🇭🇰|🇭🇲|🇭🇳|🇭🇷|🇭🇹|🇭🇺|🇮🇨|🇮🇩|🇮🇪|🇮🇱|🇮🇲|🇮🇳|🇮🇴|🇮🇶|🇮🇷|🇮🇸|🇮🇹|🇯🇪|🇯🇲|🇯🇴|🇯🇵|🇰🇪|🇰🇬|🇰🇭|🇰🇮|🇰🇲|🇰🇳|🇰🇵|🇰🇷|🇰🇼|🇰🇾|🇰🇿|🇱🇦|🇱🇧|🇱🇨|🇱🇮|🇱🇰|🇱🇷|🇱🇸|🇱🇹|🇱🇺|🇱🇻|🇱🇾|🇲🇦|🇲🇨|🇲🇩|🇲🇪|🇲🇫|🇲🇬|🇲🇭|🇲🇰|🇲🇱|🇲🇲|🇲🇳|🇲🇴|🇲🇵|🇲🇶|🇲🇷|🇲🇸|🇲🇹|🇲🇺|🇲🇻|🇲🇼|🇲🇽|🇲🇾|🇲🇿|🇳🇦|🇳🇨|🇳🇪|🇳🇫|🇳🇬|🇳🇮|🇳🇱|🇳🇴|🇳🇵|🇳🇷|🇳🇺|🇳🇿|🇴🇲|🇵🇦|🇵🇪|🇵🇫|🇵🇬|🇵🇭|🇵🇰|🇵🇱|🇵🇲|🇵🇳|🇵🇷|🇵🇸|🇵🇹|🇵🇼|🇵🇾|🇶🇦|🇷🇪|🇷🇴|🇷🇸|🇷🇺|🇷🇼|🇸🇦|🇸🇧|🇸🇨|🇸🇩|🇸🇪|🇸🇬|🇸🇭|🇸🇮|🇸🇯|🇸🇰|🇸🇱|🇸🇲|🇸🇳|🇸🇴|🇸🇷|🇸🇸|🇸🇹|🇸🇻|🇸🇽|🇸🇾|🇸🇿|🇹🇦|🇹🇨|🇹🇩|🇹🇫|🇹🇬|🇹🇭|🇹🇯|🇹🇰|🇹🇱|🇹🇲|🇹🇳|🇹🇴|🇹🇷|🇹🇹|🇹🇻|🇹🇼|🇹🇿|🇺🇦|🇺🇬|🇺🇲|🇺🇳|🇺🇸|🇺🇾|🇺🇿|🇻🇦|🇻🇨|🇻🇪|🇻🇬|🇻🇮|🇻🇳|🇻🇺|🇼🇫|🇼🇸|🇽🇰|🇾🇪|🇾🇹|🇿🇦|🇿🇲|🇿🇼|👋🏻|👋🏼|👋🏽|👋🏾|👋🏿|🤚🏻|🤚🏼|🤚🏽|🤚🏾|🤚🏿|🖐🏻|🖐🏼|🖐🏽|🖐🏾|🖐🏿|🖖🏻|🖖🏼|🖖🏽|🖖🏾|🖖🏿|🫱🏻|🫱🏼|🫱🏽|🫱🏾|🫱🏿|🫲🏻|🫲🏼|🫲🏽|🫲🏾|🫲🏿|🫳🏻|🫳🏼|🫳🏽|🫳🏾|🫳🏿|🫴🏻|🫴🏼|🫴🏽|🫴🏾|🫴🏿|🫷🏻|🫷🏼|🫷🏽|🫷🏾|🫷🏿|🫸🏻|🫸🏼|🫸🏽|🫸🏾|🫸🏿|👌🏻|👌🏼|👌🏽|👌🏾|👌🏿|🤌🏻|🤌🏼|🤌🏽|🤌🏾|🤌🏿|🤏🏻|🤏🏼|🤏🏽|🤏🏾|🤏🏿|🤞🏻|🤞🏼|🤞🏽|🤞🏾|🤞🏿|🫰🏻|🫰🏼|🫰🏽|🫰🏾|🫰🏿|🤟🏻|🤟🏼|🤟🏽|🤟🏾|🤟🏿|🤘🏻|🤘🏼|🤘🏽|🤘🏾|🤘🏿|🤙🏻|🤙🏼|🤙🏽|🤙🏾|🤙🏿|👈🏻|👈🏼|👈🏽|👈🏾|👈🏿|👉🏻|👉🏼|👉🏽|👉🏾|👉🏿|👆🏻|👆🏼|👆🏽|👆🏾|👆🏿|🖕🏻|🖕🏼|🖕🏽|🖕🏾|🖕🏿|👇🏻|👇🏼|👇🏽|👇🏾|👇🏿|🫵🏻|🫵🏼|🫵🏽|🫵🏾|🫵🏿|👍🏻|👍🏼|👍🏽|👍🏾|👍🏿|👎🏻|👎🏼|👎🏽|👎🏾|👎🏿|👊🏻|👊🏼|👊🏽|👊🏾|👊🏿|🤛🏻|🤛🏼|🤛🏽|🤛🏾|🤛🏿|🤜🏻|🤜🏼|🤜🏽|🤜🏾|🤜🏿|👏🏻|👏🏼|👏🏽|👏🏾|👏🏿|🙌🏻|🙌🏼|🙌🏽|🙌🏾|🙌🏿|🫶🏻|🫶🏼|🫶🏽|🫶🏾|🫶🏿|👐🏻|👐🏼|👐🏽|👐🏾|👐🏿|🤲🏻|🤲🏼|🤲🏽|🤲🏾|🤲🏿|🤝🏻|🤝🏼|🤝🏽|🤝🏾|🤝🏿|🙏🏻|🙏🏼|🙏🏽|🙏🏾|🙏🏿|💅🏻|💅🏼|💅🏽|💅🏾|💅🏿|🤳🏻|🤳🏼|🤳🏽|🤳🏾|🤳🏿|💪🏻|💪🏼|💪🏽|💪🏾|💪🏿|🦵🏻|🦵🏼|🦵🏽|🦵🏾|🦵🏿|🦶🏻|🦶🏼|🦶🏽|🦶🏾|🦶🏿|👂🏻|👂🏼|👂🏽|👂🏾|👂🏿|🦻🏻|🦻🏼|🦻🏽|🦻🏾|🦻🏿|👃🏻|👃🏼|👃🏽|👃🏾|👃🏿|👶🏻|👶🏼|👶🏽|👶🏾|👶🏿|🧒🏻|🧒🏼|🧒🏽|🧒🏾|🧒🏿|👦🏻|👦🏼|👦🏽|👦🏾|👦🏿|👧🏻|👧🏼|👧🏽|👧🏾|👧🏿|🧑🏻|🧑🏼|🧑🏽|🧑🏾|🧑🏿|👱🏻|👱🏼|👱🏽|👱🏾|👱🏿|👨🏻|👨🏼|👨🏽|👨🏾|👨🏿|🧔🏻|🧔🏼|🧔🏽|🧔🏾|🧔🏿|👩🏻|👩🏼|👩🏽|👩🏾|👩🏿|🧓🏻|🧓🏼|🧓🏽|🧓🏾|🧓🏿|👴🏻|👴🏼|👴🏽|👴🏾|👴🏿|👵🏻|👵🏼|👵🏽|👵🏾|👵🏿|🙍🏻|🙍🏼|🙍🏽|🙍🏾|🙍🏿|🙎🏻|🙎🏼|🙎🏽|🙎🏾|🙎🏿|🙅🏻|🙅🏼|🙅🏽|🙅🏾|🙅🏿|🙆🏻|🙆🏼|🙆🏽|🙆🏾|🙆🏿|💁🏻|💁🏼|💁🏽|💁🏾|💁🏿|🙋🏻|🙋🏼|🙋🏽|🙋🏾|🙋🏿|🧏🏻|🧏🏼|🧏🏽|🧏🏾|🧏🏿|🙇🏻|🙇🏼|🙇🏽|🙇🏾|🙇🏿|🤦🏻|🤦🏼|🤦🏽|🤦🏾|🤦🏿|🤷🏻|🤷🏼|🤷🏽|🤷🏾|🤷🏿|👮🏻|👮🏼|👮🏽|👮🏾|👮🏿|🕵🏻|🕵🏼|🕵🏽|🕵🏾|🕵🏿|💂🏻|💂🏼|💂🏽|💂🏾|💂🏿|🥷🏻|🥷🏼|🥷🏽|🥷🏾|🥷🏿|👷🏻|👷🏼|👷🏽|👷🏾|👷🏿|🫅🏻|🫅🏼|🫅🏽|🫅🏾|🫅🏿|🤴🏻|🤴🏼|🤴🏽|🤴🏾|🤴🏿|👸🏻|👸🏼|👸🏽|👸🏾|👸🏿|👳🏻|👳🏼|👳🏽|👳🏾|👳🏿|👲🏻|👲🏼|👲🏽|👲🏾|👲🏿|🧕🏻|🧕🏼|🧕🏽|🧕🏾|🧕🏿|🤵🏻|🤵🏼|🤵🏽|🤵🏾|🤵🏿|👰🏻|👰🏼|👰🏽|👰🏾|👰🏿|🤰🏻|🤰🏼|🤰🏽|🤰🏾|🤰🏿|🫃🏻|🫃🏼|🫃🏽|🫃🏾|🫃🏿|🫄🏻|🫄🏼|🫄🏽|🫄🏾|🫄🏿|🤱🏻|🤱🏼|🤱🏽|🤱🏾|🤱🏿|👼🏻|👼🏼|👼🏽|👼🏾|👼🏿|🎅🏻|🎅🏼|🎅🏽|🎅🏾|🎅🏿|🤶🏻|🤶🏼|🤶🏽|🤶🏾|🤶🏿|🦸🏻|🦸🏼|🦸🏽|🦸🏾|🦸🏿|🦹🏻|🦹🏼|🦹🏽|🦹🏾|🦹🏿|🧙🏻|🧙🏼|🧙🏽|🧙🏾|🧙🏿|🧚🏻|🧚🏼|🧚🏽|🧚🏾|🧚🏿|🧛🏻|🧛🏼|🧛🏽|🧛🏾|🧛🏿|🧜🏻|🧜🏼|🧜🏽|🧜🏾|🧜🏿|🧝🏻|🧝🏼|🧝🏽|🧝🏾|🧝🏿|💆🏻|💆🏼|💆🏽|💆🏾|💆🏿|💇🏻|💇🏼|💇🏽|💇🏾|💇🏿|🚶🏻|🚶🏼|🚶🏽|🚶🏾|🚶🏿|🧍🏻|🧍🏼|🧍🏽|🧍🏾|🧍🏿|🧎🏻|🧎🏼|🧎🏽|🧎🏾|🧎🏿|🏃🏻|🏃🏼|🏃🏽|🏃🏾|🏃🏿|💃🏻|💃🏼|💃🏽|💃🏾|💃🏿|🕺🏻|🕺🏼|🕺🏽|🕺🏾|🕺🏿|🕴🏻|🕴🏼|🕴🏽|🕴🏾|🕴🏿|🧖🏻|🧖🏼|🧖🏽|🧖🏾|🧖🏿|🧗🏻|🧗🏼|🧗🏽|🧗🏾|🧗🏿|🏇🏻|🏇🏼|🏇🏽|🏇🏾|🏇🏿|🏂🏻|🏂🏼|🏂🏽|🏂🏾|🏂🏿|🏌🏻|🏌🏼|🏌🏽|🏌🏾|🏌🏿|🏄🏻|🏄🏼|🏄🏽|🏄🏾|🏄🏿|🚣🏻|🚣🏼|🚣🏽|🚣🏾|🚣🏿|🏊🏻|🏊🏼|🏊🏽|🏊🏾|🏊🏿|🏋🏻|🏋🏼|🏋🏽|🏋🏾|🏋🏿|🚴🏻|🚴🏼|🚴🏽|🚴🏾|🚴🏿|🚵🏻|🚵🏼|🚵🏽|🚵🏾|🚵🏿|🤸🏻|🤸🏼|🤸🏽|🤸🏾|🤸🏿|🤽🏻|🤽🏼|🤽🏽|🤽🏾|🤽🏿|🤾🏻|🤾🏼|🤾🏽|🤾🏾|🤾🏿|🤹🏻|🤹🏼|🤹🏽|🤹🏾|🤹🏿|🧘🏻|🧘🏼|🧘🏽|🧘🏾|🧘🏿|🛀🏻|🛀🏼|🛀🏽|🛀🏾|🛀🏿|🛌🏻|🛌🏼|🛌🏽|🛌🏾|🛌🏿|👭🏻|👭🏼|👭🏽|👭🏾|👭🏿|👫🏻|👫🏼|👫🏽|👫🏾|👫🏿|👬🏻|👬🏼|👬🏽|👬🏾|👬🏿|💏🏻|💏🏼|💏🏽|💏🏾|💏🏿|💑🏻|💑🏼|💑🏽|💑🏾|💑🏿|#️⃣|\u002a️⃣|0️⃣|1️⃣|2️⃣|3️⃣|4️⃣|5️⃣|6️⃣|7️⃣|8️⃣|9️⃣|✋🏻|✋🏼|✋🏽|✋🏾|✋🏿|✌🏻|✌🏼|✌🏽|✌🏾|✌🏿|☝🏻|☝🏼|☝🏽|☝🏾|☝🏿|✊🏻|✊🏼|✊🏽|✊🏾|✊🏿|✍🏻|✍🏼|✍🏽|✍🏾|✍🏿|⛹🏻|⛹🏼|⛹🏽|⛹🏾|⛹🏿|😀|😃|😄|😁|😆|😅|🤣|😂|🙂|🙃|🫠|😉|😊|😇|🥰|😍|🤩|😘|😗|😚|😙|🥲|😋|😛|😜|🤪|😝|🤑|🤗|🤭|🫢|🫣|🤫|🤔|🫡|🤐|🤨|😐|😑|😶|🫥|😏|😒|🙄|😬|🤥|🫨|😌|😔|😪|🤤|😴|😷|🤒|🤕|🤢|🤮|🤧|🥵|🥶|🥴|😵|🤯|🤠|🥳|🥸|😎|🤓|🧐|😕|🫤|😟|🙁|😮|😯|😲|😳|🥺|🥹|😦|😧|😨|😰|😥|😢|😭|😱|😖|😣|😞|😓|😩|😫|🥱|😤|😡|😠|🤬|😈|👿|💀|💩|🤡|👹|👺|👻|👽|👾|🤖|😺|😸|😹|😻|😼|😽|🙀|😿|😾|🙈|🙉|🙊|💌|💘|💝|💖|💗|💓|💞|💕|💟|💔|🩷|🧡|💛|💚|💙|🩵|💜|🤎|🖤|🩶|🤍|💋|💯|💢|💥|💫|💦|💨|🕳|💬|🗨|🗯|💭|💤|👋|🤚|🖐|🖖|🫱|🫲|🫳|🫴|🫷|🫸|👌|🤌|🤏|🤞|🫰|🤟|🤘|🤙|👈|👉|👆|🖕|👇|🫵|👍|👎|👊|🤛|🤜|👏|🙌|🫶|👐|🤲|🤝|🙏|💅|🤳|💪|🦾|🦿|🦵|🦶|👂|🦻|👃|🧠|🫀|🫁|🦷|🦴|👀|👁|👅|👄|🫦|👶|🧒|👦|👧|🧑|👱|👨|🧔|👩|🧓|👴|👵|🙍|🙎|🙅|🙆|💁|🙋|🧏|🙇|🤦|🤷|👮|🕵|💂|🥷|👷|🫅|🤴|👸|👳|👲|🧕|🤵|👰|🤰|🫃|🫄|🤱|👼|🎅|🤶|🦸|🦹|🧙|🧚|🧛|🧜|🧝|🧞|🧟|🧌|💆|💇|🚶|🧍|🧎|🏃|💃|🕺|🕴|👯|🧖|🧗|🤺|🏇|🏂|🏌|🏄|🚣|🏊|🏋|🚴|🚵|🤸|🤼|🤽|🤾|🤹|🧘|🛀|🛌|👭|👫|👬|💏|💑|🗣|👤|👥|🫂|👪|👣|🦰|🦱|🦳|🦲|🐵|🐒|🦍|🦧|🐶|🐕|🦮|🐩|🐺|🦊|🦝|🐱|🐈|🦁|🐯|🐅|🐆|🐴|🫎|🫏|🐎|🦄|🦓|🦌|🦬|🐮|🐂|🐃|🐄|🐷|🐖|🐗|🐽|🐏|🐑|🐐|🐪|🐫|🦙|🦒|🐘|🦣|🦏|🦛|🐭|🐁|🐀|🐹|🐰|🐇|🐿|🦫|🦔|🦇|🐻|🐨|🐼|🦥|🦦|🦨|🦘|🦡|🐾|🦃|🐔|🐓|🐣|🐤|🐥|🐦|🐧|🕊|🦅|🦆|🦢|🦉|🦤|🪶|🦩|🦚|🦜|🪽|🪿|🐸|🐊|🐢|🦎|🐍|🐲|🐉|🦕|🦖|🐳|🐋|🐬|🦭|🐟|🐠|🐡|🦈|🐙|🐚|🪸|🪼|🐌|🦋|🐛|🐜|🐝|🪲|🐞|🦗|🪳|🕷|🕸|🦂|🦟|🪰|🪱|🦠|💐|🌸|💮|🪷|🏵|🌹|🥀|🌺|🌻|🌼|🌷|🪻|🌱|🪴|🌲|🌳|🌴|🌵|🌾|🌿|🍀|🍁|🍂|🍃|🪹|🪺|🍄|🍇|🍈|🍉|🍊|🍋|🍌|🍍|🥭|🍎|🍏|🍐|🍑|🍒|🍓|🫐|🥝|🍅|🫒|🥥|🥑|🍆|🥔|🥕|🌽|🌶|🫑|🥒|🥬|🥦|🧄|🧅|🥜|🫘|🌰|🫚|🫛|🍞|🥐|🥖|🫓|🥨|🥯|🥞|🧇|🧀|🍖|🍗|🥩|🥓|🍔|🍟|🍕|🌭|🥪|🌮|🌯|🫔|🥙|🧆|🥚|🍳|🥘|🍲|🫕|🥣|🥗|🍿|🧈|🧂|🥫|🍱|🍘|🍙|🍚|🍛|🍜|🍝|🍠|🍢|🍣|🍤|🍥|🥮|🍡|🥟|🥠|🥡|🦀|🦞|🦐|🦑|🦪|🍦|🍧|🍨|🍩|🍪|🎂|🍰|🧁|🥧|🍫|🍬|🍭|🍮|🍯|🍼|🥛|🫖|🍵|🍶|🍾|🍷|🍸|🍹|🍺|🍻|🥂|🥃|🫗|🥤|🧋|🧃|🧉|🧊|🥢|🍽|🍴|🥄|🔪|🫙|🏺|🌍|🌎|🌏|🌐|🗺|🗾|🧭|🏔|🌋|🗻|🏕|🏖|🏜|🏝|🏞|🏟|🏛|🏗|🧱|🪨|🪵|🛖|🏘|🏚|🏠|🏡|🏢|🏣|🏤|🏥|🏦|🏨|🏩|🏪|🏫|🏬|🏭|🏯|🏰|💒|🗼|🗽|🕌|🛕|🕍|🕋|🌁|🌃|🏙|🌄|🌅|🌆|🌇|🌉|🎠|🛝|🎡|🎢|💈|🎪|🚂|🚃|🚄|🚅|🚆|🚇|🚈|🚉|🚊|🚝|🚞|🚋|🚌|🚍|🚎|🚐|🚑|🚒|🚓|🚔|🚕|🚖|🚗|🚘|🚙|🛻|🚚|🚛|🚜|🏎|🏍|🛵|🦽|🦼|🛺|🚲|🛴|🛹|🛼|🚏|🛣|🛤|🛢|🛞|🚨|🚥|🚦|🛑|🚧|🛟|🛶|🚤|🛳|🛥|🚢|🛩|🛫|🛬|🪂|💺|🚁|🚟|🚠|🚡|🛰|🚀|🛸|🛎|🧳|🕰|🕛|🕧|🕐|🕜|🕑|🕝|🕒|🕞|🕓|🕟|🕔|🕠|🕕|🕡|🕖|🕢|🕗|🕣|🕘|🕤|🕙|🕥|🕚|🕦|🌑|🌒|🌓|🌔|🌕|🌖|🌗|🌘|🌙|🌚|🌛|🌜|🌡|🌝|🌞|🪐|🌟|🌠|🌌|🌤|🌥|🌦|🌧|🌨|🌩|🌪|🌫|🌬|🌀|🌈|🌂|🔥|💧|🌊|🎃|🎄|🎆|🎇|🧨|🎈|🎉|🎊|🎋|🎍|🎎|🎏|🎐|🎑|🧧|🎀|🎁|🎗|🎟|🎫|🎖|🏆|🏅|🥇|🥈|🥉|🥎|🏀|🏐|🏈|🏉|🎾|🥏|🎳|🏏|🏑|🏒|🥍|🏓|🏸|🥊|🥋|🥅|🎣|🤿|🎽|🎿|🛷|🥌|🎯|🪀|🪁|🔫|🎱|🔮|🪄|🎮|🕹|🎰|🎲|🧩|🧸|🪅|🪩|🪆|🃏|🀄|🎴|🎭|🖼|🎨|🧵|🪡|🧶|🪢|👓|🕶|🥽|🥼|🦺|👔|👕|👖|🧣|🧤|🧥|🧦|👗|👘|🥻|🩱|🩲|🩳|👙|👚|🪭|👛|👜|👝|🛍|🎒|🩴|👞|👟|🥾|🥿|👠|👡|🩰|👢|🪮|👑|👒|🎩|🎓|🧢|🪖|📿|💄|💍|💎|🔇|🔈|🔉|🔊|📢|📣|📯|🔔|🔕|🎼|🎵|🎶|🎙|🎚|🎛|🎤|🎧|📻|🎷|🪗|🎸|🎹|🎺|🎻|🪕|🥁|🪘|🪇|🪈|📱|📲|📞|📟|📠|🔋|🪫|🔌|💻|🖥|🖨|🖱|🖲|💽|💾|💿|📀|🧮|🎥|🎞|📽|🎬|📺|📷|📸|📹|📼|🔍|🔎|🕯|💡|🔦|🏮|🪔|📔|📕|📖|📗|📘|📙|📚|📓|📒|📃|📜|📄|📰|🗞|📑|🔖|🏷|💰|🪙|💴|💵|💶|💷|💸|💳|🧾|💹|📧|📨|📩|📤|📥|📦|📫|📪|📬|📭|📮|🗳|🖋|🖊|🖌|🖍|📝|💼|📁|📂|🗂|📅|📆|🗒|🗓|📇|📈|📉|📊|📋|📌|📍|📎|🖇|📏|📐|🗃|🗄|🗑|🔒|🔓|🔏|🔐|🔑|🗝|🔨|🪓|🛠|🗡|💣|🪃|🏹|🛡|🪚|🔧|🪛|🔩|🗜|🦯|🔗|🪝|🧰|🧲|🪜|🧪|🧫|🧬|🔬|🔭|📡|💉|🩸|💊|🩹|🩼|🩺|🩻|🚪|🛗|🪞|🪟|🛏|🛋|🪑|🚽|🪠|🚿|🛁|🪤|🪒|🧴|🧷|🧹|🧺|🧻|🪣|🧼|🫧|🪥|🧽|🧯|🛒|🚬|🪦|🧿|🪬|🗿|🪧|🪪|🏧|🚮|🚰|🚹|🚺|🚻|🚼|🚾|🛂|🛃|🛄|🛅|🚸|🚫|🚳|🚭|🚯|🚱|🚷|📵|🔞|🔃|🔄|🔙|🔚|🔛|🔜|🔝|🛐|🕉|🕎|🔯|🪯|🔀|🔁|🔂|🔼|🔽|🎦|🔅|🔆|📶|🛜|📳|📴|🟰|💱|💲|🔱|📛|🔰|🔟|🔠|🔡|🔢|🔣|🔤|🅰|🆎|🅱|🆑|🆒|🆓|🆔|🆕|🆖|🅾|🆗|🅿|🆘|🆙|🆚|🈁|🈂|🈷|🈶|🈯|🉐|🈹|🈚|🈲|🉑|🈸|🈴|🈳|🈺|🈵|🔴|🟠|🟡|🟢|🔵|🟣|🟤|🟥|🟧|🟨|🟩|🟦|🟪|🟫|🔶|🔷|🔸|🔹|🔺|🔻|💠|🔘|🔳|🔲|🏁|🚩|🎌|🏴|🏳|🏻|🏼|🏽|🏾|🏿|☺|☹|☠|❣|❤|✋|✌|☝|✊|✍|⛷|⛹|☘|☕|⛰|⛪|⛩|⛲|⛺|♨|⛽|⚓|⛵|⛴|✈|⌛|⏳|⌚|⏰|⏱|⏲|☀|⭐|☁|⛅|⛈|☂|☔|⛱|⚡|❄|☃|⛄|☄|✨|⚽|⚾|⛳|⛸|♠|♥|♦|♣|♟|⛑|☎|⌨|✉|✏|✒|✂|⛏|⚒|⚔|⚙|⚖|⛓|⚗|⚰|⚱|♿|⚠|⛔|☢|☣|⬆|↗|➡|↘|⬇|↙|⬅|↖|↕|↔|↩|↪|⤴|⤵|⚛|✡|☸|☯|✝|☦|☪|☮|♈|♉|♊|♋|♌|♍|♎|♏|♐|♑|♒|♓|⛎|▶|⏩|⏭|⏯|◀|⏪|⏮|⏫|⏬|⏸|⏹|⏺|⏏|♀|♂|⚧|✖|➕|➖|➗|♾|‼|⁉|❓|❔|❕|❗|〰|⚕|♻|⚜|⭕|✅|☑|✔|❌|❎|➰|➿|〽|✳|✴|❇|©|®|™|ℹ|Ⓜ|㊗|㊙|⚫|⚪|⬛|⬜|◼|◻|◾|◽|▪|▫"#;
