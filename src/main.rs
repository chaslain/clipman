use std::{sync::{Arc, Mutex}, fs::File, io::{Write, Read}, process::exit};

use clipboard_win::{formats, Clipboard, Getter, Setter, Unicode};
use mki::{register_hotkey, Keyboard};
use notify_rust::Notification;

struct ClipData {
    data: Vec<Mutex<Option<String>>>,
}

impl Default for ClipData {
    fn default() -> Self {
        let mut data: Vec<Mutex<Option<String>>> = Vec::new();

        for i in 0..=9 {
            data.insert(i, Mutex::new(None));
        }
        Self { data }
    }
}

impl ClipData
{
    pub fn load_to_serializable(&self) -> Vec<Option<String>> {
        let mut result = Vec::new();
        for i in self.data.iter()
        {
            match i.lock() {
                Ok(obj) => {
                    result.push(obj.clone());
                },
                Err(_) => {}
            }
        }

        result
    }

    pub fn load_from_serialized(&mut self, no_mutex: Vec<Option<String>>) {
        let mut data: Vec<Mutex<Option<String>>> = Vec::new();

        for i in no_mutex {
            let mutex = Mutex::new(i);
            data.push(mutex);
        }

        self.data = data;
    }
}

fn get_set_callback(clip_data: Arc<ClipData>, index: usize) -> impl Fn() {
    move || {
        let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

        let mut output: String = String::new();
        match formats::Unicode.read_clipboard(&mut output) {
            Ok(_) => match clip_data.data[index].lock() {
                Ok(mut v) => {
                    *v = Some(output);
                    _ = Notification::new()
                        .body(&format!("Saved in slot {}", index))
                        .show();
                }
                Err(e) => println!("{:?}", e),
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

fn get_read_callback(clip_data: Arc<ClipData>, index: usize) -> impl Fn() {
    move || {
        let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
        match clip_data.data[index].lock() {
            Ok(v) => match &*v {
                Some(string) => match Unicode.write_clipboard(string) {
                    Ok(_) => {
                        _ = Notification::new()
                            .body(&format!("Retreived value {}.", index))
                            .show();
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                },
                None => {
                    _ = Notification::new()
                        .body(&format!("No value saved in {}", index))
                        .show();
                }
            },
            Err(_) => {}
        }
    }
}


fn get_default() -> ClipData
{
    match File::open("data.yaml") {
        Ok(mut file) => {
            let mut file_data: String = String::new();
            file.read_to_string(&mut file_data).unwrap();
            match serde_yaml::from_str::<Vec<Option<String>>>(&file_data) {
                Ok(values) => {
                    let mut result: ClipData = ClipData::default();
                    result.load_from_serialized(values);
                    result
                },
                Err(_) => ClipData::default()
            }
        },
        Err(_) => ClipData::default()
    }
}
fn main() {
    let clip = get_default();

    let clip_data = Arc::new(clip);
    setup_interrupt(clip_data.clone());

   



    for i in 0..=9 {
        register_hotkey(
            &[
                Keyboard::LeftControl,
                Keyboard::LeftShift,
                Keyboard::LeftAlt,
                number_to_key(i),
            ],
            get_set_callback(clip_data.clone(), i),
        );

        register_hotkey(
            &[Keyboard::LeftControl, Keyboard::LeftAlt, number_to_key(i)],
            get_read_callback(clip_data.clone(), i),
        );
    }

    loop {}
}


fn setup_interrupt(clip_arc: Arc<ClipData>)
{
    ctrlc::set_handler(move || {
        let data = serde_yaml::to_string(&clip_arc.load_to_serializable());
        let mut file = File::create("data.yaml").expect("Could not open save directory.");
        file.write_all(&data.unwrap().as_bytes()).unwrap();
        exit(0);
    }).expect("Could not set interrupt handler.");
}

fn number_to_key(number: usize) -> Keyboard {
    match number {
        0 => Keyboard::Number0,
        1 => Keyboard::Number1,
        2 => Keyboard::Number2,
        3 => Keyboard::Number3,
        4 => Keyboard::Number4,
        5 => Keyboard::Number5,
        6 => Keyboard::Number6,
        7 => Keyboard::Number7,
        8 => Keyboard::Number8,
        9 => Keyboard::Number9,
        _ => panic!("Invalid input"),
    }
}
