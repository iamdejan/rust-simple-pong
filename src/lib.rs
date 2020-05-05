mod hello_world;
mod game;
mod pad;
mod ball;

use gdnative::*;
use self::hello_world::*;
use self::game::*;
use self::pad::*;
use self::ball::*;

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<Game>();
    handle.add_class::<Pad>();
    handle.add_class::<Ball>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
