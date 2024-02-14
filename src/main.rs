use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, keymap::Keycode, MousePosition, MouseButton};
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};
use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Font, Sandbox, Settings};
use iced::window::{Level, PlatformSpecific, Position};

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





const SPEED: u8 = 1;

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
    pub at_time: u128
}
#[derive(Debug, Clone)]
struct EventStep {
    pub event_type: InputEvent,
    pub event_data: EventData
}
#[derive(Debug, Clone)]
enum EventData {
    MouseMove(MousePosition),
    MouseData(MouseButton),
    KeyData(Keycode),
}





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
                },
                InputEvent::KeyUp => {
                    enigo.key_up(convert_key(keycode));
                },
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

fn wait_for_event(current_time: &Instant, required_time: u128) {
    let mut time = current_time.elapsed().as_millis();
    while time < required_time / SPEED as u128 {
        time = current_time.elapsed().as_millis();
        thread::sleep(Duration::from_millis(1));
    }
}


fn record_macro() -> Vec<Step> {
    let device_state = DeviceState::new();

    println!("Press 'F12' to stop the loop.");

    let actions: Arc<Mutex<Vec<Step>>> = Arc::new(Mutex::new(Vec::new()));


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
            if *key_pressed == Keycode::F12 || *key_pressed == Keycode::F10 {
                return;
            }
            actions.lock().unwrap().push(get_current_step(InputEvent::KeyDown, EventData::KeyData(*key_pressed), &initial_function_time));
        }
    });
    let _guard = device_state.on_key_up({
        let actions = actions.clone();
        move |key_pressed| {
            if *key_pressed == Keycode::F12 || *key_pressed == Keycode::F10 {
                return;
            }
            actions.lock().unwrap().push(get_current_step(InputEvent::KeyUp, EventData::KeyData(*key_pressed), &initial_function_time));
        }
    });


    loop {
        if device_state.get_keys().contains(&Keycode::F12) {
            break;
        }
    };

    let final_actions = actions.lock().unwrap().clone().to_vec();
    final_actions
}


fn play_macro(actions: Vec<Step>) {
    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_millis(2000));

    let start_time = Instant::now();
    for action in actions {
        wait_for_event(&start_time, action.at_time);
        handle_event(&mut enigo, &action.event);
    }
}


fn main() {
    let app_settings = Settings {
        window: iced::window::Settings {
            size: (100, 384),
            resizable: false,
            decorations: true,
            min_size: None,
            max_size: None,
            transparent: false,
            icon: None,
            level: Level::Normal,
            visible: true,
            position: Position::Centered,
            platform_specific: PlatformSpecific::default()
        },
        default_font: Font::MONOSPACE,
        antialiasing: true,
        default_text_size: 20.,
        id: None,
        flags: (),
        exit_on_close_request: true,
    };


    let _ = QuickRun::run(app_settings);
    // let actions = record_macro();
    // println!("{:?}", &actions);
    //
    // play_macro(actions);
}

struct QuickRun {
    current_macro: Vec<Step>,
    is_recording: bool,
    is_playing: bool,
}

#[derive(Debug, Clone, Copy)]
enum PressedEvent {
    RecordPressed,
    PlayPressed,
    SavePressed
}

impl Sandbox for QuickRun {
    type Message = PressedEvent;

    fn new() -> Self {
        Self { current_macro: Vec::new(), is_recording: false, is_playing: false }
    }

    fn title(&self) -> String {
        String::from("QuickRun")
    }

    fn update(&mut self, pressed_event: PressedEvent) {
        match pressed_event {
            PressedEvent::RecordPressed => {
                println!("Recording");
            },
            PressedEvent::PlayPressed => {
                println!("Playing");
            },
            _ => {unimplemented!("Button is not implemented yet.")}
        }
    }

    fn view(&self) -> Element<PressedEvent> {
        column![
            Row![
                text("QuickRun").size(16),
                button("X").on_press(PressedEvent::RecordPressed),
            ],
            button("Start Recording").on_press(PressedEvent::RecordPressed),
            text("Hello world").size(50),
            button("Stop Recording").on_press(PressedEvent::PlayPressed)
        ]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}