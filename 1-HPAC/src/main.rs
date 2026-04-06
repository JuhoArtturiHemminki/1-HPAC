#![allow(static_mut_refs)]
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

pub const PHI: f64 = 1.618033988749895;
pub const H_C: f64 = 5.0832104;

pub struct FractalManifold {
    pub drift: f64,
    pub resonance: f64,
    pub index: u64,
}

impl FractalManifold {
    pub const fn new() -> Self {
        Self {
            drift: 0.0,
            resonance: 79.111933,
            index: 0,
        }
    }

    pub fn step(&mut self, entropy: f64) -> (f64, f64) {
        let correction = (entropy % PHI) / H_C;
        if entropy > 0.0000000001 {
            self.drift = (self.drift + correction) / PHI;
        } else {
            self.drift /= PHI;
        }

        let mut p_inv = 1.0;
        let power = (self.index % 7) as i32;
        for _ in 0..power {
            p_inv /= PHI;
        }

        let res = self.resonance * (1.0 + (self.drift * p_inv));
        let shield = (res * (PHI * PHI)) + (self.drift.abs() * 1.618);

        self.index = self.index.wrapping_add(1);
        (res, shield)
    }
}

pub struct GoldilocksLimiter {
    pub target: f64,
}

impl GoldilocksLimiter {
    pub const fn new(t: f64) -> Self {
        Self { target: t }
    }

    pub fn balance(&self, current: f64, drift: &mut f64) {
        let d = current - self.target;
        *drift += (d * PHI) / H_C;
    }
}

static mut MANIFOLD: FractalManifold = FractalManifold::new();
static mut LIMITER: GoldilocksLimiter = GoldilocksLimiter::new(0.21);

#[inline(always)]
unsafe fn msr(m: u32, v: u64) {
    let low = (v & 0xFFFFFFFF) as u32;
    let high = (v >> 32) as u32;
    asm!("wrmsr", in("ecx") m, in("eax") low, in("edx") high);
}

#[inline(always)]
unsafe fn entropy() -> f64 {
    let low: u32;
    asm!("rdmsr", in("ecx") 0x19C, out("eax") low, out("edx") _);
    ((low >> 16) & 0x7F) as f64 / 127.0
}

#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(
    _h: *const core::ffi::c_void,
    _s: *const core::ffi::c_void,
) -> usize {
    asm!("cli");

    msr(0x1A2, 0x00000000_00006400);
    msr(0x610, 0x00007FFF_00007FFF);
    msr(0x1FC, 0x00000001);
    msr(0x199, 0x13D4);

    loop {
        let e = entropy();
        LIMITer.balance(e, &mut MANIFOLD.drift);

        let (r, s) = MANIFOLD.step(e);

        let v_tune = (r * 1000.0) as u32;
        let s_tune = (s * 10.0) as u32;

        asm!("out dx, eax", in("dx") 0xCF8_u16, in("eax") v_tune);
        asm!("out dx, al", in("dx") 0x3C8_u16, in("al") (s_tune & 0xFF) as u8);

        if MANIFOLD.drift.abs() < 0.0000000001 {
            asm!("wbinvd");
        }

        asm!("pause");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
