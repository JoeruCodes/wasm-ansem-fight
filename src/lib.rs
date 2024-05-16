#[warn(unreachable_code)]
mod config;
use config::{Characters, ImageSets, PunchTiers, DODGE_PROBS, IMAGE_SETS, SOUNDS, WIN_PUNCHES};
use wasm_bindgen::prelude::*;
use rand::Rng;
use web_sys::{AudioContext, HtmlAudioElement};
use crate::config::{PunchesConfig, PUNCHES_CONFIG};
use js_sys::{JsString, Number};
#[wasm_bindgen]
pub struct Game{
    player: Characters,
    wif: f64,
    tier: PunchTiers,
    npunches: usize,
    doges: usize,
    lpunches: usize,
    render_buf: Vec<String>,
    image_ref: web_sys::Element,
    audio: AudioOps
}

impl Game{
    pub fn new(player: &str, wif: f64, refs: web_sys::Element) -> Game{
        assert!(wif > 0.0);

        let punch_config;
        let tier;
        if wif <= 1.0{
            punch_config = &PUNCHES_CONFIG[0];
            tier = PunchTiers::T1;
        }else if wif < 41.0{
            punch_config = &PUNCHES_CONFIG[1];
            tier = PunchTiers::T2;
        }else{
            punch_config = &PUNCHES_CONFIG[2];
            tier = PunchTiers::T3;
        }
        let player_e;
        match player{
            "ansem" => player_e=Characters::ANSEM,
            "kook" => player_e = Characters::COOK,
            _ => panic!()
        }
        return Game{
            player: player_e,
            wif,
            tier,
            npunches:Game::generate_punches(&punch_config.min_punches, &punch_config.max_punches),
            doges: 0,
            lpunches: 0,
            render_buf: {
                match player_e{
                    Characters::ANSEM => punch_config.image_arr_p1.into_iter().map(|x| x.to_string()).collect(),
                    Characters::COOK => punch_config.image_arr_p2.into_iter().map(|x| x.to_string()).collect()
                }
            },
            image_ref:refs,
            audio: AudioOps{audio_ref: web_sys::HtmlAudioElement::new().unwrap(), audio_context: AudioContext::new().unwrap()}
        };
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
        let mut shuffled = Self::shuffle_array(&mut self.render_buf[1..]).to_vec();
        shuffled.truncate(num_punches);
        self.render_buf = vec![self.render_buf[0].to_owned()];
        self.render_buf.extend_from_slice(&shuffled);
    }
    fn shuffle_array<'a, T>(array: &'a mut [T]) -> &'a mut [T] {
        let mut rng = rand::thread_rng();
        let len = array.len();
        for i in (1..len).rev() {
            let j = rng.gen_range(0..=i);
            array.swap(i, j);
        }
        array
    }
    pub fn generate_punches(min: &usize, max: &usize) -> usize{
        rand::thread_rng().gen_range(*min..*max)
    }
    pub fn set_frame(&self, path: &str){
        if let Some(img_element) = self.image_ref.clone().dyn_into::<web_sys::HtmlImageElement>().ok() {
            img_element.set_src(path);
        } else {
            panic!("The referenced element is not an image element.");
        }
    }
    pub fn flip_frame(&self, bool: bool){
        if let Some(element) = self.image_ref.dyn_ref::<web_sys::HtmlElement>() {
            element.set_class_name({
                if bool{
                    "scale-x-[-1]"
                }else{
                    ""
                }
            });
        } else {
            // Handle the case where the referenced element is not an HTML element
            panic!("The referenced element is not an HTML element.");
        }
    }
    pub fn cleanup(&mut self){
        self.flip_frame(false);
        let win_or_lose = self.npunches > WIN_PUNCHES;
        self.set_frame({
            let idx = if win_or_lose{
                1
            }else{
                0
            };
            match &self.player{
                Characters::ANSEM => IMAGE_SETS.result_ansem[idx],
                Characters::COOK => IMAGE_SETS.result_cook[idx]
            }
        });
        self.play_sound({
            if win_or_lose{
                SOUNDS.win
            }else{
                SOUNDS.lose
            }
        });

        if !(self.render_buf.last().unwrap() == IMAGE_SETS.ansem_t3.last().unwrap() ||self.render_buf.last().unwrap() == IMAGE_SETS.cook_t3.last().unwrap()){
            self.play_sound(&SOUNDS.punch);
        }
        self.set_frame(IMAGE_SETS.default[0]);
    }

    pub fn render_sequence(&mut self){
        for i in 0..self.render_buf.len(){

            //TODO: Add delays
            if self.render_buf[i] == IMAGE_SETS.cook_t3[1] || self.render_buf[i] == IMAGE_SETS.result_cook[2]{
                self.flip_frame(false);
            }else if self.render_buf[i] == IMAGE_SETS.ansem_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.ansem_dodge_2[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_2[1]{
                self.play_sound(&SOUNDS.dodge);
                self.flip_frame(false);
            }else if let Characters::COOK = self.player {
                self.flip_frame(true);
            }

            if self.render_buf[i] == IMAGE_SETS.ansem_t3[2] || self.render_buf[i] == IMAGE_SETS.cook_t3[1]{
                self.play_sound(&SOUNDS.tier3);
            }else if !(self.render_buf[i] == IMAGE_SETS.ansem_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.ansem_dodge_2[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_1[1] || self.render_buf[i] == IMAGE_SETS.cook_dodge_2[1]){
                
                self.play_sound(&SOUNDS.punch);

                if(self.player == Characters::COOK && !(self.render_buf[i] == IMAGE_SETS.ansem_t1[1] || self.render_buf[i] == IMAGE_SETS.ansem_t2[2])) || (self.player == Characters::ANSEM && !(self.render_buf[i] == IMAGE_SETS.cook_t1[1] || self.render_buf[i] == IMAGE_SETS.cook_t2[2])){
                    self.lpunches += 1;    
                }else{
                    self.doges += 1;
                }
            }
            if let Some(x) = self.image_ref.get_attribute("src"){
                if x!=self.render_buf[i]{
                    self.set_frame(&self.render_buf[i]);
                }
            }
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
    pub fn render(player: &str, wif: f64, refs: web_sys::Element) ->  Ret{
        let mut game = Game::new(player, wif, refs);
    
        for i in 0..game.npunches {
            if game.render_buf != IMAGE_SETS.cook_dodge_1 || game.render_buf != IMAGE_SETS.cook_dodge_2 || game.render_buf != IMAGE_SETS.ansem_dodge_1 || game.render_buf != IMAGE_SETS.ansem_dodge_2 || game.render_buf != IMAGE_SETS.ansem_t3 || game.render_buf != IMAGE_SETS.cook_t3 {
                game.randomize_punch_sequences();
            }
        }
        game.cleanup();
        Ret { npunches: game.npunches, wif: game.wif }
    }
    
    pub fn play_sound(&self, path: &str){

        self.audio.audio_ref.set_src(path);
        let _ = self.audio.audio_ref.play();


        //idk how to use context but ye


        // let source = self.audio.audio_context.create_media_element_source(&self.audio.audio_ref).unwrap(); 
        // source.start_with_when(Some(start_time)).unwrap();
        // source.connect_with_audio_node(&audio_context.destination()).unwrap();
    }   
}

#[wasm_bindgen]
pub struct AudioOps{
    audio_ref: HtmlAudioElement,
    audio_context: AudioContext
}