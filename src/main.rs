use std::env::current_exe;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::current;
use std::time::Duration;
use device_query::{DeviceEvents, DeviceQuery, DeviceState, keymap::Keycode, MousePosition, MouseState};
use enigo::{Enigo, MouseControllable};

const SAMPLING: u8 = 16;

#[derive(PartialEq, Debug)]
struct Step {
    pub input_state: InputState,
    pub duration: u8
}

#[derive(PartialEq, Debug)]
struct InputState {
    pub mouse_current: MouseState,
    pub keys_current: Vec<Keycode>,
}


fn excecute_action(enigo: &mut Enigo, go_to: &InputState) {
    enigo.mouse_move_to(go_to.mouse_current.coords.0, go_to.mouse_current.coords.1);
    thread::sleep(Duration::from_millis(SAMPLING.into()))
}



fn main() {
    let device_state = DeviceState::new();

    println!("Press 'F12' to stop the loop.");

    let mut actions: Vec<Step> = Vec::new();


    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::F12) {
            break;
        }

        let current_state = Step {
            input_state: InputState {
                mouse_current: device_state.get_mouse(),
                keys_current: keys,
            },
            duration: 0
        };



        let last_el = actions.len();


        if last_el != 0 && &actions[last_el - 1].input_state == &current_state.input_state {
            actions[last_el - 1].duration += 1
        } else {
            actions.push(current_state);
        }

        // Sleep for a short duration to avoid high CPU usage
        std::thread::sleep(std::time::Duration::from_millis(SAMPLING.into()));
    }





    println!("{:?}", actions);



    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_millis(2000));
    for action in &actions {
        for i in 0..(action.duration + 1) {
            excecute_action(&mut enigo, &action.input_state);
        }
    }
}
