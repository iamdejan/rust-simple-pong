use gdnative::*;
use gdnative::user_data::MutexData;

type BaseNode = RigidBody2D;

pub struct Pad {
    pub score: i32,
    pub linear_velocity: Vector2
}

unsafe impl Send for Pad {}

impl NativeClass for Pad {
    type Base = BaseNode;
    type UserData = MutexData<Pad>;

    fn class_name() -> &'static str {
        "Pad"
    }

    fn init(_owner: Self::Base) -> Self {
        Self::_init()
    }

    fn register_properties(builder: &gdnative::init::ClassBuilder<Self>) {
        builder.add_property::<i32>("score")
            .with_getter(|pad: &Pad, _| pad.score)
            .with_setter(|pad: &mut Pad, _: BaseNode, score: i32| pad.score = score)
            .done();

        builder.add_property::<Vector2>("linear_velocity")
            .with_getter(|pad: &Pad, _| pad.linear_velocity)
            .with_setter(|pad: &mut Pad, _: BaseNode, linear_velocity: Vector2| pad.linear_velocity = linear_velocity)
            .done();
    }
}

#[methods]
impl Pad {
    fn _init() -> Pad {
        Pad {
            score: 0,
            linear_velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: BaseNode) {
        godot_print!("Pad is ready");
        self.score = 0;
    }

    #[export]
    unsafe fn _physics_process(&self, mut owner: BaseNode, delta: f64) {
        owner.set_linear_velocity(self.linear_velocity);
    }
}
