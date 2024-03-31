use std::sync::mpsc::{Sender, Receiver};

use gba_core::GbaCore;

use wasm_bindgen::prelude::*;
use web_sys::{console, WorkerGlobalScope};

use crate::cpu_debug::CpuDebugInfo;
use crate::to_js_result::{ToJsResult, OptionToJsResult};
use crate::control::{Event, ControlState, Response};

pub struct GbaThread {
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
                .to_js_result("performance should be available")?;
        
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
                        self.gba = GbaCore::default();
                        self.gba.load_rom(&rom);
                        self.gba.skip_bios();
                    }
                    Event::ScreenData => {
                        screen_render = true;
                    }
                    Event::CpuDebugInfo => {
                        cpu_debug_info = true; 
                    }
                    Event::KeyEvent { key, pressed } => {
                        self.gba.set_key(key, pressed);
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
                self.tx.send(Response::ScreenData(data)).to_js_result()?;
            }
            if cpu_debug_info {
                cpu_debug_info = false;
                let pc = self.gba.pc();
                let info = CpuDebugInfo {
                    pc
                };
                self.tx.send(Response::CpuDebugInfo(info)).to_js_result()?;
            }
        }
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
