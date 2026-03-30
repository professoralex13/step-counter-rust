use embassy_stm32::gpio::{Input, Level};

pub struct Debouncer<T: PartialEq> {
    required_poll_count: u16,

    state: T,

    new_state_counter: u16,

    has_changed: bool,
}

impl<T: PartialEq> Debouncer<T> {
    pub fn new(initial_state: T, required_poll_count: u16) -> Self {
        Self {
            required_poll_count,
            state: initial_state,
            new_state_counter: 0,
            has_changed: false,
        }
    }

    pub fn poll(&mut self, raw_state: T) {
        self.has_changed = false;

        if raw_state != self.state {
            self.new_state_counter += 1;

            if self.new_state_counter >= self.required_poll_count {
                self.state = raw_state;

                self.has_changed = true;

                self.new_state_counter = 0;
            }
        }
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    pub fn just_changed(&self) -> bool {
        self.has_changed
    }

    pub fn just_changed_to(&self, value: &T) -> bool {
        self.has_changed && &self.state == value
    }
}

pub struct DebouncedButton {
    pin: Input<'static>,

    debouncer: Debouncer<Level>,
}

impl DebouncedButton {
    pub fn new(pin: Input<'static>) -> Self {
        let initial_level = pin.get_level();

        Self {
            pin,
            debouncer: Debouncer::new(initial_level, 5),
        }
    }

    pub fn poll(&mut self) {
        self.debouncer.poll(self.pin.get_level())
    }

    pub fn get_level(&self) -> Level {
        *self.debouncer.state()
    }

    pub fn just_changed(&self) -> bool {
        self.debouncer.just_changed()
    }

    pub fn just_changed_to(&self, level: Level) -> bool {
        self.debouncer.just_changed_to(&level)
    }
}
