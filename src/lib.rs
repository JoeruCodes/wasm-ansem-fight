#[warn(unreachable_code)]
mod config;
mod helpers;

use std::time::Duration;

use config::{Characters, PunchTiers, DODGE_PROBS, IMAGE_SETS, PLAY_DODGE_SOUND_AT, PLAY_PUNCH_SOUNDS_AT, PLAY_PWRUP_SOUND_AT, SOUNDS, WIN_PUNCHES};
use ::futures::channel::oneshot;
use helpers::play_sound;
use tokio_with_wasm::tokio::time::sleep;
use wasm_bindgen::prelude::*;
use rand::Rng;
use web_sys::{AudioContext, Event, HtmlAudioElement};
use crate::{config::PUNCHES_CONFIG, helpers::{document_get_element_by_id, generate_punches, shuffle_array}};
use wasm_bindgen_futures::JsFuture;
#[wasm_bindgen]
pub struct Game{
    player: Characters,
    tier: PunchTiers,
    npunches: usize,
    doges: usize,
    lpunches: usize,
    render_buf: Vec<String>,
    temp_render_buf: Vec<String>,
    image_ref: web_sys::HtmlImageElement,
}

impl Game{
    pub fn new(player: &str, wif: f64) -> Game {
        assert!(wif > 0.0, "WIF must be greater than 0");
    
        let punch_config;
        let tier;
        if wif <= 1.0 {
            punch_config = &PUNCHES_CONFIG[0];
            tier = PunchTiers::T1;
        } else if wif < 41.0 {
            punch_config = &PUNCHES_CONFIG[1];
            tier = PunchTiers::T2;
        } else {
            punch_config = &PUNCHES_CONFIG[2];
            tier = PunchTiers::T3;
        }
    
        let player_e = match player {
            "ansem" => Characters::ANSEM,
            "kook" => Characters::COOK,
            _ => panic!("Invalid player name: {}", player),
        };
    
        Game {
            player: player_e,
            tier,
            npunches: generate_punches(&punch_config.min_punches, &punch_config.max_punches),
            doges: 0,
            lpunches: 0,
            render_buf: {
                if let PunchTiers::T3 = tier{
                    match player_e{
                        Characters::ANSEM => PUNCHES_CONFIG[1].image_arr_p1.into_iter().map(|x| x.to_string()).collect(),
                        Characters::COOK => PUNCHES_CONFIG[1].image_arr_p2.into_iter().map(|x| x.to_string()).collect()
                    }
                }else{
                    match player_e{
                        Characters::ANSEM => punch_config.image_arr_p1.into_iter().map(|x| x.to_string()).collect(),
                        Characters::COOK => punch_config.image_arr_p2.into_iter().map(|x| x.to_string()).collect()
                    }
                }
            },
            temp_render_buf: {
                if let PunchTiers::T3 = tier{
                    match player_e{
                        Characters::ANSEM => punch_config.image_arr_p1.into_iter().map(|x| x.to_string()).collect(),
                        Characters::COOK => punch_config.image_arr_p2.into_iter().map(|x| x.to_string()).collect()
                    }
                }else{
                    vec![]
                }
            },
            image_ref: document_get_element_by_id(),
        }
    }


    pub fn randomize_punch_sequences(&mut self){
            let dodges = rand::thread_rng().gen::<f64>() < {
                match &self.tier{
                    PunchTiers::T1 => DODGE_PROBS.t1,
                    PunchTiers::T2 => DODGE_PROBS.t2,
                    PunchTiers::T3 => DODGE_PROBS.t2,
                }
            };
            if dodges{
                match &self.player{
                    Characters::ANSEM => {
                        if rand::thread_rng().gen::<f64>() < 0.5{
                            self.render_buf = IMAGE_SETS.ansem_dodge_1.into_iter().map(|x| x.to_string()).collect()
                        }else{
                            self.render_buf = IMAGE_SETS.ansem_dodge_2.into_iter().map(|x| x.to_string()).collect()
                        }
                    },
                    Characters::COOK => {
                        if rand::thread_rng().gen::<f64>() < 0.5{
                            self.render_buf = IMAGE_SETS.cook_dodge_1.into_iter().map(|x| x.to_string()).collect()
                        }else{
                            self.render_buf = IMAGE_SETS.cook_dodge_2.into_iter().map(|x| x.to_string()).collect()
                        }
                    }
                }
            }

            if self.tier == PunchTiers::T1 || self.tier == PunchTiers::T2{
                self.shuffle_punch_seq();
            }
        
    }
    pub fn shuffle_punch_seq(&mut self){
        let mut rng = rand::thread_rng();
        let num_punches: usize = if rng.gen::<f64>() < 0.5 { 1 } else { 2 };
        let mut shuffled = shuffle_array(&mut self.render_buf[1..]).to_vec();
        shuffled.truncate(num_punches);
        self.render_buf = vec![self.render_buf[0].to_owned()];
        self.render_buf.extend_from_slice(&shuffled);
    }
    pub fn set_frame(&self, path: &str){
        self.image_ref.set_src(&format!("{}/{}", "/src/assets", path));
    }
    pub fn flip_frame(&self, bool: bool){
        if bool{
            self.image_ref.set_class_name("scale-x-[-1]")
        }else{
            self.image_ref.set_class_name("");
        }
    }
    pub async fn cleanup(&mut self) {
        self.flip_frame(false);
        let winlose = self.npunches > WIN_PUNCHES;
        if winlose {
            match &self.player{
                Characters::ANSEM => {
                    self.set_frame(IMAGE_SETS.result_ansem[1]);
                },
                Characters::COOK => {
                    self.set_frame(IMAGE_SETS.result_cook[1]);
                }
            }
            play_sound(&SOUNDS.punch).await;
            play_sound(&SOUNDS.win).await;
        }else{
            match &self.player{
                Characters::ANSEM => {
                    self.set_frame(IMAGE_SETS.result_ansem[0]);
                },
                Characters::COOK => {
                    self.set_frame(IMAGE_SETS.result_cook[0]);
                }
            }
            play_sound(&SOUNDS.punch).await;
            play_sound(&SOUNDS.lose).await;
        }
        sleep(Duration::from_millis(50)).await;
        self.set_frame(IMAGE_SETS.default[0]);
        if !self.render_buf.is_empty() {
            self.render_buf.clear();
        }
    }

    pub async fn render_sequence(&mut self) {
        for i in 0..self.render_buf.len() {

//========= TODO: FIX FLIP IMAGES FOR KOOK =========================================================================
            if self.render_buf[i] == IMAGE_SETS.cook_t3[1] || self.render_buf[i] == IMAGE_SETS.result_cook[1] {
                self.flip_frame(false);
            } else if self.render_buf[i] == IMAGE_SETS.ansem_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.ansem_dodge_2[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_2[1] {
                self.flip_frame(false);
            } else if self.player == Characters::COOK {
                self.flip_frame(true);
            }
// ==================================================================================================================

            if self.image_ref.src() != self.render_buf[i] {
                self.set_frame(&self.render_buf[i]);
            }

            if PLAY_PUNCH_SOUNDS_AT.contains(&self.render_buf[i].as_str()){
                play_sound(&SOUNDS.punch).await
            }else if PLAY_DODGE_SOUND_AT.contains(&self.render_buf[i].as_str()){
                play_sound(&SOUNDS.dodge).await
            }else if PLAY_PWRUP_SOUND_AT.contains(&self.render_buf[i].as_str()){
                play_sound(&SOUNDS.tier3).await
            }

            sleep(Duration::from_millis(200)).await;
        }
    }
    
    
}
#[wasm_bindgen]
pub struct Ret{
    npunches: usize,
    wif: f64
}
#[wasm_bindgen]
impl Game{
    pub async fn render(player: &str, wif: f64) -> Ret {
        if player.is_empty() || wif <= 0.0 {
            panic!("Invalid input parameters");
        }
        let mut game = Game::new(player, wif);
        if game.player == Characters::COOK{
            game.flip_frame(true);
        }
        for i in 0..game.npunches {
            if game.tier ==PunchTiers::T3 && i==game.npunches - 1{
                game.render_buf = game.render_buf.to_owned();
                game.temp_render_buf.clear();
            }
            if game.render_buf != IMAGE_SETS.cook_dodge_1 && 
               game.render_buf != IMAGE_SETS.cook_dodge_2 && 
               game.render_buf != IMAGE_SETS.ansem_dodge_1 && 
               game.render_buf != IMAGE_SETS.ansem_dodge_2 && 
               game.render_buf != IMAGE_SETS.ansem_t3 && 
               game.render_buf != IMAGE_SETS.cook_t3 {
                game.randomize_punch_sequences();
            }
            game.render_sequence().await;
        }
        
        game.cleanup().await;

        //ret is not needed do direct DOM manipulation TODO: get rid of this
        Ret{npunches: game.npunches, wif}
    }

}

