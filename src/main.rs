use std::io::{stdout, Write};
use std::collections::HashMap;

enum State {
    On,
    Off
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::On => { write!(f, "on") },
            State::Off => { write!(f, "off") }
        }
    }
}

struct Light {
    brightness: f32,
    state: State
}


impl Light {
    fn new(brightness: f32, state: State) -> Light {
        Light {brightness, state}
    }

    fn set_brightness(&mut self, brightness: f32) {
        if brightness > 100.0 {
            println!("Brightness is too high!");
            return
        }

        if brightness < 0.0 {
            println!("Brightness is too low!");
            return
        }

        self.brightness = brightness;
    }

    fn toggle_state(&mut self) {
        match self.state {
            State::Off => { self.state = State::On },
            State::On => { self.state = State::Off }
        }
    }

    fn get_all(&self) -> String {
        format!("\nbrightness: {}\nstate: {}", self.brightness, self.state)
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();

    input.trim().to_string()
}

fn get_light(lights: &mut HashMap<String, Light>) -> Option<&mut Light> {
    // get user input
    let name = prompt("\nEnter your light name: ").to_lowercase();
    lights.get_mut(name.trim())
}

fn menu(lights: &mut HashMap<String, Light>) {
    let choice = prompt("\nPick from the following options:\n\
              | 1. Set brightness\n\
              | 2. Toggle light on/off\n\
              | 3. Add new light\n\
              | 4. List all lights\n\
              Choice: ");

    match choice.trim() {
        "1" => {
            let Some(light) = get_light(lights) else {
                println!("Light does not exist!");
                return;
            };
            let bright_str = prompt("Brightness: ");

            let Ok(brightness) = bright_str.trim().parse() else {
                println!("Invalid input"); return
            };

            light.set_brightness(brightness);
            println!("{}", light.get_all());
        }
        "2" => {
            let Some(light) = get_light(lights) else {
                println!("Light does not exist!");
                return;
            };
            light.toggle_state();
            println!("{}", light.get_all());
        }
        "3" => {
            let light_name = prompt("\nLight name: ").to_lowercase().trim().to_string();

            let brightness_str = prompt("Brightness: ");
            let Ok(brightness) = brightness_str.trim().parse() else {
                println!("Invalid input"); return
            };

            let state_str = prompt("State (1 for on, 0 for off): ");
            let state: State;
            match state_str.trim() {
                "1" => { state = State::On; }
                "0" => { state = State::Off; }
                _ => { println!("Invalid input"); return }
            }

            let new_light = Light::new(brightness, state);
            match lights.insert(light_name, new_light) {
                None => println!("Light added."),
                Some(_) => println!("Replaced existing light.")
            }
        }
        "4" => {
            for (name, light) in lights {
                println!("{}{}\n", name, light.get_all());
            }
        }
        _ => println!("Invalid choice!"),
    }
}

fn main() {
    let mut lights: HashMap<String, Light> = HashMap::new();

    let desk_light = Light::new(68.0, State::On);
    let overhead_light = Light::new(75.0, State::Off);

    lights.insert("desktop".to_string(), desk_light);
    lights.insert("overhead".to_string(), overhead_light);

    loop {
        menu(&mut lights);
    }
}