fn main() {
    let mut state = State::default();
    std::io::stdin()
        .bytes()
        .flatten()
        .map(|b| b as char)
        .for_each(|c| state.step(c))
}

#[derive(Debug)]
pub enum State {
    Before(Before),
    Inside(Inside),
    After(After),
}

impl Default for State {
    fn default() -> Self {
        State::Before(Before::default())
    }
}

trait StateStep {
    type State;
    fn entry(&self) {}
    fn exit(&self) {}
    fn step(&self, c: char) -> Option<Self::State>;
}

impl State {
    pub fn step(&mut self, c: char) {
        if let Some(state) = self.step(c) {
            self.exit();
            *self = state;
            self.entry();
        }
    }
}

#[derive(Default)]
pub struct Before {}

impl StateStep for Before {
    fn state_step(&self, c: char) -> Option<State> {
        if !c.is_whitespace() {
            Some(State::Inside(Inside::default()))
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Inside {}

impl StateStep for Inside {
    fn state_step(&self, c: char) -> Option<State> {
        match c {
            '\n' => Some(State::Before(Before::default())),
            c if c.is_whitespace() => Some(State::After(After::default())),
            _ => {
                print!("{c}");
                None
            }
        }
    }
}

#[derive(Default)]
pub struct After {}

impl StateStep for After {
    fn state_step(&self, c: char) -> Option<State> {
        match c {
            '\n' => Some(State::Before(Before::default())),
            _ => None,
        }
    }
}

// NOT needed at all, if we used enum_dispatch
impl StateStep for State {
    fn state_entry(&self, c: char) {
        match self {
            State::Before(s) => s.state_entry(c),
            State::Inside(s) => s.state_entry(c),
            State::After(s) => s.state_entry(c),
        }
    }
    fn state_step(&self, c: char) -> Option<State> {
        match self {
            State::Before(s) => s.state_step(c),
            State::Inside(s) => s.state_step(c),
            State::After(s) => s.state_step(c),
        }
    }
    fn state_exit(&self, c: char) {
        match self {
            State::Before(s) => s.state_exit(c),
            State::Inside(s) => s.state_exit(c),
            State::After(s) => s.state_exit(c),
        }
    }
}
