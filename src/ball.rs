use gdnative::*;

type BaseNode = RigidBody2D;

#[derive(NativeClass)]
#[inherit(BaseNode)]
pub struct Ball {
    pub timer: Timer
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

    pub unsafe fn reset(&mut self, mut owner: BaseNode) {
        owner.get_global_position().x = 640.0;
        owner.get_global_position().y = 30.0;
        owner.set_linear_velocity(Vector2::new(0.0, 0.0));
    }

    #[export]
    unsafe fn _process(&mut self, owner: BaseNode, _delta: f64) {
        if self.timer.is_stopped() {
            godot_print!("{}", owner.get_global_position());
            return;
        }

        if self.timer.get_time_left() <= 0.1 {
            godot_print!("Time out!");
            self.on_timer_timeout(owner);
        }
    }

    #[export]
    unsafe fn _physics_process(&self, mut owner: BaseNode, _delta: f64) {
        owner.set_linear_velocity(owner.get_linear_velocity() * 1.0025);
    }

    unsafe fn on_timer_timeout(&mut self, mut owner: BaseNode) {
        owner.set_linear_velocity(Vector2::new(-500.0, 300.0));
        owner.set_physics_process(true);
        self.timer.stop();
    }
}
