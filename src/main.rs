use std::env::current_exe;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::current;
use std::time::{Duration, Instant};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, keymap::Keycode, MousePosition, MouseButton, MouseState};
use enigo::{Enigo, KeyboardControllable, MouseControllable};

const SAMPLE_RATE: u8 = 16;
const SPEED: u8 = 1;

#[derive(Debug)]
enum InputEvent {
    MouseMove,
    MouseDown,
    MouseUp,

    KeyDown,
    KeyUp,
}




#[derive(Debug)]
struct Step {
    pub event: EventStep,
    pub at_time: u128
}
#[derive(Debug)]
struct EventStep {
    pub event_type: InputEvent,
    pub event_data: EventData
}
#[derive(Debug)]
enum EventData {
    MouseMove(MousePosition),
    MouseData(MouseButton),
    KeyData(Keycode),
}


fn handle_event(enigo: &mut Enigo, event: &EventStep) {
    match event.event_data {
        EventData::MouseMove(mouse_pos) => {
            enigo.mouse_move_to(mouse_pos.0 as i32, mouse_pos.1 as i32);
        }
        EventData::MouseData(mouse_button) => {
            match event.event_type {
                InputEvent::MouseDown => {
                    enigo.mouse_down(match mouse_button {
                        MouseButton::Left => enigo::MouseButton::Left,
                        MouseButton::Right => enigo::MouseButton::Right,
                        MouseButton::Middle => enigo::MouseButton::Middle,
                    });
                }
                InputEvent::MouseUp => {
                    enigo.mouse_up(match mouse_button {
                        MouseButton::Left => enigo::MouseButton::Left,
                        MouseButton::Right => enigo::MouseButton::Right,
                        MouseButton::Middle => enigo::MouseButton::Middle,
                    });
                }
                _ => {}
            }
        }
        // EventData::KeyData(keycode) => {
        //     match event.event_type {
        //         InputEvent::KeyDown => {
        //             enigo.key_down(match keycode {
        //                 Keycode::A => enigo::Key::A,
        //                 Keycode::B => enigo::Key::B,
        //                 Keycode::C => enigo::Key::C,
        //                 Keycode::D => enigo::Key::D,
        //                 Keycode::E => enigo::Key::E,
        //                 Keycode::F => enigo::Key::F,
        //                 Keycode::G => enigo::Key::G,
        //                 Keycode::H => enigo::Key::H,
        //                 Keycode::I => enigo::Key::I,
        //                 Keycode::J => enigo::Key::J,
        //                 Keycode::K => enigo::Key::K,
        //                 Keycode::L => enigo::Key::L,
        //                 Keycode::M => enigo::Key::M,
        //                 Keycode::N => enigo::Key::N,
        //                 Keycode::O => enigo::Key::O,
        //                 Keycode::P => enigo::Key::P,
        //                 Keycode::Q => enigo::Key::Q,
        //                 Keycode::R => enigo::Key::R,
        //                 Keycode::S => enigo::Key::S,
        //                 Keycode::T => enigo::Key::T,
        //                 Keycode::U => enigo::Key::U,
        //                 Keycode::V => enigo::Key::V,
        //                 Keycode::W => enigo::Key::W,
        //                 Keycode::X => enigo::Key::X,
        //                 Keycode::Y => enigo::Key::Y,
        //                 Keycode::Z => enigo::Key::Z,
        //                 Keycode::Key1 => enigo::Key::Num1,
        //                 Keycode::Key2 => enigo::Key::Num2,
        //                 Keycode::Key3 => enigo::Key::Num3,
        //                 Keycode::Key4 => enigo::Key::Num4,
        //                 Keycode::Key5 => enigo::Key::Num5,
        //                 Keycode::Key6 => enigo::Key::Num6,
        //                 Keycode::Key7 => enigo::Key::Num7,
        //                 Keycode::Key8 => enigo::Key::Num8,
        //                 Keycode::Key9 => enigo::Key::Num9,
        //                 Keycode::Key0 => enigo::Key::Num0,
        //                 Keycode::Enter => enigo::Key::Return,
        //                 Keycode::Escape => enigo::Key::Escape,
        //                 Keycode::Backspace => enigo::Key::Backspace,
        //                 Keycode::Tab => enigo::Key::Tab,
        //                 Keycode::Space => enigo::Key::Space,
        //                 Keycode::Minus => enigo::Key::OEMMinus,
        //                 Keycode::Equal => enigo::Key::OEMNECEqual
        //             });
        //         }
        //     }
        // }
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
        at_time: time.elapsed().as_millis().clone(),
    }
}

fn main() {
    let device_state = DeviceState::new();

    println!("Press 'F12' to stop the loop.");

    let mut actions: Arc<Mutex<Vec<Step>>> = Arc::new(Mutex::new(Vec::new()));


    let initial_function_time = Instant::now();

    let _guard = device_state.on_mouse_move({
        let actions = actions.clone();
        move |pos: &MousePosition| {

            actions.lock().unwrap().push(get_current_step(InputEvent::MouseMove, EventData::MouseMove(*pos), &initial_function_time));
        }
    });
    let _guard = device_state.on_mouse_down({
        let actions = actions.clone();
        move |mouse_button: &MouseButton| {
            actions.lock().unwrap().push(get_current_step(InputEvent::MouseDown, EventData::MouseData(*mouse_button), &initial_function_time));
        }
    });
    let _guard = device_state.on_mouse_up({
        let actions = actions.clone();
        move |mouse_button: &MouseButton| {
            actions.lock().unwrap().push(get_current_step(InputEvent::MouseUp, EventData::MouseData(*mouse_button), &initial_function_time));
        }
    });
    let _guard = device_state.on_key_down({
        let actions = actions.clone();
        move |key_pressed| {
            actions.lock().unwrap().push(get_current_step(InputEvent::KeyDown, EventData::KeyData(*key_pressed), &initial_function_time));
        }
    });
    let _guard = device_state.on_key_up({
        let actions = actions.clone();
        move |key_pressed| {
            actions.lock().unwrap().push(get_current_step(InputEvent::KeyUp, EventData::KeyData(*key_pressed), &initial_function_time));
        }
    });


    loop {
        if device_state.get_keys().contains(&Keycode::F12) {
            break;
        }
    }

    println!("{:?}", &actions.lock().unwrap());


    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_millis(2000));
    for action in actions.lock().unwrap().iter() {
        handle_event(&mut enigo, &action.event);
    }
}
