mod frame;

pub use frame::*;

pub trait AppInput: Clone + PartialEq {}

pub trait AppState<Input>: Clone + PartialEq
where
    Input: AppInput,
{
    /// This function is called every tick.
    fn tick(&mut self);

    /// This function is called when an input is received.
    fn handle_input(&mut self, player_id: PlayerId, input: Input);
}

const MAX_TICKS_PER_UDPATE: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub u32);

#[derive(Clone, PartialEq)]
pub struct App<State, Input>
where
    State: AppState<Input>,
    Input: AppInput,
{
    frame_delay: u64,
    confirmed_state: State,
    confirmed_frame: Frame,
    current_state: State,
    current_frame: Frame,
    inputs: Vec<Input>,
    accumulated_in_seconds: f64,
    tick_rate_in_seconds: f64,
}

impl<State, Input> App<State, Input>
where
    State: AppState<Input>,
    Input: AppInput,
{
    pub fn new(initial_state: State, tick_rate_in_seconds: f64, frame_delay: u64) -> Self {
        App {
            frame_delay,
            current_frame: Frame::default(),
            confirmed_frame: Frame::default(),
            confirmed_state: initial_state.clone(),
            current_state: initial_state,
            inputs: Vec::new(),
            accumulated_in_seconds: 0.0,
            tick_rate_in_seconds,
        }
    }

    pub fn update(&mut self, delta_t_in_seconds: f64) {
        self.accumulated_in_seconds += delta_t_in_seconds;
        for _ in 0..MAX_TICKS_PER_UDPATE {
            if self.accumulated_in_seconds >= self.tick_rate_in_seconds {
                self.tick();
                self.accumulated_in_seconds -= self.tick_rate_in_seconds;
            } else {
                break;
            }
        }
    }

    pub fn register_remote_input(&mut self, player_id: PlayerId, input: Input, frame: Frame) {
        todo!("Buffer remote input")
    }

    pub fn register_local_input(&mut self, input: Input) {
        let execution_frame = self.current_frame + self.frame_delay;

        todo!("Buffer local input")
    }

    fn tick(&mut self) {
        // TODO: rollback stuff and input handling
        let should_rollback = false;

        if should_rollback {
            let mut working_frame = self.confirmed_frame;
            self.current_state = self.confirmed_state.clone();
            while self.current_frame.greater_than(working_frame) {
                // TODO: apply inputs for working frame

                self.current_state.tick();

                // TODO: save rollback state
                let is_new_confirmed_frame = false;
                if is_new_confirmed_frame {
                    self.confirmed_frame = working_frame;

                    // TODO: might be able to optimize this by reducing a clone
                    self.confirmed_state = self.current_state.clone();
                }

                working_frame += 1;
            }
        }

        self.current_state.tick();
        self.current_frame += 1;
    }

    pub fn get_state(&self) -> &State {
        &self.current_state
    }

    pub fn set_state(&mut self, new_state: State) {
        self.current_state = new_state;
    }
}
