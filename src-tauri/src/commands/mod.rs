use rusty_xinput::XInputHandle;
use tauri::State;
use winapi::um::xinput::*;
use super::models::{ControllerState, GamepadInfo, GamepadState};
use super::utils::{normalize_xinput_axis, normalize_xinput_trigger};

#[tauri::command(rename_all = "snake_case")]
pub fn get_gamepads(controller_state: State<ControllerState>) -> Vec<GamepadInfo> {
    let mut controllers = Vec::new();
    let mut manager = controller_state.0.lock().unwrap();
    manager.poll();

    // Add XInput controllers (IDs 0-3)
    if let Ok(xinput) = XInputHandle::load_default() {
        for i in 0..4 {
            match xinput.get_state(i) {
                Ok(_) => {
                    if !manager.xinput_previous_states[i as usize].connected {
                        println!("XInput Controller {} connected", i + 1);
                        manager.xinput_previous_states[i as usize].connected = true;
                    }
                    controllers.push(GamepadInfo {
                        id: i as usize,
                        name: format!("Xbox Controller {}", i + 1),
                        controller_type: "xinput".to_string(),
                        vendor_id: None,
                        product_id: None,
                    });
                },
                Err(_) => {
                    if manager.xinput_previous_states[i as usize].connected {
                        println!("XInput Controller {} disconnected", i + 1);
                        manager.xinput_previous_states[i as usize].connected = false;
                    }
                }
            }
        }
    }

    // Add SDL controllers using their actual instance IDs
    if let Some(sdl) = manager.get_sdl_context() {
        if let Ok(js) = sdl.context.joystick() {
            if let Ok(available) = js.num_joysticks() {
                let states = manager.get_controller_states();
                
                for (id, name, _, _) in states.iter() {
                    for joy_id in 0..available {
                        if let Ok(joystick) = js.open(joy_id) {
                            if joystick.instance_id() == *id {
                                // Get the GUID string and try to parse vendor/product IDs
                                let guid_str = format!("{}", joystick.guid());
                                // The GUID string format is typically "00000000000000000000000000000000"
                                // where bytes 8-10 contain vendor ID and bytes 10-12 contain product ID
                                let vid = u16::from_str_radix(&guid_str[8..12], 16).ok();
                                let pid = u16::from_str_radix(&guid_str[12..16], 16).ok();
                                
                                controllers.push(GamepadInfo {
                                    id: *id as usize,
                                    name: name.clone(),
                                    controller_type: "sdl".to_string(),
                                    vendor_id: vid,
                                    product_id: pid,
                                });
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    controllers
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_gamepad_state(id: usize, controller_type: String, controller_state: State<ControllerState>) -> Option<GamepadState> {
    let mut manager = controller_state.0.lock().unwrap();
    manager.poll();
    
    match controller_type.as_str() {
        "xinput" => {
            // XInput controller handling
            if let Ok(xinput) = XInputHandle::load_default() {
                if let Ok(state) = xinput.get_state(id as u32) {
                    let raw = state.raw;
                    let mut prev_state = manager.xinput_previous_states[id];
                    
                    // Calculate all analog values first
                    let left_x = normalize_xinput_axis(raw.Gamepad.sThumbLX);
                    let left_y = normalize_xinput_axis(raw.Gamepad.sThumbLY);
                    let right_x = normalize_xinput_axis(raw.Gamepad.sThumbRX);
                    let right_y = normalize_xinput_axis(raw.Gamepad.sThumbRY);
                    let left_trigger = normalize_xinput_trigger(raw.Gamepad.bLeftTrigger);
                    let right_trigger = normalize_xinput_trigger(raw.Gamepad.bRightTrigger);
                    
                    // Check for button state changes
                    let buttons = [
                        (XINPUT_GAMEPAD_A as u16, "A"),
                        (XINPUT_GAMEPAD_B as u16, "B"),
                        (XINPUT_GAMEPAD_X as u16, "X"),
                        (XINPUT_GAMEPAD_Y as u16, "Y"),
                        (XINPUT_GAMEPAD_LEFT_SHOULDER as u16, "LB"),
                        (XINPUT_GAMEPAD_RIGHT_SHOULDER as u16, "RB"),
                        (XINPUT_GAMEPAD_BACK as u16, "Back"),
                        (XINPUT_GAMEPAD_START as u16, "Start"),
                        (XINPUT_GAMEPAD_LEFT_THUMB as u16, "L3"),
                        (XINPUT_GAMEPAD_RIGHT_THUMB as u16, "R3"),
                        (XINPUT_GAMEPAD_DPAD_UP as u16, "DPad Up"),
                        (XINPUT_GAMEPAD_DPAD_DOWN as u16, "DPad Down"),
                        (XINPUT_GAMEPAD_DPAD_LEFT as u16, "DPad Left"),
                        (XINPUT_GAMEPAD_DPAD_RIGHT as u16, "DPad Right"),
                    ];

                    // Only log button changes
                    if raw.Gamepad.wButtons != prev_state.buttons {
                        for (button_mask, button_name) in buttons.iter() {
                            let was_pressed = prev_state.buttons & button_mask != 0;
                            let is_pressed = raw.Gamepad.wButtons & button_mask != 0;
                            if was_pressed != is_pressed {
                                if is_pressed {
                                    println!("XInput {}: {} button pressed", id, button_name);
                                } else {
                                    println!("XInput {}: {} button released", id, button_name);
                                }
                            }
                        }
                    }

                    // Only log stick changes if they differ significantly from previous state
                    let stick_threshold: i32 = 8192; // Convert to i32 for safe math
                    let left_x_diff = (raw.Gamepad.sThumbLX as i32) - (prev_state.left_x as i32);
                    let left_y_diff = (raw.Gamepad.sThumbLY as i32) - (prev_state.left_y as i32);
                    let right_x_diff = (raw.Gamepad.sThumbRX as i32) - (prev_state.right_x as i32);
                    let right_y_diff = (raw.Gamepad.sThumbRY as i32) - (prev_state.right_y as i32);

                    if left_x_diff.abs() > stick_threshold || left_y_diff.abs() > stick_threshold {
                        println!("XInput {}: Left stick at ({:.2}, {:.2})", id, left_x, left_y);
                    }
                    if right_x_diff.abs() > stick_threshold || right_y_diff.abs() > stick_threshold {
                        println!("XInput {}: Right stick at ({:.2}, {:.2})", id, right_x, right_y);
                    }

                    // Only log trigger changes if they differ significantly from previous state
                    let trigger_threshold = 20;
                    let left_trigger_diff = (raw.Gamepad.bLeftTrigger as i16) - (prev_state.left_trigger as i16);
                    let right_trigger_diff = (raw.Gamepad.bRightTrigger as i16) - (prev_state.right_trigger as i16);

                    if left_trigger_diff.abs() > trigger_threshold {
                        println!("XInput {}: Left trigger at {:.2}", id, left_trigger);
                    }
                    if right_trigger_diff.abs() > trigger_threshold {
                        println!("XInput {}: Right trigger at {:.2}", id, right_trigger);
                    }

                    // Update previous state
                    prev_state.buttons = raw.Gamepad.wButtons;
                    prev_state.left_x = raw.Gamepad.sThumbLX;
                    prev_state.left_y = raw.Gamepad.sThumbLY;
                    prev_state.right_x = raw.Gamepad.sThumbRX;
                    prev_state.right_y = raw.Gamepad.sThumbRY;
                    prev_state.left_trigger = raw.Gamepad.bLeftTrigger;
                    prev_state.right_trigger = raw.Gamepad.bRightTrigger;
                    manager.xinput_previous_states[id] = prev_state;

                    return Some(GamepadState {
                        buttons: vec![
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_A as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_B as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_X as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_Y as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_LEFT_SHOULDER as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_RIGHT_SHOULDER as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_BACK as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_START as u16) != 0,
                            false,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_LEFT_THUMB as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_RIGHT_THUMB as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_DPAD_UP as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_DPAD_DOWN as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_DPAD_LEFT as u16) != 0,
                            raw.Gamepad.wButtons & (XINPUT_GAMEPAD_DPAD_RIGHT as u16) != 0,
                        ],
                        axes: vec![
                            left_x,
                            left_y,
                            right_x,
                            right_y,
                            left_trigger,
                            right_trigger,
                        ],
                    });
                }
            }
        }
        "sdl" => {
            // SDL controller handling
            let states = manager.get_controller_states();
            if let Some((_, _, buttons, axes)) = states.iter().find(|(controller_id, _, _, _)| *controller_id == id as u32) {
                return Some(GamepadState {
                    buttons: buttons.clone(),
                    axes: axes.clone(),
                });
            }
        }
        _ => {}
    }
    
    None
}