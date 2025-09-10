use c2a_monazite_thermometer_bind::Thermometer as ThermometerBind;

pub struct Thermometer(core::marker::PhantomData<()>);

impl Thermometer {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl ThermometerBind for Thermometer {
    fn value(&self) -> f32 {
        123.456
    }
}
