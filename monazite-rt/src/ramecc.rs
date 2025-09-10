use c2a_monazite_ramecc_bind::Ramecc as RameccBind;
use core::ops::Range;
use core::sync::atomic::{AtomicU32, Ordering};
use hal::dma::{
    mdma::{MdmaConfig, MdmaIncrement, MdmaTrigger, Stream0},
    traits::Direction,
    MasterTransfer, MemoryToMemory, Transfer,
};
use stm32h7xx_hal as hal;
use stm32h7xx_hal::stm32::{MDMA, RAMECC1, RAMECC2, RAMECC3};

const AXI_SRAM: Range<usize> = 0x2400_0000..0x2408_0000;
const DTCM: Range<usize> = 0x2000_0000..0x2002_0000;
const SRAM1_0: Range<usize> = 0x3000_0000..0x3001_0000;
const SRAM1_1: Range<usize> = 0x3001_0000..0x3002_0000;
const SRAM2_0: Range<usize> = 0x3002_0000..0x3003_0000;
const SRAM2_1: Range<usize> = 0x3003_0000..0x3004_0000;
const SRAM3: Range<usize> = 0x3004_0000..0x3004_8000;
const SRAM4: Range<usize> = 0x3800_0000..0x3801_0000;

const ECC_TARGETS: [Range<usize>; 7] = [AXI_SRAM, SRAM1_0, SRAM1_1, SRAM2_0, SRAM2_1, SRAM3, SRAM4];

// scrubbing entire memory in 2200 sec
// RAM size / src_buf size = memory scrub time
// -> (512 + 128 + 128 + 32 + 64) * 1024 [byte] / 400 [byte / sec] = 2211.84 sec
const MEMORY_SCRUB_INTERVAL_TICKS: u32 = 1000;
pub const MEMORY_SCRUB_WORDS: usize = 100; // src_buf size

type OptionTransfer<S> = Option<
    Transfer<
        S,
        hal::dma::MemoryToMemory<u32>,
        MemoryToMemory<u32>,
        &'static mut [u32],
        MasterTransfer,
    >,
>;

pub struct RamScrubber<S = Stream0<MDMA>>
where
    S: hal::dma::traits::MasterStream,
{
    target_ram_index: usize,
    dma_ptr: usize,
    dma_config: MdmaConfig,
    regs: Regs,
    // if we don't store the transfer, it will be dropped and the DMA will stop
    transfer: OptionTransfer<S>,
    counter: u32,
    stats: &'static EccStats,
}

impl<S> RamScrubber<S>
where
    S: hal::dma::traits::MasterStream + hal::dma::traits::Stream<Config = MdmaConfig>,
{
    pub fn new(
        dst_buf: &'static mut [u32; 1],
        stats: &'static EccStats,
        (ramecc1, ramecc2, ramecc3): (RAMECC1, RAMECC2, RAMECC3),
        dma_stream: S,
    ) -> Self {
        let target_ram_index = 0;
        let dma_ptr = ECC_TARGETS[target_ram_index].start as *mut u32;
        let src_buf = unsafe { core::slice::from_raw_parts_mut(dma_ptr, MEMORY_SCRUB_WORDS) };

        let dma_config = MdmaConfig::default()
            .trigger_mode(MdmaTrigger::Buffer)
            .buffer_length(4)
            .destination_increment(MdmaIncrement::Fixed)
            .source_increment(MdmaIncrement::Increment);

        let mut transfer: Transfer<_, _, _, _, _> = Transfer::init_master(
            dma_stream,
            MemoryToMemory::new(),
            &mut dst_buf[..],
            Some(src_buf),
            dma_config,
        );

        stats.set_scrubbing_interval(MEMORY_SCRUB_INTERVAL_TICKS);

        let mut ramecc = Regs::new(ramecc1, ramecc2, ramecc3);
        ramecc.enable_ecc();

        transfer.start(|_| {});

        Self {
            target_ram_index,
            dma_ptr: dma_ptr as usize,
            dma_config,
            regs: ramecc,
            transfer: Some(transfer),
            counter: 0,
            stats,
        }
    }

    pub fn tick(&mut self) {
        self.counter += 1;
        if self.counter >= self.stats.scrubbing_interval() {
            self.next();
            self.counter = 0;
        }
    }

    pub fn next(&mut self) {
        if self
            .transfer
            .as_ref()
            .is_some_and(stm32h7xx_hal::dma::Transfer::get_transfer_complete_flag)
        {
            let src_buf =
                unsafe { core::slice::from_raw_parts_mut(self.dma_ptr_next(), MEMORY_SCRUB_WORDS) };

            let old_transfer = self.transfer.take();
            let (dma_stream, mem2mem, dst_buf, _) = old_transfer.unwrap().free();
            let mut transfer: Transfer<_, _, _, _, _> = Transfer::init_master(
                dma_stream,
                mem2mem,
                &mut dst_buf[..],
                Some(src_buf),
                self.dma_config,
            );

            transfer.start(|_| {});

            self.transfer = Some(transfer);
        }
    }

    fn dma_ptr_next(&mut self) -> *mut u32 {
        self.dma_ptr = self.dma_ptr.wrapping_add(MEMORY_SCRUB_WORDS * 4); // add MEMORY_SCRUB_WORDS * 4 bytes (1 WORD)
        if !ECC_TARGETS[self.target_ram_index].contains(&self.dma_ptr) {
            if self.target_ram_index + 1 < ECC_TARGETS.len() {
                self.target_ram_index += 1;
            } else {
                self.incr_scrubbing_loop_count();
                self.target_ram_index = 0;
            }
            self.dma_ptr = ECC_TARGETS[self.target_ram_index].start;
        }

        self.dma_ptr as *mut u32
    }

    #[allow(clippy::too_many_lines)]
    pub fn handling_interrupt(&mut self) {
        let ramecc1 = &self.regs.ramecc1;
        let ramecc2 = &self.regs.ramecc2;
        let ramecc3 = &self.regs.ramecc3;

        let r1m1sr = ramecc1.m1sr.read(); // D1 M1 = AXI SRAM
        let r1m3sr = ramecc1.m3sr.read(); // D1 M3 = D0TCM
        let r1m4sr = ramecc1.m4sr.read(); // D1 M4 = D1TCM
        let r2m1sr = ramecc2.m1sr.read(); // D2 M1 = SRAM1_0
        let r2m2sr = ramecc2.m2sr.read(); // D2 M2 = SRAM1_1
        let r2m3sr = ramecc2.m3sr.read(); // D2 M3 = SRAM2_0
        let r2m4sr = ramecc2.m4sr.read(); // D2 M4 = SRAM2_1
        let r2m5sr = ramecc2.m5sr.read(); // D2 M5 = SRAM3
        let r3m1sr = ramecc3.m1sr.read(); // D3 M1 = SRAM4

        if r1m1sr.bits() != 0 {
            // AXI SRAM
            let fadd = ramecc1.m1far.read().fadd().bits();
            ramecc1.m1sr.reset();
            let addr = crate::ramecc::AXI_SRAM.start + fadd as usize * 8; // word size of AXI SRAM is 8;
            ramecc1.m1cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u64).read_volatile() };
            unsafe { (addr as *mut u64).write_volatile(data) };
            ramecc1.m1cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r1m1sr.sedcf().bit_is_set() {
                self.incr_single_error_count();
            }
            if r1m1sr.dedf().bit_is_set() {
                self.incr_double_error_count();
            }
            if r1m1sr.debwdf().bit_is_set() {
                self.incr_double_error_on_byte_write_count();
            }
        } else if r1m3sr.bits() != 0 {
            // D0TCM
            let fadd = ramecc1.m3far.read().fadd().bits();
            let fdatal = ramecc1.m3fdrl.read().fdatal().bits();
            ramecc1.m3sr.reset();
            let addr = crate::ramecc::DTCM.start + fadd as usize * 8;
            ramecc1.m3cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc1.m3cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            unsafe { (addr as *mut u32).write_volatile(fdatal) };
            if r1m3sr.sedcf().bit_is_set() {
                self.incr_dtcm_single_error_count();
            }
            if r1m3sr.dedf().bit_is_set() {
                self.incr_dtcm_double_error_count();
            }
            if r1m3sr.debwdf().bit_is_set() {
                self.incr_dtcm_double_error_on_byte_write_count();
            }
        } else if r1m4sr.bits() != 0 {
            // D1TCM
            let fadd = ramecc1.m4far.read().fadd().bits();
            ramecc1.m4sr.reset();
            let addr = crate::ramecc::DTCM.start + 4 + fadd as usize * 8;
            ramecc1.m4cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc1.m4cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r1m4sr.sedcf().bit_is_set() {
                self.incr_dtcm_single_error_count();
            }
            if r1m4sr.dedf().bit_is_set() {
                self.incr_dtcm_double_error_count();
            }
            if r1m4sr.debwdf().bit_is_set() {
                self.incr_dtcm_double_error_on_byte_write_count();
            }
        } else if r2m1sr.bits() != 0 {
            // SRAM1_0
            let fadd = ramecc2.m1far.read().fadd().bits();
            ramecc2.m1sr.reset();
            let addr = crate::ramecc::SRAM1_0.start + fadd as usize * 4;
            ramecc2.m1cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc2.m1cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r2m1sr.sedcf().bit_is_set() {
                self.incr_single_error_count();
            }
            if r2m1sr.dedf().bit_is_set() {
                self.incr_double_error_count();
            }
            if r2m1sr.debwdf().bit_is_set() {
                self.incr_double_error_on_byte_write_count();
            }
        } else if r2m2sr.bits() != 0 {
            // SRAM1_1
            let fadd = ramecc2.m2far.read().fadd().bits();
            ramecc2.m2sr.reset();
            let addr = crate::ramecc::SRAM1_1.start + fadd as usize * 4;
            ramecc2.m2cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc2.m2cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r2m2sr.sedcf().bit_is_set() {
                self.incr_single_error_count();
            }
            if r2m2sr.dedf().bit_is_set() {
                self.incr_double_error_count();
            }
            if r2m2sr.debwdf().bit_is_set() {
                self.incr_double_error_on_byte_write_count();
            }
        } else if r2m3sr.bits() != 0 {
            // SRAM2_0
            let fadd = ramecc2.m3far.read().fadd().bits();
            ramecc2.m3sr.reset();
            let addr = crate::ramecc::SRAM2_0.start + fadd as usize * 4;
            ramecc2.m3cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc2.m3cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r2m3sr.sedcf().bit_is_set() {
                self.incr_single_error_count();
            }
            if r2m3sr.dedf().bit_is_set() {
                self.incr_double_error_count();
            }
            if r2m3sr.debwdf().bit_is_set() {
                self.incr_double_error_on_byte_write_count();
            }
        } else if r2m4sr.bits() != 0 {
            // SRAM2_1
            let fadd = ramecc2.m4far.read().fadd().bits();
            ramecc2.m4sr.reset();
            let addr = crate::ramecc::SRAM2_1.start + fadd as usize * 4;
            ramecc2.m4cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc2.m4cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r2m4sr.sedcf().bit_is_set() {
                self.incr_single_error_count();
            }
            if r2m4sr.dedf().bit_is_set() {
                self.incr_double_error_count();
            }
            if r2m4sr.debwdf().bit_is_set() {
                self.incr_double_error_on_byte_write_count();
            }
        } else if r2m5sr.bits() != 0 {
            // SRAM3
            let fadd = ramecc2.m5far.read().fadd().bits();
            ramecc2.m5sr.reset();
            let addr = crate::ramecc::SRAM3.start + fadd as usize * 4;
            ramecc2.m5cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc2.m5cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r2m5sr.sedcf().bit_is_set() {
                self.incr_single_error_count();
            }
            if r2m5sr.dedf().bit_is_set() {
                self.incr_double_error_count();
            }
            if r2m5sr.debwdf().bit_is_set() {
                self.incr_double_error_on_byte_write_count();
            }
        } else if r3m1sr.bits() != 0 {
            // SRAM4
            let fadd = ramecc3.m1far.read().fadd().bits();
            ramecc3.m1sr.reset();
            let addr = crate::ramecc::SRAM4.start + fadd as usize * 4;
            ramecc3.m1cr.write(|w| {
                w.eccelen().clear_bit();
                w
            });
            let data = unsafe { (addr as *mut u32).read_volatile() };
            unsafe { (addr as *mut u32).write_volatile(data) };
            ramecc3.m1cr.write(|w| {
                w.eccelen().set_bit();
                w
            });
            if r3m1sr.sedcf().bit_is_set() {
                self.incr_single_error_count();
            }
            if r3m1sr.dedf().bit_is_set() {
                self.incr_double_error_count();
            }
            if r3m1sr.debwdf().bit_is_set() {
                self.incr_double_error_on_byte_write_count();
            }
        }
    }

    pub fn incr_scrubbing_loop_count(&mut self) {
        self.stats.scrubbing_loops.fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_single_error_count(&mut self) {
        self.stats.single_errors.fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_double_error_count(&mut self) {
        self.stats.double_errors.fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_double_error_on_byte_write_count(&mut self) {
        self.stats
            .double_errors_on_byte_write
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_dtcm_single_error_count(&mut self) {
        self.stats
            .dtcm_single_errors
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_dtcm_double_error_count(&mut self) {
        self.stats
            .dtcm_double_errors
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_dtcm_double_error_on_byte_write_count(&mut self) {
        self.stats
            .dtcm_double_errors_on_byte_write
            .fetch_add(1, Ordering::Relaxed);
    }
}

struct Regs {
    ramecc1: RAMECC1,
    ramecc2: RAMECC2,
    ramecc3: RAMECC3,
}

impl Regs {
    fn new(ramecc1: RAMECC1, ramecc2: RAMECC2, ramecc3: RAMECC3) -> Self {
        Self {
            ramecc1,
            ramecc2,
            ramecc3,
        }
    }

    fn enable_ecc(&mut self) {
        self.ramecc1.m1sr.reset();
        self.ramecc1.m1cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });
        self.ramecc1.m3cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });
        self.ramecc1.m4cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });

        self.ramecc2.m1sr.reset();
        self.ramecc2.m1cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });
        self.ramecc2.m2sr.reset();
        self.ramecc2.m2cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });
        self.ramecc2.m3sr.reset();
        self.ramecc2.m3cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });
        self.ramecc2.m4sr.reset();
        self.ramecc2.m4cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });
        self.ramecc2.m5sr.reset();
        self.ramecc2.m5cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });

        self.ramecc3.m1sr.reset();
        self.ramecc3.m1cr.write(|w| {
            w.eccelen().set_bit();
            w.eccseie().set_bit();
            w.eccdeie().set_bit();
            w.eccdebwie().set_bit();
            w
        });

        self.ramecc2.ier.write(|w| {
            w.geccseie().set_bit();
            w.geccdeie().set_bit();
            w.gie().set_bit();
            w
        });
        self.ramecc3.ier.write(|w| {
            w.geccseie().set_bit();
            w.geccdeie().set_bit();
            w.gie().set_bit();
            w
        });
    }
}

#[derive(Default)]
pub struct EccStats {
    pub scrubbing_loops: AtomicU32,
    pub single_errors: AtomicU32,
    pub double_errors: AtomicU32,
    pub double_errors_on_byte_write: AtomicU32,
    pub dtcm_single_errors: AtomicU32,
    pub dtcm_double_errors: AtomicU32,
    pub dtcm_double_errors_on_byte_write: AtomicU32,
    pub scrubbing_interval_tick: AtomicU32,
}

impl RameccBind for EccStats {
    fn scrubbing_loops(&self) -> u32 {
        self.scrubbing_loops.load(Ordering::Relaxed)
    }

    fn scrubbing_interval(&self) -> u32 {
        self.scrubbing_interval_tick.load(Ordering::Relaxed)
    }

    fn single_errors(&self) -> u32 {
        self.single_errors.load(Ordering::Relaxed)
    }

    fn double_errors(&self) -> u32 {
        self.double_errors.load(Ordering::Relaxed)
    }

    fn double_errors_on_byte_write(&self) -> u32 {
        self.double_errors_on_byte_write.load(Ordering::Relaxed)
    }

    fn dtcm_single_errors(&self) -> u32 {
        self.dtcm_single_errors.load(Ordering::Relaxed)
    }

    fn dtcm_double_errors(&self) -> u32 {
        self.dtcm_double_errors.load(Ordering::Relaxed)
    }

    fn dtcm_double_errors_on_byte_write(&self) -> u32 {
        self.dtcm_double_errors_on_byte_write
            .load(Ordering::Relaxed)
    }

    #[warn(unused_variables)]
    fn set_scrubbing_interval(&self, scrubbing_interval_tick: u32) {
        self.scrubbing_interval_tick
            .store(scrubbing_interval_tick, Ordering::Relaxed);
    }
}
