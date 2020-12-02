#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

mod mstsc;

pub use mstsc::Server;
pub use mstsc::start_rdp;

use napi::{CallContext, Env, JsObject, JsNumber, JsUnknown, Result, Task};

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;


struct AsyncTask(Server);

impl Task for AsyncTask {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    start_rdp(&self.0);
    Ok(2)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

pub fn register_js(exports: &mut JsObject) -> Result<()> {
  exports.create_named_method("startRdpWithoutGui", start_rdp_js)?;
  exports.create_named_method("startRdpWithGui", start_rdp_sync)?;
  Ok(())
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  register_js(&mut exports)?;
  Ok(())
}

#[js_function(1)]
fn start_rdp_js(ctx: CallContext) -> Result<JsObject> {
  let arg0 = ctx.get::<JsUnknown>(0)?;
  let server: Server = ctx.env.from_js_value(arg0)?;
  // let de_serialized: AnObject = ctx.env.from_js_value(arg0)?;
  let task = AsyncTask(server);
  let async_promise = ctx.env.spawn(task)?;
  Ok(async_promise.promise_object())
}

#[js_function(1)]
fn start_rdp_sync(ctx: CallContext) -> Result<JsNumber> {
  let arg0 = ctx.get::<JsUnknown>(0)?;
  let server: Server = ctx.env.from_js_value(arg0)?;
  // let de_serialized: AnObject = ctx.env.from_js_value(arg0)?;
  start_rdp(&server);
  ctx.env.create_uint32(0)
}
