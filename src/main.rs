use std::env::current_exe;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::current;
use std::time::{Duration, Instant};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, keymap::Keycode, MousePosition, MouseButton, MouseState};
use enigo::{Enigo, MouseControllable};

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



fn main() {
    let device_state = DeviceState::new();

    println!("Press 'F12' to stop the loop.");

    let mut actions: Arc<Mutex<Vec<Step>>> = Arc::new(Mutex::new(Vec::new()));


    let initial_function_time = Instant::now();

    let _guard = device_state.on_mouse_move({
        let actions = actions.clone();
        move |pos: &MousePosition| {
            let current_step = Step {
                event: EventStep {
                    event_type: InputEvent::MouseMove,
                    event_data: EventData::MouseMove(*pos),
                },
                at_time: initial_function_time.elapsed().as_millis().clone(),
            };
            actions.lock().unwrap().push(current_step);
        }
    });
    let _guard = device_state.on_mouse_down({
        let actions = actions.clone();
        move |mouse_button: &MouseButton| {
            let current_step = Step {
                event: EventStep {
                    event_type: InputEvent::MouseDown,
                    event_data: EventData::MouseData(*mouse_button),
                },
                at_time: initial_function_time.elapsed().as_millis().clone(),
            };
            actions.lock().unwrap().push(current_step);
        }
    });
    let _guard = device_state.on_mouse_up({
        let actions = actions.clone();
        move |mouse_button: &MouseButton| {
            let current_step = Step {
                event: EventStep {
                    event_type: InputEvent::MouseUp,
                    event_data: EventData::MouseData(*mouse_button),
                },
                at_time: initial_function_time.elapsed().as_millis().clone(),
            };
            actions.lock().unwrap().push(current_step);
        }
    });


    let _guard = device_state.on_key_down({
        let actions = actions.clone();
        move |key_pressed| {
            let current_step = Step {
                event: EventStep {
                    event_type: InputEvent::KeyDown,
                    event_data: EventData::KeyData(*key_pressed),
                },
                at_time: initial_function_time.elapsed().as_millis().clone(),
            };
            actions.lock().unwrap().push(current_step);
        }
    });
    let _guard = device_state.on_key_up({
        let actions = actions.clone();
        move |key_pressed| {
            let current_step = Step {
                event: EventStep {
                    event_type: InputEvent::KeyUp,
                    event_data: EventData::KeyData(*key_pressed),
                },
                at_time: initial_function_time.elapsed().as_millis().clone(),
            };
            actions.lock().unwrap().push(current_step);
        }
    });


    loop {
        if device_state.get_keys().contains(&Keycode::F12) {
            break;
        }
    }

    println!("{:?}", actions.lock().unwrap());



    // let mut enigo = Enigo::new();
    // // thread::sleep(Duration::from_millis(2000));
    // for action in &actions {
    //     for i in 0..(action.duration + 1) {
    //         excecute_action(&mut enigo, &action.input_state);
    //     }
    // }
}
