use std::ffi::CString;
use std::os::raw::c_char;

use serde_json;
use serde::de::DeserializeOwned;

#[doc(hidden)]
pub mod exports;
mod events;

/// An ID representing an atom.
///
/// This ID acts as a handle that we can use across FFI boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AtomId(u32);

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash)]
#[repr(u32)]
pub enum EventType {
    Click,
    MouseDown,
    MouseUp,
}

impl EventType {
    pub fn from(raw: u32) -> Option<EventType> {
        Some(match raw {
            0 => EventType::Click,
            1 => EventType::MouseDown,
            2 => EventType::MouseUp,
            _ => return None,
        })
    }
}

impl AtomId {
    pub fn root() -> Self {
        AtomId(0)
    }

    pub fn wrap(id: u32) -> Self {
        AtomId(id)
    }
}

/// Environment imports.
extern {
    fn blocks_out_println(ptr: *const c_char, len: usize);
    fn blocks_out_defer(f: extern fn());
    fn blocks_out_mount_id(length: *mut [u8; 4]) -> *mut u8;
    fn blocks_out_create_element(ptr: *const c_char, len: usize) -> u32;
    fn blocks_out_create_text_node(ptr: *const c_char, len: usize, parent: u32) -> u32;
    fn blocks_out_update_text_node(ptr: *const c_char, len: usize, id: u32);
    fn blocks_out_delete_node(id: u32);
    fn blocks_out_node_text_to_element(ptr: *const c_char, len: usize);
    fn blocks_out_element_to_text_node(ptr: *const c_char, len: usize, id: u32);
    fn blocks_out_update_element(id: u32, ptr: *const c_char, len: usize);
    fn blocks_out_create_event(atom: u32, type_: u32);
    fn blocks_out_delete_event(atom: u32, type_: u32);
    fn blocks_out_inject_stylesheet(ptr: *const c_char, len: usize);
}

pub fn defer(f: extern fn()) {
    unsafe {
        blocks_out_defer(f);
    }
}

pub fn println<T>(string: T) where T: AsRef<str> {
    let len = string.as_ref().len();

    let ffi_string = CString::new(string.as_ref()).unwrap();

    unsafe {
        blocks_out_println(ffi_string.as_ptr(), len);
    }
}

/// Recreate a rust string given a little endian length buffer and a pointer.
unsafe fn read_string(length_buf: [u8; 4], ptr: *mut u8) -> String {
    let length =
        (((length_buf[0] as usize) << 0)  & 0x000000ff) +
        (((length_buf[1] as usize) << 8)  & 0x0000ff00) +
        (((length_buf[2] as usize) << 16) & 0x00ff0000) +
        (((length_buf[3] as usize) << 24) & 0xff000000);

    String::from_raw_parts(ptr, length, length)
}

pub fn mount_id() -> String {
    let mut length_buf = [0u8; 4];

    unsafe {
        let ptr = blocks_out_mount_id(&mut length_buf);

        read_string(length_buf, ptr)
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
    pub fn new<K, V>(key: K, value: V) -> Attribute
    where
        K: Into<String>,
        V: Into<String>,
    {
        Attribute {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Into<(String, String)> for Attribute {
    fn into(self) -> (String, String) {
        (self.key, self.value)
    }
}

pub fn create_element(attributes: Vec<Attribute>, AtomId(parent): AtomId) -> AtomId {
    #[derive(Serialize)]
    struct CreateElement {
        attributes: Vec<(String, String)>,
        parent: u32,
    }

    let params = CreateElement {
        attributes: attributes
            .into_iter()
            .map(|attr| attr.into())
            .collect(),
        parent,
    };

    let json = serde_json::to_string(&params).unwrap();

    let len = json.len();
    let ffi_string = CString::new(json).unwrap();

    AtomId(unsafe {
        blocks_out_create_element(ffi_string.as_ptr(), len)
    })
}

pub fn create_text_node<T>(text: T, AtomId(parent): AtomId) -> AtomId where T: AsRef<str> {
    let text = text.as_ref();

    let len = text.len();
    let ffi_string = CString::new(text).unwrap();

    AtomId(unsafe {
        blocks_out_create_text_node(ffi_string.as_ptr(), len, parent)
    })
}

pub fn update_text_node<T>(text: T, AtomId(id): AtomId) where T: AsRef<str> {
    let text = text.as_ref();

    let len = text.len();
    let ffi_string = CString::new(text).unwrap();

    unsafe {
        blocks_out_update_text_node(ffi_string.as_ptr(), len, id);
    }
}

pub fn delete_node(AtomId(id): AtomId) {
    unsafe {
        blocks_out_delete_node(id);
    }
}

pub fn text_node_to_element(AtomId(id): AtomId, attributes: Vec<Attribute>) {
    #[derive(Serialize)]
    struct TextToElement {
        id: u32,
        attributes: Vec<(String, String)>,
    }

    let params = TextToElement {
        id,
        attributes: attributes
            .into_iter()
            .map(|attr| attr.into())
            .collect(),
    };

    let json = serde_json::to_string(&params).unwrap();

    let len = json.len();
    let ffi_string = CString::new(json).unwrap();

    unsafe {
        blocks_out_node_text_to_element(ffi_string.as_ptr(), len);
    }
}

pub fn element_to_text_node<T>(AtomId(id): AtomId, text: T) where T: AsRef<str> {
    let text = text.as_ref();

    let len = text.len();
    let ffi_string = CString::new(text).unwrap();

    unsafe {
        blocks_out_element_to_text_node(ffi_string.as_ptr(), len, id);
    }
}

pub fn update_element(AtomId(id): AtomId, attributes: Vec<Attribute>) {
    #[derive(Serialize)]
    struct UpdateElement {
        attributes: Vec<(String, String)>,
    }

    let params = UpdateElement {
        attributes: attributes
            .into_iter()
            .map(|attr| attr.into())
            .collect(),
    };

    let json = serde_json::to_string(&params).unwrap();

    let len = json.len();
    let ffi_string = CString::new(json).unwrap();

    unsafe {
        blocks_out_update_element(id, ffi_string.as_ptr(), len);
    }
}

pub fn inject_stylesheet<T>(sheet: T) where T: AsRef<str> {
    let sheet = sheet.as_ref();

    let len = sheet.len();
    let ffi_string = CString::new(sheet).unwrap();

    unsafe {
        blocks_out_inject_stylesheet(ffi_string.as_ptr(), len);
    }
}

pub fn create_event<F, T>(id: AtomId, type_: EventType, callback: F)
where
    F: 'static + Send + Fn(T),
    T: DeserializeOwned,
{
    events::create_event(id, type_, move |s: String| {
        let deserialized = serde_json::from_str(&s).unwrap();

        callback(deserialized);
    });

    unsafe {
        blocks_out_create_event(id.0, type_ as u32);
    }
}
