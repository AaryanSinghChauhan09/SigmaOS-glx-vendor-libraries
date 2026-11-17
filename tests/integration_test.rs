// Integration test for SigmaOS-glx-vendor-libraries
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use glx_vendor_libraries as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
