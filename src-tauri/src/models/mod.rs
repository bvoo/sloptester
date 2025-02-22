use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Clone)]
pub struct GamepadInfo {
    pub id: usize,
    pub name: String,
    pub controller_type: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GamepadState {
    pub buttons: Vec<bool>,
    pub axes: Vec<f32>,
}

#[derive(Clone)]
pub struct ControllerData {
    pub name: String,
    pub buttons: Vec<bool>,
    pub axes: Vec<f32>,
}

#[derive(Clone, Copy)]
pub struct XInputState {
    pub connected: bool,
    pub buttons: u16,
    pub left_x: i16,
    pub left_y: i16,
    pub right_x: i16,
    pub right_y: i16,
    pub left_trigger: u8,
    pub right_trigger: u8,
}

impl Default for XInputState {
    fn default() -> Self {
        Self {
            connected: false,
            buttons: 0,
            left_x: 0,
            left_y: 0,
            right_x: 0,
            right_y: 0,
            left_trigger: 0,
            right_trigger: 0,
        }
    }
}

pub struct ControllerState(pub Arc<Mutex<super::managers::ControllerManager>>);