use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {}

type BaseNode = Node2D;

#[methods]
impl Game {
    fn _init(_owner: BaseNode) -> Self {
        Game{}
    }

    #[export]
    fn _ready(&self, _owner: BaseNode) {
        gdnative::godot_print!("this is Game scene");
    }
}
