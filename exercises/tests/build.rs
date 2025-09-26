//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // Tell Cargo to set an environment variable TEST_FOO at compile time
    // so the tests can read it at runtime.
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // Enable a cfg flag so that tests8 can early-return under feature `pass`.
    // Build scripts cannot add features dynamically, but we can set a cfg
    // value seen by the current crate at compile time.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
