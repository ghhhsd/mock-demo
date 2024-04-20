#[cfg(target_arch = "arm")]
pub use arm::{cache_flush, get_instruction_len, replace_instruction};
#[cfg(not(target_arch = "arm"))]
pub use x86::{cache_flush, get_instruction_len, replace_instruction};

#[cfg(target_arch = "arm")]
mod arm;

#[cfg(not(target_arch = "arm"))]
mod x86;

