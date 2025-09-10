use c2a_monazite_wdt_bind::Wdt as WdtBind;

pub struct Wdt(core::marker::PhantomData<()>);

impl Wdt {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl WdtBind for Wdt {
    fn initialize(&self) {
        // no-op
    }

    fn clear(&self) {
        // no-op
    }

    fn enable(&self, _time: u32) {
        // no-op
    }
}
