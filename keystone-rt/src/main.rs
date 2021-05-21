#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message)]

mod entry;
mod frame;
mod klog;
mod panic;
mod syscall;
mod trap;
mod uart;
mod vm;

#[no_mangle]
extern "C" fn rt_main() {
    // initialize modules
    klog::klog_init().expect("failed to initialize klog module");
    log::debug!("It did not crash!");

    // execute U-mode program
    unsafe {
        riscv::register::sepc::write(0x400000);
        riscv::register::sstatus::set_spp(riscv::register::sstatus::SPP::User);
        #[rustfmt::skip]
        asm!(
            "csrw sscratch, sp",
            "li sp, 0x402000",
            "sret",
        );
    }
}
