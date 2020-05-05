use gdnative::*;

type BaseNode = RigidBody2D;

#[derive(NativeClass)]
#[inherit(BaseNode)]
pub struct Ball {
    timer: Timer
}

#[methods]
impl Ball {
    fn _init(_owner: BaseNode) -> Self {
        Ball {
            timer: Timer::new()
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: BaseNode) {
        self.timer = owner
            .get_node(NodePath::from_str("Timer"))
            .expect("Missing Timer node")
            .cast::<Timer>()
            .expect("Cannot cast to timer");

        self.reset(owner);
    }

    unsafe fn reset(&mut self, mut owner: BaseNode) {
        owner.set_global_position(Vector2::new(640.0, 30.0));
        owner.set_physics_process(false);
        self.timer.start(self.timer.get_wait_time());
        owner.set_linear_velocity(Vector2::new(0.0, 0.0));
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: BaseNode) {
        owner.set_linear_velocity(owner.get_linear_velocity() * 1.25);
    }
}
