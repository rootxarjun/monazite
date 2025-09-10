use std::convert::Infallible;

use c2a_monazite_iflash_bind::{Error, Iflash as IflashBind};

pub struct Iflash(core::marker::PhantomData<()>);

impl Iflash {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl IflashBind for Iflash {
    fn start_erase(&self) -> nb::Result<(), Infallible> {
        Ok(())
    }

    fn start_program(&self, _offset: usize, _data: &[u8]) -> nb::Result<(), Error> {
        Ok(())
    }

    fn status(&self) -> nb::Result<(), Error> {
        Ok(())
    }
}
