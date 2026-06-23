# lightcontroller
A learning project to get more familiar with rust.

## Description
Manages lights and their attributes, including setting and editing current state and brightness level. 
Uses a HashMap to store the lights with the name/location of the light as the key. Users can add lights.

## How to run
```bash
git clone https://github.com/hiiraeth/lightcontroller.git
cd lightcontroller
cargo run
```

## Usage
Toggling a light:
```
Pick from the following options:
| 1. Set brightness
| 2. Toggle light on/off
| 3. Add new light
Choice: 2

Enter your light name: desktop

brightness: 68
state: false
```

Adding a new light:
```
Pick from the following options:
| 1. Set brightness
| 2. Toggle light on/off
| 3. Add new light
Choice: 3

Light name: lamp
Brightness: 45
State (1 for on, 0 for off): 1
Light added.
```