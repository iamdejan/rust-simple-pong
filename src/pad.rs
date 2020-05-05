use gdnative::*;
use gdnative::user_data::MutexData;

type BaseNode = RigidBody2D;

pub struct Pad {
    pub score: i32,
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
    }
}

#[methods]
impl Pad {
    fn _init() -> Pad {
        Pad {
            score: 0
        }
    }

    #[export]
    fn _ready(&mut self, _owner: BaseNode) {
        godot_print!("Pad is ready");
        self.score = 0;
    }
}
