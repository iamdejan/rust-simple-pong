mod hello_world;
mod game;

use gdnative::*;
use self::hello_world::*;
use self::game::*;

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<Game>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
