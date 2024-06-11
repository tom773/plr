# Power Utilities for Linux/Hyprland

Another project I've spun up because the existing tools either don't exist, or are too bloated for my liking.
I wrote this one in Rust because I liked the look of Egui over the GUI frameworks in the Go ecosystem.

## Features
- [x] Shutdown
- [x] Reboot
- [x] Logout

## Pre-Installation
- Install Rust: https://www.rust-lang.org/tools/install
- Add the following windowrulev2 to your hyprland config:
```
windowrulev2 = float,title:^(pm)$
```
## Installation
```
git clone git@github.com:tom773/plr.git
cargo build
```
## Usage
- Bind a keypress or a button in waybar to the binary generated (target/debug). Will make this more streamlined later.
- Click the desired button.
