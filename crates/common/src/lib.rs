pub mod result;

pub use holochain_serialized_bytes::prelude::*;
pub use result::*;
pub use serde_bytes;

/// something like usize for wasm
/// wasm has a memory limit of 4GB so offsets and lengths fit in u32
///
/// when rust compiles to the wasm32-unknown-unknown target this means that usize will be u32 in
/// wasm but the host could interpret usize as either u32 or u64.
///
/// we need the host and the guest to have a shared agreement on the size of an offset/length or
/// the host will not be able to directly manipulate the host memory as it needs to
///
/// wasmer itself uses u32 in the WasmPtr abstraction etc.
/// @see https://docs.rs/wasmer-runtime/0.17.0/wasmer_runtime/struct.WasmPtr.html
pub type WasmSize = u32;

/// a WasmSize integer that represents the size of bytes to read/write to memory in direct
/// manipulations
pub type Len = WasmSize;

/// a WasmSize integer that points to a position in wasm linear memory that the host and guest are
/// sharing to communicate across function calls
pub type GuestPtr = WasmSize;
