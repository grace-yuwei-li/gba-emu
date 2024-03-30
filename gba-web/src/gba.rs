use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use gba_core::Key;

use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

use wasm_thread as thread;

use crate::thread::GbaThread;
use crate::to_js_result::ToJsResult;
use crate::control::{Event, ControlEvent, Response};

#[wasm_bindgen]
/// Gameboy with debugger
pub struct Gba {
    tx: Sender<Event>,
    rx: Receiver<Response>,

    screen_array: Option<Uint8ClampedArray>,
}


#[wasm_bindgen]
impl Gba {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Gba {
        console_error_panic_hook::set_once();

        let (to_thread, from_control)  = mpsc::channel();
        let (to_control, from_thread)  = mpsc::channel();

        
        let _join_handle = thread::spawn(move|| {
            let mut gba_thread = GbaThread::new(to_control, from_control);
            gba_thread.start().unwrap();
        });
        

        Gba {
            tx: to_thread,
            rx: from_thread,
            screen_array: None,
        }
    }

    /// Load a rom
    pub fn load_rom(&self, rom: Vec<u8>) -> Result<(), JsValue> {
        self.tx.send(Event::LoadRom(rom)).to_js_result()
    }

    pub fn set_key(&mut self, key: Key, pressed: bool) -> Result<(), JsValue> {
        self.tx.send(Event::KeyEvent{key, pressed}).to_js_result()
    }

    /// Pause the GBA execution
    pub fn set_pause(&self, pause: bool) -> Result<(), JsValue> {
        self.tx.send(Event::ControlEvent(ControlEvent::Pause(pause))).to_js_result()
    }

    pub fn set_screen_array(&mut self, array: Uint8ClampedArray) {
        self.screen_array = Some(array);
    }

    pub fn request_screen_draw(&self) -> Result<(), JsValue> {
        self.tx.send(Event::ScreenData).to_js_result()
    }

    pub fn request_cpu_debug_info(&self) -> Result<(), JsValue> {
        self.tx.send(Event::CpuDebugInfo).to_js_result()
    }

    pub fn process_responses(&self) -> Result<(), JsValue> {
        for response in self.rx.try_iter() {
            match response {
                Response::ScreenData(screen_data) => {
                    if let Some(screen_array) = &self.screen_array {
                        let js_screen_data: Vec<u8> = screen_data.chunks_exact(3).flat_map(|chunk| [chunk[0], chunk[1], chunk[2], 255]).collect();
                        screen_array.copy_from(&js_screen_data);
                    }
                }
                Response::CpuDebugInfo(info) => {
                }
            }
        }

        Ok(())
    }
}
