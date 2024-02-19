use std::{
    borrow::BorrowMut,
    io::{self, Write},
};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use wasi_common::WasiCtx;
use wasmtime::{Engine, InstancePre, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

static ENGINE_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/quicky_wasm.wasm"));

#[get("/wasmtime")]
async fn handle_wasmtime(
    instance_pre: web::Data<InstancePre<WasiCtx>>,
    engine: web::Data<Engine>,
) -> HttpResponse {
    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);

    let instance = instance_pre.instantiate(store.borrow_mut()).unwrap();

    match instance.get_typed_func::<(), i32>(&mut store, "lodash") {
        Ok(func) => match func.call(store, ()) {
            Ok(n) => HttpResponse::Ok().body(format!("{}", n)),
            Err(err) => {
                HttpResponse::InternalServerError().body(format!("error calling function: {}", err))
            }
        },
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("error extracting typed lodash function: {}", err)),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let server = HttpServer::new(|| {
        // Define the WASI functions globally on the `Config`.
        let engine = Engine::default();
        let mut linker = Linker::<WasiCtx>::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

        let module = Module::from_binary(&engine, ENGINE_BYTES).unwrap();
        let precompiled = linker.instantiate_pre(&module).unwrap();

        let app = App::new()
            .app_data(web::Data::new(engine))
            .app_data(web::Data::new(precompiled))
            .service(hello)
            .service(handle_wasmtime);

        print!("service created");

        let _ = std::io::stdout().flush();

        app
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    print!("server listening");

    std::io::stdout().flush()?;

    server.await
}
