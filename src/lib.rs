#![no_std]

use core::cell::OnceCell;

use anyhow::{anyhow, Result};
use rquickjs::{context, Context, Runtime};

extern crate alloc;

const SHARED_BUFFER_SIZE_MAX: usize = 1024;

#[export_name = "SHARED_BUFFER"]
pub static mut SHARED_BUFFER: [u8; SHARED_BUFFER_SIZE_MAX] = [0; SHARED_BUFFER_SIZE_MAX];

#[export_name = "SHARED_BUFFER_SIZE"]
pub static SHARED_BUFFER_SIZE: u32 = SHARED_BUFFER_SIZE_MAX as u32;

static mut CONTEXT: OnceCell<Context> = OnceCell::new();

#[export_name = "wizer.initialize"]
pub extern "C" fn _init() {
    match init() {
        Ok(_) => {}
        Err(e) => panic!("Failed to initialize {}", e),
    }
}

fn init<'a>() -> Result<()> {
    let runtime = Runtime::new().map_err(|e| anyhow!("Failed to create runtime {}", e))?;
    let ctx = Context::custom::<(
        context::intrinsic::Base,
        context::intrinsic::BigDecimal,
        context::intrinsic::Date,
        context::intrinsic::Eval,
        context::intrinsic::Json,
        context::intrinsic::MapSet,
        context::intrinsic::Operators,
    )>(&runtime)
    .map_err(|e| anyhow!("Failed to create context {}", e))?;

    unsafe {
        CONTEXT
            .set(ctx)
            .map_err(|_| anyhow!("Context already bound"))?;
    };

    Ok(())
}

// #[no_mangle]
// unsafe extern "C" fn __stdio_write(ptr: *const u8, len: usize) -> usize {
//     0
// }

// #[no_mangle]
// unsafe extern "C" fn __stdout_write(ptr: *const u8, len: usize) -> usize {
//     0
// }

// #[no_mangle]
// unsafe extern "C" fn __stdio_seek(ptr: *const u8, len: usize) -> usize {
//     0
// }

// #[no_mangle]
// unsafe extern "C" fn __stdio_close(ptr: *const u8, len: usize) -> usize {
//     0
// }

// #[no_mangle]
// unsafe extern "C" fn __assert_fail(msg: *const i8, file: *const i8, line: u32, func: *const i8) {
//     panic!(
//         "assertion failed: {}",
//         CStr::from_ptr(msg).to_str().unwrap()
//     );
// }
