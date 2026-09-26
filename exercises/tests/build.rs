//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // tests7: set TEST_FOO to the current Unix timestamp.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("cargo:rustc-env=TEST_FOO={timestamp}");

    // tests8: enable the custom cfg `pass`.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
