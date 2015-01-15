use time::precise_time_ns;

pub struct State {
    delta_time: u64,
    last_time: u64,
}

impl State {
    pub fn new() -> State {
        State {
            delta_time: 0,
            last_time: precise_time_ns(),
        }
    }

    pub fn tick(&mut self) {
         let time = precise_time_ns();
         self.delta_time = time - self.last_time;
         self.last_time = time;
         println!("tick: {}", self.delta_time);
    }
}
