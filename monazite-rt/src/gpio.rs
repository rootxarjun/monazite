use core::cell::RefCell;

use c2a_monazite_gpio_bind::{Error, Gpio as GpioBind, Value};
use cortex_m::interrupt::Mutex;
use stm32h7xx_hal::gpio::{ErasedPin, Input, Output};

pub struct Gpio<'a> {
    outputs: Mutex<RefCell<&'a mut [ErasedPin<Output>]>>,
    inputs: &'a [ErasedPin<Input>],
}

impl<'a> Gpio<'a> {
    pub fn new(outputs: &'a mut [ErasedPin<Output>], inputs: &'a [ErasedPin<Input>]) -> Self {
        Self {
            outputs: Mutex::new(RefCell::new(outputs)),
            inputs,
        }
    }
}

impl GpioBind for Gpio<'_> {
    fn initialize(&self) -> Result<(), Error> {
        Ok(())
    }

    fn set_output(&self, port_id: u8, value: Value) -> Result<(), Error> {
        cortex_m::interrupt::free(|cs| {
            let mut outputs = self.outputs.borrow(cs).borrow_mut();
            let port = outputs.get_mut(port_id as usize).ok_or(Error::Port)?;
            port.set_state(bool::from(value).into());
            Ok(())
        })
    }

    fn get_output(&self, port_id: u8) -> Result<Value, Error> {
        cortex_m::interrupt::free(|cs| {
            let outputs = self.outputs.borrow(cs).borrow();
            let port = outputs.get(port_id as usize).ok_or(Error::Port)?;
            Ok(Value::from(port.is_set_high()))
        })
    }

    fn get_input(&self, port_id: u8) -> Result<Value, Error> {
        let port = self.inputs.get(port_id as usize).ok_or(Error::Port)?;
        Ok(Value::from(port.is_high()))
    }
}
