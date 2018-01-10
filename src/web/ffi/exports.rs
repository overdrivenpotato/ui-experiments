// TODO: Move this to `src/web/ffi/exports.rs`.
//! Web exports.

use std::mem;

use super::{events, EventType};
use super::AtomId;

/// Invoke a callback that accepts 0 arguments.
#[no_mangle]
pub extern fn blocks_in_callback0(f: extern fn()) {
    f();
}

/// Allocate memory for transferring strings.
#[no_mangle]
pub extern fn blocks_in_create_string(length: usize) -> *const u8 {
    let s = String::with_capacity(length);
    let ptr = s.as_ptr();
    mem::forget(s);

    ptr
}

/// Call back a registered event.
#[no_mangle]
pub unsafe extern fn blocks_in_callback_event(atom: u32, type_: u32, ptr: *mut u8, len: usize) {
    if let Some(type_) = EventType::from(type_) {
        events::call(type_, AtomId::wrap(atom), String::from_raw_parts(ptr, len, len));
    }
}
