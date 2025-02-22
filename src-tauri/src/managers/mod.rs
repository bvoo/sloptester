use super::models::*;
use sdl2::controller::GameController;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct SdlContext {
    pub context: sdl2::Sdl,
    pub game_controller_subsystem: sdl2::GameControllerSubsystem,
    pub controllers: Arc<Mutex<Vec<GameController>>>,
}

unsafe impl Send for SdlContext {}
unsafe impl Sync for SdlContext {}

pub struct ControllerManager {
    sdl: Option<SdlContext>,
    controller_states: Arc<Mutex<HashMap<u32, ControllerData>>>,
    pub xinput_previous_states: [XInputState; 4],
}

impl ControllerManager {
    pub fn new() -> Self {
        let controller_states = Arc::new(Mutex::new(HashMap::new()));

        // Try to get SDL controller mappings from environment first
        if let Ok(mappings) = std::env::var("SDL_GAMECONTROLLERCONFIG") {
            println!("Found SDL_GAMECONTROLLERCONFIG environment variable");
            std::env::set_var("SDL_GAMECONTROLLERCONFIG", mappings);
        }

        let sdl = match sdl2::init() {
            Ok(context) => {
                println!("SDL initialized successfully");

                // First initialize joystick subsystem
                let joystick_subsystem = match context.joystick() {
                    Ok(js) => {
                        println!("SDL Joystick subsystem initialized");
                        js.set_event_state(true);
                        Some(js)
                    },
                    Err(e) => {
                        eprintln!("Failed to initialize joystick subsystem: {}", e);
                        None
                    }
                };

                // Then initialize game controller subsystem
                match context.game_controller() {
                    Ok(game_controller) => {
                        println!("SDL GameController subsystem initialized");
                        game_controller.set_event_state(true);

                        let controllers = Arc::new(Mutex::new(Vec::new()));

                        // If we have joystick subsystem, scan for devices
                        if let Some(js) = joystick_subsystem {
                            let available = js.num_joysticks().unwrap_or(0);
                            println!("Found {} joystick device(s)", available);

                            for id in 0..available {
                                if game_controller.is_game_controller(id) {
                                    match game_controller.open(id) {
                                        Ok(controller) => {
                                            println!("Found game controller: {} (instance ID: {})", controller.name(), controller.instance_id());
                                            println!("  GUID: {}", js.name_for_index(id).unwrap_or_default());
                                            println!("  Mapping: {}", controller.mapping());

                                            let instance_id = controller.instance_id();
                                            let mut states = controller_states.lock().unwrap();
                                            states.insert(instance_id, ControllerData {
                                                name: controller.name(),
                                                buttons: vec![false; 15],
                                                axes: vec![0.0; 6],
                                            });
                                            controllers.lock().unwrap().push(controller);
                                        },
                                        Err(e) => eprintln!("Failed to open controller {}: {}", id, e),
                                    }
                                } else {
                                    println!("Device {} is not a game controller", id);
                                }
                            }
                        }

                        Some(SdlContext { 
                            context,
                            game_controller_subsystem: game_controller,
                            controllers,
                        })
                    },
                    Err(e) => {
                        eprintln!("Failed to initialize SDL GameController subsystem: {}", e);
                        None
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to initialize SDL: {}", e);
                None
            }
        };

        Self {
            sdl,
            controller_states,
            xinput_previous_states: [XInputState::default(); 4],
        }
    }

    pub fn poll(&self) {
        let timeout = Duration::from_micros(50);
        let start = Instant::now();

        if let Some(sdl) = &self.sdl {
            if let Ok(mut event_pump) = sdl.context.event_pump() {
                while start.elapsed() < timeout {
                    if let Some(event) = event_pump.poll_event() {
                        match event {
                            sdl2::event::Event::JoyDeviceAdded { which, .. } => {
                                println!("SDL Joy Device added: {}", which);
                                if sdl.game_controller_subsystem.is_game_controller(which) {
                                    if let Ok(controller) = sdl.game_controller_subsystem.open(which) {
                                        println!("Opening as game controller: {}", controller.name());
                                        let instance_id = controller.instance_id();

                                        let mut states = self.controller_states.lock().unwrap();
                                        states.insert(instance_id, ControllerData {
                                            name: controller.name(),
                                            buttons: vec![false; 15],
                                            axes: vec![0.0; 6],
                                        });

                                        sdl.controllers.lock().unwrap().push(controller);
                                    }
                                }
                            },
                            sdl2::event::Event::JoyDeviceRemoved { which, .. } => {
                                println!("SDL Joy Device removed: {}", which);
                                let mut states = self.controller_states.lock().unwrap();
                                states.remove(&which);

                                let mut controllers = sdl.controllers.lock().unwrap();
                                controllers.retain(|c| c.instance_id() != which);
                            },
                            sdl2::event::Event::ControllerDeviceAdded { which, .. } => {
                                println!("SDL Controller Device added: {}", which);
                            },
                            sdl2::event::Event::ControllerDeviceRemoved { which, .. } => {
                                println!("SDL Controller Device removed: {}", which);
                                let mut states = self.controller_states.lock().unwrap();
                                states.remove(&which);

                                let mut controllers = sdl.controllers.lock().unwrap();
                                controllers.retain(|c| c.instance_id() != which);
                            },
                            sdl2::event::Event::ControllerAxisMotion { which, axis, value, .. } => {
                                println!("SDL axis motion: {:?} = {} on controller {}", axis, value, which);
                                let axis_index = super::utils::map_sdl_axis(axis);
                                if let Some(controller_data) = self.controller_states.lock().unwrap().get_mut(&which) {
                                    if axis_index < controller_data.axes.len() {
                                        controller_data.axes[axis_index] = super::utils::normalize_sdl_axis(axis, value);
                                    }
                                }
                            },
                            sdl2::event::Event::ControllerButtonDown { which, button, .. } => {
                                println!("SDL button down: {:?} on controller {}", button, which);
                                if let Some(controller_data) = self.controller_states.lock().unwrap().get_mut(&which) {
                                    let button_index = super::utils::map_sdl_button(button);
                                    if button_index < controller_data.buttons.len() {
                                        controller_data.buttons[button_index] = true;
                                    }
                                }
                            },
                            sdl2::event::Event::ControllerButtonUp { which, button, .. } => {
                                println!("SDL button up: {:?} on controller {}", button, which);
                                if let Some(controller_data) = self.controller_states.lock().unwrap().get_mut(&which) {
                                    let button_index = super::utils::map_sdl_button(button);
                                    if button_index < controller_data.buttons.len() {
                                        controller_data.buttons[button_index] = false;
                                    }
                                }
                            },
                            _ => {}
                        }
                    } else {
                        // Update state for all connected controllers
                        let controllers = sdl.controllers.lock().unwrap();
                        let mut states = self.controller_states.lock().unwrap();

                        for controller in controllers.iter() {
                            if let Some(state) = states.get_mut(&controller.instance_id()) {
                                // Update axes
                                for axis in &[
                                    sdl2::controller::Axis::LeftX,
                                    sdl2::controller::Axis::LeftY,
                                    sdl2::controller::Axis::RightX,
                                    sdl2::controller::Axis::RightY,
                                    sdl2::controller::Axis::TriggerLeft,
                                    sdl2::controller::Axis::TriggerRight,
                                ] {
                                    let value = controller.axis(*axis);
                                    let axis_index = super::utils::map_sdl_axis(*axis);
                                    if axis_index < state.axes.len() {
                                        state.axes[axis_index] = super::utils::normalize_sdl_axis(*axis, value);
                                    }
                                }

                                // Update buttons
                                for button in &[
                                    sdl2::controller::Button::A,
                                    sdl2::controller::Button::B,
                                    sdl2::controller::Button::X,
                                    sdl2::controller::Button::Y,
                                    sdl2::controller::Button::Back,
                                    sdl2::controller::Button::Guide,
                                    sdl2::controller::Button::Start,
                                    sdl2::controller::Button::LeftStick,
                                    sdl2::controller::Button::RightStick,
                                    sdl2::controller::Button::LeftShoulder,
                                    sdl2::controller::Button::RightShoulder,
                                    sdl2::controller::Button::DPadUp,
                                    sdl2::controller::Button::DPadDown,
                                    sdl2::controller::Button::DPadLeft,
                                    sdl2::controller::Button::DPadRight,
                                ] {
                                    let pressed = controller.button(*button);
                                    let button_index = super::utils::map_sdl_button(*button);
                                    if button_index < state.buttons.len() {
                                        state.buttons[button_index] = pressed;
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    pub fn get_controller_states(&self) -> Vec<(u32, String, Vec<bool>, Vec<f32>)> {
        let states = self.controller_states.lock().unwrap();
        states
            .iter()
            .map(|(id, data)| (*id, data.name.clone(), data.buttons.clone(), data.axes.clone()))
            .collect()
    }

    pub fn get_sdl_context(&self) -> &Option<SdlContext> {
        &self.sdl
    }
}