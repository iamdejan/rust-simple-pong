use gdnative::*;

type BaseNode = Node2D;

#[derive(NativeClass)]
#[inherit(BaseNode)]
pub struct Game;

#[methods]
impl Game {
    fn _init(_owner: BaseNode) -> Self {
        Game
    }

    #[export]
    fn _ready(&self, _owner: BaseNode) {
        gdnative::godot_print!("this is Game scene");
    }
}
