//! SigmaOS-glx-vendor-libraries — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_glx_vendor_libraries_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-glx-vendor-libraries: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-glx-vendor-libraries: ok");
}
