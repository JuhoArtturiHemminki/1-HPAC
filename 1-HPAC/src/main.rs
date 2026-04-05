#![no_std]
#![no_main]

mod fractal_feedback;
mod thermal_limiter;

use core::arch::asm;
use core::panic::PanicInfo;
use crate::fractal_feedback::FractalManifold;
use crate::thermal_limiter::GoldilocksLimiter;

static mut MANIFOLD: FractalManifold = FractalManifold::new();
static mut LIMITER: GoldilocksLimiter = GoldilocksLimiter::new(0.21);

#[inline(always)]
unsafe fn sync_msr(msr: u32, value: u64) {
    let low = (value & 0xFFFFFFFF) as u32;
    let high = (value >> 32) as u32;
    asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
}

#[inline(always)]
unsafe fn read_entropy() -> f64 {
    let eax: u32;
    asm!("rdmsr", in("ecx") 0x19C, out("eax") eax, out("edx") _);
    (eax as f64) / 127.0
}

#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(_handle: *const core::ffi::c_void, 
_st: *const core::ffi::c_void) -> usize {
    asm!("cli");
    sync_msr(0x199, 0x13D4);
    sync_msr(0x1A0, 0x0800);
    
    loop {
        let entropy = read_entropy();
        let resonance = MANIFOLD.calculate_fractal_step(entropy);
        let _energy = LIMITER.balance(entropy, &mut MANIFOLD.drift);
        
        let v_tune = (resonance * 1000.0) as u32;
        asm!("out dx, eax", in("dx") 0xCF8_u16, in("eax") v_tune);
        
        let vga_sync = 0x1F_u8;
        asm!("out dx, al", in("dx") 0x3C8_u16, in("al") vga_sync);

        if MANIFOLD.drift.abs() < 0.0000000001 {
            asm!("wbinvd");
        }
        asm!("pause");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { unsafe { asm!("pause"); } }
}

