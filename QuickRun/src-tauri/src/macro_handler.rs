use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, keymap::Keycode, MousePosition, MouseButton};
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};

fn convert_key(key: Keycode) -> Key {
    match key {
        Keycode::A => Key::Layout('a'),
        Keycode::B => Key::Layout('b'),
        Keycode::C => Key::Layout('c'),
        Keycode::D => Key::Layout('d'),
        Keycode::E => Key::Layout('e'),
        Keycode::F => Key::Layout('f'),
        Keycode::G => Key::Layout('g'),
        Keycode::H => Key::Layout('h'),
        Keycode::I => Key::Layout('i'),
        Keycode::J => Key::Layout('j'),
        Keycode::K => Key::Layout('k'),
        Keycode::L => Key::Layout('l'),
        Keycode::M => Key::Layout('m'),
        Keycode::N => Key::Layout('n'),
        Keycode::O => Key::Layout('o'),
        Keycode::P => Key::Layout('p'),
        Keycode::Q => Key::Layout('q'),
        Keycode::R => Key::Layout('r'),
        Keycode::S => Key::Layout('s'),
        Keycode::T => Key::Layout('t'),
        Keycode::U => Key::Layout('u'),
        Keycode::V => Key::Layout('v'),
        Keycode::W => Key::Layout('w'),
        Keycode::X => Key::Layout('x'),
        Keycode::Y => Key::Layout('y'),
        Keycode::Z => Key::Layout('z'),
        Keycode::Space => Key::Layout(' '),
        Keycode::Backspace => Key::Backspace,
        Keycode::Tab => Key::Tab,
        Keycode::Enter => Key::Return,
        Keycode::Escape => Key::Escape,
        Keycode::LControl => Key::Control,
        Keycode::LShift => Key::Shift,
        Keycode::LAlt => Key::Alt,

        _ => unimplemented!("Keycode not implemented: {}", key),
    }
}

fn convert_mouse_button(button: MouseButton) -> enigo::MouseButton {
    match button {
        1 => enigo::MouseButton::Left,
        2 => enigo::MouseButton::Middle,
        3 => enigo::MouseButton::Right,
        _ => unimplemented!("Mouse button not implemented"),
    }
}


#[derive(Debug, Clone)]
enum InputEvent {
    MouseMove,
    MouseDown,
    MouseUp,

    KeyDown,
    KeyUp,
}

#[derive(Debug, Clone)]
struct Step {
    pub event: EventStep,
    pub at_time: u128,
}

#[derive(Debug, Clone)]
struct EventStep {
    pub event_type: InputEvent,
    pub event_data: EventData,
}

#[derive(Debug, Clone)]
enum EventData {
    MouseMove(MousePosition),
    MouseData(MouseButton),
    KeyData(Keycode),
}


pub struct MacroHandler {
    stored_actions: Vec<Step>,
    pub is_recording: bool,
    pub is_playing: bool,
}

impl MacroHandler {
    pub fn new() -> MacroHandler {
        MacroHandler {
            stored_actions: Vec::new(),
            is_recording: false,
            is_playing: false,
        }
    }


    pub fn start_recording(&mut self) -> bool {
        if (self.is_recording) { return false; };
        self.is_recording = true;
        let device_state = DeviceState::new();


        let actions: Arc<Mutex<Vec<Step>>> = Arc::new(Mutex::new(Vec::new()));
        let initial_function_time = Instant::now();

        macro_rules! key_events {
            ($event_type: ident, $data_type: ident) => {
                {
                    let actions = actions.clone();
                    move |event| {
                        if *event != Keycode::F12 && *event != Keycode::F10 {
                            actions.lock().unwrap().push(get_current_step(InputEvent::$event_type, EventData::$data_type(*event), &initial_function_time));
                        }
                    }
                }
            };
            ($event_type: ident, $input_type: ty, $data_type: ident) => {
                {
                    let actions = actions.clone();
                    move |event: $input_type| {
                        actions.lock().unwrap().push(get_current_step(InputEvent::$event_type, EventData::$data_type(*event), &initial_function_time));
                    }
                }
            };
        }


        let _guard = device_state.on_mouse_move(key_events!(MouseMove, &MousePosition, MouseMove));
        let _guard = device_state.on_mouse_down(key_events!(MouseDown, &MouseButton, MouseData));
        let _guard = device_state.on_mouse_up(key_events!(MouseUp, &MouseButton, MouseData));

        let _guard = device_state.on_key_down(key_events!(KeyDown, KeyData));
        let _guard = device_state.on_key_up(key_events!(KeyUp, KeyData));

        println!("up to loop");
        loop {
            if device_state.get_keys().contains(&Keycode::F12) || !self.is_recording {
                println!("Testing");
                break;
            }
            thread::sleep(Duration::from_millis(10))
        };

        let final_actions = actions.lock().unwrap().clone().to_vec();
        println!("{:?}", &final_actions);
        self.stored_actions = final_actions;
        self.is_recording = false;
        println!("Done?");
        true
    }
    pub fn stop_recording(&mut self) {
        self.is_recording = false;
    }


    pub fn play_macro(&mut self, speed: &u8) {
        println!("Trying to play");
        (if (self.is_playing) { return; });
        self.is_playing = true;
        let mut enigo = Enigo::new();

        let start_time = Instant::now();
        for action in self.stored_actions.clone() {
            println!("About to do an event: {}", &speed);
            wait_for_event(&start_time, action.at_time, &speed);
            handle_event(&mut enigo, &action.event);
        }
        self.is_playing = false;
    }
}

// hello world this is a test hello world testing one two three

fn handle_event(enigo: &mut Enigo, event: &EventStep) {
    match event.event_data {
        EventData::MouseMove(mouse_pos) => {
            enigo.mouse_move_to(mouse_pos.0, mouse_pos.1);
        }
        EventData::MouseData(mouse_button) => {
            match event.event_type {
                InputEvent::MouseDown => {
                    enigo.mouse_down(convert_mouse_button(mouse_button));
                }
                InputEvent::MouseUp => {
                    enigo.mouse_up(convert_mouse_button(mouse_button));
                }
                _ => {}
            }
        }
        EventData::KeyData(keycode) => {
            match event.event_type {
                InputEvent::KeyDown => {
                    enigo.key_down(convert_key(keycode));
                }
                InputEvent::KeyUp => {
                    enigo.key_up(convert_key(keycode));
                }
                _ => {}
            }
        }
        _ => {
            panic!("Unhandled event type");
        }
    }
}

fn get_current_step(event_type: InputEvent, data: EventData, time: &Instant) -> Step {
    Step {
        event: EventStep {
            event_type,
            event_data: data,
        },
        at_time: time.elapsed().as_millis(),
    }
}

fn wait_for_event(current_time: &Instant, required_time: u128, speed: &u8) {
    let mut time = current_time.elapsed().as_millis();
    while time < required_time / *speed as u128 {
        time = current_time.elapsed().as_millis();
        thread::sleep(Duration::from_millis(1)); // testing one two three four five
    }
}