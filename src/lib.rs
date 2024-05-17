#[warn(unreachable_code)]
mod config;
mod helpers;

use std::time::Duration;

use config::{Characters, PunchTiers, DODGE_PROBS, IMAGE_SETS, PLAY_DODGE_SOUND_AT, PLAY_PUNCH_SOUNDS_AT, PLAY_PWRUP_SOUND_AT, SOUNDS, WIN_PUNCHES};
use ::futures::channel::oneshot;
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
    image_ref: web_sys::HtmlImageElement,
    audio: AudioOps
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
            render_buf: match player_e {
                Characters::ANSEM => punch_config.image_arr_p1.into_iter().map(|x| x.to_string()).collect(),
                Characters::COOK => punch_config.image_arr_p2.into_iter().map(|x| x.to_string()).collect(),
            },
            image_ref: document_get_element_by_id(),
            audio: AudioOps::new(),
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
        web_sys::console::log_1(&JsValue::from_str("Setting flip className"));
        self.image_ref.set_class_name({
            if bool{
                "scale-x-[-1]"
            }else{
                ""
            }
        });
    }
    pub fn cleanup(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("Cleanup started"));

        web_sys::console::log_1(&JsValue::from_str("Setting to win or lose frame"));
        self.set_frame({
            if self.npunches > WIN_PUNCHES{
                match &self.player{
                    Characters::ANSEM => IMAGE_SETS.result_ansem[1],
                    Characters::COOK => IMAGE_SETS.result_cook[1]
                }
            }else{
                match &self.player{
                    Characters::ANSEM => IMAGE_SETS.result_ansem[0],
                    Characters::COOK => IMAGE_SETS.result_cook[0]
                }
            }
        });
        web_sys::console::log_1(&JsValue::from_str("Setting to default frame"));
        self.set_frame(IMAGE_SETS.default[0]);
        // Clear punch sequences
        if !self.render_buf.is_empty() {
            web_sys::console::log_1(&JsValue::from_str("Clearing punch sequences"));
            self.render_buf.clear();
        } else {
            web_sys::console::log_1(&JsValue::from_str("No punch sequences to clear"));
        }

        // Reset audio
        web_sys::console::log_1(&JsValue::from_str("Resetting audio"));
        match self.audio.audio_context.close() {
            Ok(_) => web_sys::console::log_1(&JsValue::from_str("Audio context closed successfully")),
            Err(e) => web_sys::console::log_1(&JsValue::from_str(&format!("Error closing audio context: {:?}", e))),
        }

        web_sys::console::log_1(&JsValue::from_str("Cleanup finished"));
    }

    pub async fn render_sequence(&mut self) {
        web_sys::console::log_1(&JsValue::from_str("Starting render_sequence"));
        for i in 0..self.render_buf.len() {
            web_sys::console::log_1(&JsValue::from_str(&format!("Rendering frame: {}", self.render_buf[i])));

            //TODO: FIX FLIP IMAGES FOR KOOK


            // web_sys::console::log_1(&JsValue::from_str("Flipping frames"));
            // if self.render_buf[i] == IMAGE_SETS.cook_t3[1] || self.render_buf[i] == IMAGE_SETS.result_cook[2] {
            //     self.flip_frame(false);
            // } else if self.render_buf[i] == IMAGE_SETS.ansem_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.ansem_dodge_2[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_2[1] {
            //     self.flip_frame(false);
            // } else if let Characters::COOK = self.player {
            //     self.flip_frame(true);
            // }

            
            if self.image_ref.src() != self.render_buf[i] {
                self.set_frame(&self.render_buf[i]);
            }
            web_sys::console::log_1(&JsValue::from_str("playing sounds"));

            if PLAY_PUNCH_SOUNDS_AT.contains(&self.render_buf[i].as_str()){
                self.audio.play_sound(&SOUNDS.punch).await
            }else if PLAY_DODGE_SOUND_AT.contains(&self.render_buf[i].as_str()){
                self.audio.play_sound(&SOUNDS.dodge).await
            }else if PLAY_PWRUP_SOUND_AT.contains(&self.render_buf[i].as_str()){
                self.audio.play_sound(&SOUNDS.tier3).await
            }

            web_sys::console::log_1(&JsValue::from_str("Sound played and finished"));
            sleep(Duration::from_millis(300)).await;
        }
        web_sys::console::log_1(&JsValue::from_str("Finished render_sequence"));
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
        // Validate inputs
        if player.is_empty() || wif <= 0.0 {
            web_sys::console::log_1(&JsValue::from_str("Invalid input parameters"));
            panic!("Invalid input parameters");
        }
        
        // Log the inputs
        web_sys::console::log_1(&JsValue::from_str(&format!("Rendering game with player: {}, wif: {}", player, wif)));
        
        // Try to create a new game
        let mut game = Game::new(player, wif);
        web_sys::console::log_1(&JsValue::from_str("Game instance created successfully"));
        
        // Log initial state
        web_sys::console::log_1(&JsValue::from_str(&format!("Initial punches: {}", game.npunches)));
        
        for i in 0..game.npunches {
            // Log the state before randomize_punch_sequences
            web_sys::console::log_1(&JsValue::from_str(&format!("Randomizing punch sequences, iteration: {}", i)));
            
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
        
        // Log state before cleanup
        web_sys::console::log_1(&JsValue::from_str("Running cleanup"));
        game.cleanup();
        web_sys::console::log_1(&JsValue::from_str("Cleanup done"));
        Ret{npunches: game.npunches, wif: wif}
    }

}
#[derive(Clone)]
#[wasm_bindgen]
pub struct AudioOps {
    audio_context: AudioContext,
}

impl AudioOps {
    pub fn new() -> AudioOps {
        AudioOps {
            audio_context: AudioContext::new().unwrap(),
        }
    }

    pub async fn play_sound(&self, path: &str) {
        let audio_element = HtmlAudioElement::new().unwrap();
        let n_p = format!("{}/{}", "/src/assets", path);
        audio_element.set_src(&n_p);
        web_sys::console::log_1(&JsValue::from_str(&n_p));
        // Start playing the audio and await the promise
        let play_promise = JsFuture::from(audio_element.play().unwrap());
        let _ = play_promise.await;

        // Create a future that resolves when the audio ends
        let (tx, rx) = oneshot::channel();
        let tx = std::rc::Rc::new(std::cell::RefCell::new(Some(tx)));

        let closure = Closure::wrap(Box::new(move |_event: Event| {
            if let Some(tx) = tx.borrow_mut().take() {
                let _ = tx.send(());
            }
        }) as Box<dyn FnMut(_)>);

        audio_element.set_onended(Some(closure.as_ref().unchecked_ref()));
        closure.forget(); // Memory management handled by wasm-bindgen

        let _ = rx.await; // Wait for the audio to finish playing

        // Reset the audio element state for future playbacks
        audio_element.set_onended(None);
        audio_element.set_src("");
    }
}

