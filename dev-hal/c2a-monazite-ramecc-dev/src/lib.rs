use c2a_monazite_ramecc_bind::Ramecc as RameccBind;

pub struct Ramecc(core::marker::PhantomData<()>);

impl Ramecc {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Ramecc(core::marker::PhantomData)
    }
}

impl RameccBind for Ramecc {
    fn scrubbing_loops(&self) -> u32 {
        0
    }

    fn scrubbing_interval(&self) -> u32 {
        0
    }

    fn single_errors(&self) -> u32 {
        0
    }

    fn double_errors(&self) -> u32 {
        0
    }

    fn double_errors_on_byte_write(&self) -> u32 {
        0
    }

    fn dtcm_single_errors(&self) -> u32 {
        0
    }

    fn dtcm_double_errors(&self) -> u32 {
        0
    }

    fn dtcm_double_errors_on_byte_write(&self) -> u32 {
        0
    }

    fn set_scrubbing_interval(&self, _scrubbing_interval_tick: u32) {}
}
