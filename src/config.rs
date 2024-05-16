#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SoundTypes {
    PUNCH,
    WIN,
    LOSE,
    BELL,
    TIER3,
    Dodge,
}

// Define struct for storing image sets
#[derive(Debug)]
pub struct ImageSets {
    pub ansem_t1: [&'static str; 2],
    pub ansem_t2: [&'static str; 3],
    pub ansem_t3: [&'static str; 3],
    pub cook_t1: [&'static str; 2],
    pub cook_t2: [&'static str; 3],
    pub cook_t3: [&'static str; 3],
    pub cook_dodge_1: [&'static str; 3],
    pub cook_dodge_2: [&'static str; 3],
    pub ansem_dodge_1: [&'static str; 3],
    pub ansem_dodge_2: [&'static str; 3],
    pub default: [&'static str; 3],
    pub result_ansem: [&'static str; 2],
    pub result_cook: [&'static str; 2],
}

// Define struct for storing sounds
#[derive(Debug)]
pub struct Sounds {
    pub punch: &'static str,
    pub win: &'static str,
    pub lose: &'static str,
    pub bell: &'static str,
    pub tier3: &'static str,
    pub dodge: &'static str,
    pub background: &'static str,
}

// Define struct for storing dodge probabilities
#[derive(Debug)]
pub struct DodgeProbs {
    pub t1: f64,
    pub t2: f64,
    pub t3: f64,
}

// Define struct for punches configuration
#[derive(Debug)]
pub struct PunchesConfig {
    pub min_punches: usize,
    pub max_punches: usize,
    pub image_arr_p1: &'static [&'static str],
    pub image_arr_p2: &'static [&'static str],
}

// Define constants
pub const SPEED: usize = 2;
pub const WIN_PUNCHES: usize = 13;
#[derive(PartialEq)]
pub enum PunchTiers{
    T1,
    T2,
    T3
}
// Define data
pub const IMAGE_SETS: ImageSets = ImageSets {
    ansem_t1: ["../assets/ansemPunch", "../assets/t1ansemPunch"],
    ansem_t2: ["../assets/ansemPunch", "../assets/t1ansemPunch", "../assets/t2ansemPunch"],
    ansem_t3: ["../assets/ansemPunch", "../assets/t3ansemPunch", "../assets/upansemPunch"],
    cook_t1: ["../assets/ansemPunch", "../assets/opponent_t1"],
    cook_t2: ["../assets/ansemPunch", "../assets/opponent_t1", "../assets/opponent_t2"],
    cook_t3: ["../assets/ansemPunch", "../assets/cook_t3_pwrup", "../assets/t3_cook_win"],
    cook_dodge_1: [
        "../assets/ansemPunch",
        "../assets/cook_dodge_1", "../assets/t1ansemPunch",
    ],
    cook_dodge_2: [
        "../assets/ansemPunch",
        "../assets/cook_dodge_2", "../assets/t2ansemPunch",
    ],
    ansem_dodge_1: [
        "../assets/ansemPunch",
        "../assets/ansem_dodge_1", "../assets/opponent_t1",
    ],
    ansem_dodge_2: [
        "../assets/ansemPunch",
        "../assets/ansem_dodge_2", "../assets/opponent_t2",
    ],
    default: ["../assets/ansem", "../assets/ansemPunch", "../assets/t1ansemPunch"],
    result_ansem: ["../assets/loseImage", "../assets/winImage"],
    result_cook: ["../assets/loseImage_cook", "../assets/t3_cook_win"],
};

pub const SOUNDS: Sounds = Sounds {
    punch: "../assets/punchSound",
    win: "../assets/winSound",
    lose: "../assets/loseSound",
    bell: "../assets/bellSound",
    tier3: "../assets/t3Sound",
    dodge: "../assets/dodge",
    background: "../assets/bgSound",
};

pub const DODGE_PROBS: DodgeProbs = DodgeProbs {
    t1: 0.4,
    t2: 0.3,
    t3: 0.2,
};

pub const PUNCHES_CONFIG: [PunchesConfig; 3] = [
    PunchesConfig {
        min_punches: 1,
        max_punches: 6,
        image_arr_p1: &IMAGE_SETS.ansem_t1,
        image_arr_p2: &IMAGE_SETS.cook_t1,
    },
    PunchesConfig {
        min_punches: 9,
        max_punches: 16,
        image_arr_p1: &IMAGE_SETS.ansem_t2,
        image_arr_p2: &IMAGE_SETS.cook_t2,
    },
    PunchesConfig {
        min_punches: 17,
        max_punches: 24,
        image_arr_p1: &IMAGE_SETS.ansem_t3,
        image_arr_p2: &IMAGE_SETS.cook_t3,
    },
];

#[derive(Clone, Copy, PartialEq)]
pub enum Characters{
    ANSEM,
    COOK
}