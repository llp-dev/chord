**rkyv > ser > writer > core**

# Module: ser::writer::core

## Contents

**Structs**

- [`Buffer`](#buffer) - Wraps a byte buffer and equips it with [`Writer`].

---

## rkyv::ser::writer::core::Buffer

*Struct*

Wraps a byte buffer and equips it with [`Writer`].

Common uses include archiving in `#![no_std]` environments and archiving
small objects without allocating.

# Examples

```
use core::mem::MaybeUninit;

use rkyv::{
    access_unchecked,
    api::high::to_bytes_in,
    rancor::{Error, Strategy},
    ser::{writer::Buffer, Writer},
    util::Align,
    Archive, Archived, Serialize,
};

#[derive(Archive, Serialize)]
enum Event {
    Spawn,
    Speak(String),
    Die,
}

let event = Event::Speak("Help me!".to_string());
let mut bytes = Align([MaybeUninit::uninit(); 256]);
let buffer = to_bytes_in::<_, Error>(&event, Buffer::from(&mut *bytes))
    .expect("failed to serialize event");
let archived = unsafe { access_unchecked::<Archived<Event>>(&buffer) };
if let Archived::<Event>::Speak(message) = archived {
    assert_eq!(message.as_str(), "Help me!");
} else {
    panic!("archived event was of the wrong type");
}
```

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **Positional**
  - `fn pos(self: &Self) -> usize`
- **Writer**
  - `fn write(self: & mut Self, bytes: &[u8]) -> Result<(), E>`
- **From**
  - `fn from(bytes: &'a  mut [u8]) -> Self`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **From**
  - `fn from(bytes: &'a  mut [u8; N]) -> Self`
- **From**
  - `fn from(bytes: &'a  mut [MaybeUninit<u8>]) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(bytes: &'a  mut [MaybeUninit<u8>; N]) -> Self`



