use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

// You can only have one impl derive methods.
// these will be exposed to Godot
// Internal methods can use a seperate impl block.
#[methods]
impl HelloWorld {
    fn new(_base: &Node) -> Self {
        HelloWorld
    }

    #[method]
    fn _ready(&self, #[base] _base: &Node) {
        godot_print!("Hello, Godot, from Rust!")
    }
}

// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    // Register HelloWorld
    handle.add_class::<HelloWorld>();
}

// Creates entry-points of dyn lib.
godot_init!(init);
