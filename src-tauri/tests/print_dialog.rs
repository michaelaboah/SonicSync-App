#[macro_use]
extern crate objc;

use base64::engine::{self, general_purpose};
use std::ffi;
use swift_bridge::SwiftBridge;

#[test]
fn example() {
    let pdf_data = std::fs::read("example.txt").unwrap();
    let abs_path = std::fs::canonicalize("./example.txt")
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();

    let string = "Hello World";
    let bytes = string.as_bytes();

    let swift_bridge = SwiftBridge::load_library("/path/to/swift_library.dylib").unwrap();

    // Invoke the Swift function or method
    swift_bridge
        .invoke("AppDelegate.applicationDidFinishLaunching(_:)", &[])
        .unwrap();
}
