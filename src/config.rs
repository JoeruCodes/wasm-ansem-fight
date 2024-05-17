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
const ansem :&str = "../assets/start.png";
const ansemPunch :&str = "../assets/idlee.png";
const t1ansemPunch :&str = "../assets/T1-Ansem-Punch2.png";
const t2ansemPunch :&str = "../assets/Tier_22.png";
const t3ansemPunch :&str = "../assets/t33.png";
const upansemPunch :&str = "../assets/uppercut.png";
const winImage :&str = "../assets/win.png";
const loseImage :&str = "../assets/lose.png";
const punchSound :&str = "../assets/punch1.m4a";
const winSound :&str = "../assets/win.m4a";
const loseSound :&str = "../assets/lose.m4a";
const bellSound :&str = "../assets/bell.m4a";
const t3Sound :&str = "../assets/tier3powerup1.m4a";
const bgSound :&str = "../assets/background.mp3";
const opponent_t1 :&str = "../assets/cook_punch_t1.png";
const opponent_t2 :&str = "../assets/cook_punch_t2.png";
const cook_dodge_1 :&str = "../assets/dodge_1_rev.png";
const cook_dodge_2 :&str = "../assets/dodge_2_rev.png";
const ansem_dodge_1 :&str = "../assets/dodge_1.png";
const ansem_dodge_2 :&str = "../assets/dodge_2.png";
const loseImage_cook :&str = "../assets/lose_cook.png";
const winImage_cook :&str = "../assets/win_cook.png";
const cook_t3_pwrup :&str = "../assets/t33_rev.png";
const t3_cook_win :&str = "../assets/t3_cook_win.png";
const dodge :&str = "../assets/dodge.mp3";
// Define data
pub const IMAGE_SETS: ImageSets = ImageSets {
    ansem_t1: [ansemPunch, t1ansemPunch],
    ansem_t2: [ansemPunch, t1ansemPunch, t2ansemPunch],
    ansem_t3: [ansemPunch, t3ansemPunch, upansemPunch],
    cook_t1: [ansemPunch, opponent_t1],
    cook_t2: [ansemPunch, opponent_t1, opponent_t2],
    cook_t3: [ansemPunch, cook_t3_pwrup, t3_cook_win],
    cook_dodge_1: [
      ansemPunch,
      cook_dodge_1,
      t1ansemPunch,
    ],
    cook_dodge_2: [
      ansemPunch,
      cook_dodge_2,
      t2ansemPunch,
    ],
    ansem_dodge_1: [
      ansemPunch,
      ansem_dodge_1,
      opponent_t1,
    ],
    ansem_dodge_2: [
      ansemPunch,
      ansem_dodge_2,
      opponent_t2,
    ],
    default: [ansem, ansemPunch, t1ansemPunch],
    result_ansem: [loseImage, winImage],
    result_cook: [loseImage_cook, t3_cook_win],
};

pub const SOUNDS: Sounds = Sounds {
    punch: &punchSound,
    win: &winSound,
    lose: &loseSound,
    bell: &bellSound,
    tier3: &t3Sound,
    dodge: &dodge,
    background: &bgSound,
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
pub const PLAY_PUNCH_SOUNDS_AT:[&str; 6] = [t1ansemPunch, t2ansemPunch, upansemPunch, opponent_t1, opponent_t2, t3_cook_win]; 
pub const PLAY_DODGE_SOUND_AT:[&str; 4] = [cook_dodge_1, cook_dodge_2, ansem_dodge_1, ansem_dodge_2];
pub const PLAY_PWRUP_SOUND_AT:[&str;2] = [t3ansemPunch, cook_t3_pwrup];

#[derive(Clone, Copy, PartialEq)]
pub enum Characters {
    ANSEM,
    COOK,
}
#[derive(Clone, Copy, PartialEq)]
pub enum PunchTiers {
    T1,
    T2,
    T3,
}
