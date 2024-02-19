// #![no_std]

// extern crate alloc;

use core::{arch::wasm32, cell::OnceCell};

use anyhow::{anyhow, Result};
use rquickjs;

// #[cfg(target_family = "wasm")]
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// #[global_allocator]
// static ALLOCATOR: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

static mut CONTEXT: OnceCell<rquickjs::Context> = OnceCell::new();
static mut DID_INIT: OnceCell<bool> = OnceCell::new();

#[cfg(target_family = "wasm")]
#[export_name = "lodash"]
#[no_mangle]
pub extern "C" fn lodash() -> i32 {
    use core::panic;

    if unsafe { DID_INIT.get() }.is_none() {
        match init() {
            Err(err) => panic!("failed to initialize: {}", err),
            _ => {}
        }
    }

    match run_lodash() {
        Ok(v) => v,
        Err(err) => panic!("Error running example {}", err),
    }
}

#[export_name = "wizer.initialize"]
pub extern "C" fn _init() {
    match init() {
        Ok(_) => {
            unsafe { DID_INIT.set(true).unwrap() };
        }
        Err(err) => panic!("failed to initialize: {}", err),
    }
}

fn init() -> Result<()> {
    let runtime =
        rquickjs::Runtime::new().map_err(|e| anyhow!("Failed to create runtime {}", e))?;
    let ctx = rquickjs::Context::custom::<(
        rquickjs::context::intrinsic::Base,
        rquickjs::context::intrinsic::Eval,
    )>(&runtime)
    .map_err(|e| anyhow!("Failed to create context {}", e))?;

    ctx.with(|ctx| {
        match ctx.eval::<rquickjs::Value, &str>(include_str!("../../../dist/index.js")) {
            Ok(_) => (),
            Err(err) => panic!("Failed to evaluate javascript: {}", err),
        }
    });

    unsafe {
        CONTEXT
            .set(ctx)
            .map_err(|_| anyhow!("Context already bound"))?;
    };

    Ok(())
}

fn run_lodash() -> Result<i32> {
    match unsafe { CONTEXT.get() } {
        Some(ctx) => ctx.with(|ctx| -> Result<i32> {
            ctx.globals()
                .get::<_, i32>("major")
                .map_err(|err| anyhow!("Failed to get global 'major': {}", err))
        }),
        None => wasm32::unreachable(),
    }
}
