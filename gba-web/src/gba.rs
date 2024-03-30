use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use gba_core::GbaCore;
use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;
use web_sys::{console, WorkerGlobalScope};

use wasm_thread::{self as thread, JoinHandle};

use crate::cpu_debug::CpuDebugInfo;

/// Events that are sent to the GBA thread
enum Event {
    ControlEvent(ControlEvent),
    LoadRom(Vec<u8>),
    ScreenData,
    CpuDebugInfo,
}

enum ControlEvent {
    Pause(bool) 
}

struct ControlState {
    pub pause: bool,
}

impl ControlState {
    pub fn new() -> Self {
        Self {pause: false}
    }

    pub fn update(&mut self, event: ControlEvent) {
        match event {
            ControlEvent::Pause(pause) => {
                self.pause = pause
            }
        }
    }
}


enum Response {
    ScreenData(Vec<u8>),
    CpuDebugInfo(CpuDebugInfo),
}


struct GbaThread {
    gba: GbaCore,

    tx: Sender<Response>,
    rx: Receiver<Event>,

    control_state: ControlState,
}

impl GbaThread {
    pub fn new(tx: Sender<Response>, rx: Receiver<Event>) -> Self {

        // Emulator instance
        let gba = GbaCore::default();
        console::log_1(&"Constructed a Gba".into());

        Self {
            gba,
            tx, 
            rx,
            control_state: ControlState::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        let worker = worker_global_scope()?;
        let performance = worker
                .performance()
                .expect("performance should be available");
        
        let ticks = 100000;

        let mut screen_render = false;
        let mut cpu_debug_info = false;

        self.gba.load_test_rom();
        self.gba.skip_bios();

        loop {
            for event in self.rx.try_iter() {
                match event {
                    Event::ControlEvent(event) => {
                        self.control_state.update(event);
                    }
                    Event::LoadRom(rom) => {
                        self.gba.load_rom(&rom);
                    }
                    Event::ScreenData => {
                        screen_render = true;
                    }
                    Event::CpuDebugInfo => {
                        cpu_debug_info = true; 
                    }
                }
            }
             
            if self.control_state.pause {
                continue;
            }

            let start_time = performance.now();
            self.gba.tick_multiple(ticks);
            let end_time = performance.now();
            let elapsed = end_time - start_time;
            // Mult by 1000 for ms -> s
            let ticks_per_sec = ticks as f64 / elapsed * 1000.;


            if screen_render {
                screen_render = false;
                let data = self.gba.screen();
                self.tx.send(Response::ScreenData(data));
            }
            if cpu_debug_info {
                cpu_debug_info = false;
                let pc = self.gba.pc();
                let info = CpuDebugInfo {
                    pc
                };
                self.tx.send(Response::CpuDebugInfo(info));
            }
        }
    }
}

#[wasm_bindgen]
/// Gameboy with debugger
pub struct Gba {
    tx: Sender<Event>,
    rx: Receiver<Response>,
    join_handle: JoinHandle<()>,

    screen_array: Option<Uint8ClampedArray>,
}


#[wasm_bindgen]
impl Gba {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Gba {
        console_error_panic_hook::set_once();

        let (to_thread, from_control)  = mpsc::channel();
        let (to_control, from_thread)  = mpsc::channel();

        
        let join_handle = thread::spawn(move|| {
            let mut gba_thread = GbaThread::new(to_control, from_control);
            gba_thread.start().unwrap();
        });
        

        Gba {
            tx: to_thread,
            rx: from_thread,
            join_handle,
            screen_array: None,
        }
    }

    pub fn load_rom(&self, rom: Vec<u8>) {
        self.tx.send(Event::LoadRom(rom));
    }

    /// Pause the GBA execution
    pub fn set_pause(&self, pause: bool) -> Result<(), JsValue> {
        self.tx.send(Event::ControlEvent(ControlEvent::Pause(pause))).map_err(|e| e.to_string().into())
    }

    pub fn set_screen_array(&mut self, array: Uint8ClampedArray) {
        self.screen_array = Some(array);
    }

    pub fn request_screen_draw(&self) -> Result<(), JsValue> {
        self.tx.send(Event::ScreenData).map_err(|e| e.to_string().into())
    }

    pub fn request_cpu_debug_info(&self) -> Result<(), JsValue> {
        self.tx.send(Event::CpuDebugInfo).map_err(|e| e.to_string().into())
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

fn worker_global_scope() -> Result<WorkerGlobalScope, JsValue> {
    let global = js_sys::global();
    // how to properly detect this in wasm_bindgen?
    if js_sys::eval("typeof WorkerGlobalScope !== 'undefined'")?.as_bool().unwrap() {
        Ok(global.dyn_into::<WorkerGlobalScope>()?)
    } else {
        Err("Not in worker".into())
    }
}
