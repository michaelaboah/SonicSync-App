#[macro_use]
extern crate objc;

use base64::engine::{self, general_purpose};
use std::ffi;

use cocoa::base::id;
#[cfg(target_os = "macos")]
use helper::NSString;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    rc::StrongPtr,
    runtime::{Object, Sel, BOOL},
    sel, sel_impl,
};

#[test]
fn example() {
    let bytes = std::fs::read("example.txt").unwrap();

    let c = class!(NSConcretePrintOperation);

    for ivars in c.instance_methods().iter() {
        if ivars.name().name().to_lowercase().contains("run") {
            dbg!(ivars.name().name());
        }
    }

    // Allocate an instance of NSData
    let ns_data: id = unsafe {
        let ns_data: id = msg_send![class!(NSData), alloc];
        let ns_data: id = msg_send![ns_data, initWithBytes:bytes.as_ptr() length:bytes.len()];
        ns_data
    };

    // let superclass = class!(PDFDocument);
}

pub mod helper {
    use cocoa::base::id;
    use std::ffi::c_char;
    const UTF8_ENCODING: usize = 4;

    pub struct NSString(id);

    impl NSString {
        pub fn new(s: &str) -> Self {
            // Safety: objc runtime calls are unsafe
            NSString(unsafe {
                let ns_string: id = msg_send![class!(NSString), alloc];
                let ns_string: id = msg_send![ns_string,
                            initWithBytes:s.as_ptr()
                            length:s.len()
                            encoding:UTF8_ENCODING];

                // The thing is allocated in rust, the thing must be set to autorelease in rust to relinquish control
                // or it can not be released correctly in OC runtime
                let _: () = msg_send![ns_string, autorelease];

                ns_string
            })
        }

        pub fn to_str(&self) -> &str {
            unsafe {
                let bytes: *const c_char = msg_send![self.0, UTF8String];
                let len = msg_send![self.0, lengthOfBytesUsingEncoding: UTF8_ENCODING];
                let bytes = std::slice::from_raw_parts(bytes as *const u8, len);
                std::str::from_utf8_unchecked(bytes)
            }
        }

        #[allow(dead_code)] // only used when `mac-proxy` feature is enabled
        fn to_cstr(&self) -> *const c_char {
            unsafe {
                let utf_8_string = msg_send![self.0, UTF8String];
                utf_8_string
            }
        }

        fn as_ptr(&self) -> id {
            self.0
        }
    }

    impl From<NSData> for NSString {
        fn from(value: NSData) -> Self {
            Self(unsafe {
                let ns_string: id = msg_send![class!(NSString), alloc];
                let ns_string: id = msg_send![ns_string, initWithData:value encoding:UTF8_ENCODING];
                let _: () = msg_send![ns_string, autorelease];

                ns_string
            })
        }
    }

    pub struct NSData(id);

    impl NSData {
        pub fn new(id: id) -> Self {
            Self(id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_string() {
        let s = "hello";

        let ns = NSString::new(s);

        assert_eq!(s, ns.to_str())
    }
}
