use clippy_crash::Message;

#[derive(Message)]
#[repr(u32)]
pub enum Foo {
    Bar = 0, // clippy doesn't like this
    // Bar = 0u32, // clippy's okay with this instead
}
