use c2a_monazite_adc_bind::{Error, InputChannelId, TestChannelId};

pub struct Adc(core::marker::PhantomData<()>);

impl Adc {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl c2a_monazite_adc_bind::Adc for Adc {
    fn initialize(&self) -> Result<(), Error> {
        Ok(())
    }

    fn get_value(&self, ch: InputChannelId) -> u16 {
        match u8::from(ch) {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => unreachable!("out of range"),
        }
    }

    fn get_test_value(&self, ch: TestChannelId) -> u16 {
        const MAX_12BIT: u16 = (1 << 12) - 1;
        match u8::from(ch) {
            0 => MAX_12BIT / 2,
            1 => MAX_12BIT,
            _ => unreachable!("out of range"),
        }
    }
}
