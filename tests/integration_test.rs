//! Integration tests for SigmaOS-glx-vendor-libraries.

use sigmaos_glx_vendor_libraries_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
