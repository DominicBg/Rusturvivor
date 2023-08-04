#[derive(Debug, Default, Copy, Clone, AsBindGroup, TypeUuid)]
#[uuid = "786779ff-e3ac-4b36-ae96-f4844f8e3064"]
struct MyRustGpuMaterial {
    #[uniform(0)]
    color: Vec4,
}

// The vertex and fragment shaders specified here can be used
// as a fallback when entrypoints are unavailable
// (see the documentation of bevy_rust_gpu::prelude::RustGpuSettings),
// but are otherwise deferred to ShaderRef::Default, so can be left unimplemented.
impl Material for MyRustGpuMaterial {}

// First, implement some marker structs to represent our shader entry points

pub enum MyVertex {}

impl EntryPoint for MyVertex {
    const NAME: EntryPointName = "vertex";
    const PARAMETERS: EntryPointParameters = &[];
    const CONSTANTS: EntryPointConstants = &[];
}

pub enum MyFragment {}

impl EntryPoint for MyFragment {
    const NAME: EntryPointName = "fragment";
    const PARAMETERS: EntryPointParameters = &[];
    const CONSTANTS: EntryPointConstants = &[];
}

// Then, impl RustGpuMaterial for our material to tie them together

impl RustGpuMaterial for MyRustGpuMaterial {
    type Vertex = MyVertex;
    type Fragment = MyFragment;
}