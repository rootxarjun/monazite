use std::sync::Mutex;

use c2a_monazite_gpio_bind::{Error, Gpio as GpioBind, Value};

struct State {
    output: Vec<Value>,
    num_inputs: usize,
}

pub struct Gpio {
    state: Mutex<State>,
}

impl Gpio {
    #[must_use]
    pub fn new(num_outputs: usize, num_inputs: usize) -> Self {
        Self {
            state: Mutex::new(State {
                output: vec![Value::Low; num_outputs],
                num_inputs,
            }),
        }
    }
}

impl Default for Gpio {
    fn default() -> Self {
        Self::new(9, 6)
    }
}

impl GpioBind for Gpio {
    fn initialize(&self) -> Result<(), Error> {
        Ok(())
    }

    fn set_output(&self, port: u8, value: Value) -> Result<(), Error> {
        let mut state = self.state.lock().unwrap();
        let port = state.output.get_mut(port as usize).ok_or(Error::Port)?;
        *port = value;
        Ok(())
    }

    fn get_output(&self, port: u8) -> Result<Value, Error> {
        let state = self.state.lock().unwrap();
        let port = state.output.get(port as usize).ok_or(Error::Port)?;
        Ok(*port)
    }

    fn get_input(&self, port: u8) -> Result<Value, Error> {
        let state = self.state.lock().unwrap();
        if (port as usize) < state.num_inputs {
            // FIXME: Emulate inputs
            Ok(Value::High)
        } else {
            Err(Error::Port)
        }
    }
}
