use gdnative::*;
use gdnative::user_data::MutexData;

use crate::pad::Pad;

type BaseNode = Node2D;

pub struct Game {
    pad0_thrust: Vector2,
    pad1_thrust: Vector2,
    pad0: Pad,
    pad1: Pad,
    score0_label: Label,
    score1_label: Label
}

unsafe impl Send for Game {}

impl NativeClass for Game {
    type Base = BaseNode;
    type UserData = MutexData<Game>;

    fn class_name() -> &'static str {
        "Game"
    }

    fn init(_owner: Self::Base) -> Self {
        Self::_init(_owner)
    }

    fn register_properties(builder: &gdnative::init::ClassBuilder<Self>) {
        builder.add_property::<Vector2>("pad0_thrust")
            .with_getter(|game: &Game, _| game.pad0_thrust)
            .with_setter(|game: &mut Game, _: BaseNode, thrust: Vector2| game.pad0_thrust = thrust)
            .done();

        builder.add_property::<Vector2>("pad1_thrust")
            .with_getter(|game: &Game, _| game.pad1_thrust)
            .with_setter(|game: &mut Game, _: BaseNode, thrust: Vector2| game.pad1_thrust = thrust)
            .done();
    }
}

#[methods]
impl Game {
    fn _init(_owner: BaseNode) -> Self {
        Game {
            pad0_thrust: Vector2::new(0.0, 0.0),
            pad1_thrust: Vector2::new(0.0, 0.0),

            pad0: Pad{
                score: 0,
                linear_velocity: Vector2::new(0.0, 0.0)
            },
            pad1: Pad{
                score: 0,
                linear_velocity: Vector2::new(0.0, 0.0)
            },

            score0_label: Label::new(),
            score1_label: Label::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: BaseNode) {
        godot_print!("this is Game scene");

        let pad0_scene: RigidBody2D = owner
            .get_node(NodePath::from_str("Pad0"))
            .expect("Missing Pad0 node")
            .cast::<RigidBody2D>()
            .expect("Cannot cast");
        self.pad0 = Pad::init(pad0_scene);

        let pad1_scene: RigidBody2D = owner
            .get_node(NodePath::from_str("Pad1"))
            .expect("Missing Pad1 node")
            .cast::<RigidBody2D>()
            .expect("Cannot cast");
        self.pad1 = Pad::init(pad1_scene);

        self.score0_label = owner.get_node(NodePath::from_str("Score0"))
            .expect("Missing Score0 node")
            .cast::<Label>()
            .expect("Cannot cast");

        self.score1_label = owner.get_node(NodePath::from_str("Score1"))
            .expect("Missing Score1 node")
            .cast::<Label>()
            .expect("Cannot cast");
    }

    #[export]
    unsafe fn _process(&mut self, _owner: BaseNode, delta: f64) {
        let input = Input::godot_singleton();
        if Input::is_action_pressed(&input, GodotString::from_str("pad0_up")) {
            self.pad0_thrust = -Vector2::new(0.0, 400.0);
        } else if Input::is_action_pressed(&input, GodotString::from_str("pad0_down")) {
            self.pad0_thrust = Vector2::new(0.0, 400.0);
        } else if Input::is_action_pressed(&input, GodotString::from_str("pad1_up")) {
            self.pad1_thrust = -Vector2::new(0.0, 400.0);
        } else if Input::is_action_pressed(&input, GodotString::from_str("pad1_down")) {
            self.pad1_thrust = Vector2::new(0.0, 400.0);
        }

        let score0 = self.pad0.score.to_string();
        self.score0_label.set_text(GodotString::from_str(score0));

        let score1 = self.pad1.score.to_string();
        self.score1_label.set_text(GodotString::from_str(score1));
    }

    #[export]
    unsafe fn _physics_process(&mut self, _owner: BaseNode, delta: f64) {
        self.pad0.linear_velocity = self.pad0_thrust;
        self.pad0_thrust *= 0.95;
        self.pad1_thrust *= 0.95;
    }
}
