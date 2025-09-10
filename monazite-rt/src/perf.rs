use cortex_m::peripheral::DWT;

#[derive(Default)]
pub struct Window {
    pub ticks: u32,
    pub max: u32,
    pub sum: u32,
}

impl Window {
    #[inline(always)]
    pub fn tick(&mut self, counter: &mut Counter) {
        self.sum += counter.sum;
        self.max = self.max.max(counter.sum);
        counter.sum = 0;
        self.ticks += 1;
    }

    pub fn reset(&mut self) {
        self.sum = 0;
        self.max = 0;
        self.sum = 0;
        self.ticks = 0;
    }
}

#[derive(Default)]
pub struct Counter {
    pub samples: u32,
    pub sum: u32,

    start: u32,
}

impl Counter {
    #[inline(always)]
    pub fn begin(&mut self) {
        self.start = DWT::cycle_count();
    }

    #[inline(always)]
    pub fn end(&mut self) {
        self.sum += DWT::cycle_count().wrapping_sub(self.start);
        self.samples += 1;
    }
}
