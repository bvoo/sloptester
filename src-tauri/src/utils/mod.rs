use sdl2::controller::{Button, Axis};

pub fn map_sdl_button(button: Button) -> usize {
    use sdl2::controller::Button::*;
    match button {
        A => 0,
        B => 1,
        X => 2,
        Y => 3,
        LeftShoulder => 4,
        RightShoulder => 5,
        Back => 6,
        Start => 7,
        Guide => 8,
        LeftStick => 9,
        RightStick => 10,
        DPadUp => 11,
        DPadDown => 12,
        DPadLeft => 13,
        DPadRight => 14,
        _ => 0,
    }
}

pub fn map_sdl_axis(axis: Axis) -> usize {
    use sdl2::controller::Axis::*;
    match axis {
        LeftX => 0,
        LeftY => 1,
        RightX => 2,
        RightY => 3,
        TriggerLeft => 4,
        TriggerRight => 5,
    }
}

pub fn normalize_sdl_axis(axis: Axis, value: i16) -> f32 {
    use sdl2::controller::Axis::*;
    match axis {
        // SDL triggers are 0 when released, 32767 when fully pressed
        TriggerLeft | TriggerRight => {
            value as f32 / 32767.0
        },
        // SDL Y axes point down for positive values, we want up for positive
        LeftY | RightY => -(value as f32 / 32767.0),
        // Other axes use regular normalization
        _ => value as f32 / 32767.0
    }
}

pub fn normalize_xinput_axis(value: i16) -> f32 {
    value as f32 / 32768.0
}

pub fn normalize_xinput_trigger(value: u8) -> f32 {
    // XInput triggers are already in range 0-255, just normalize to 0-1
    value as f32 / 255.0
}