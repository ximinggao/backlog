use std::slice::from_raw_parts;

use bincode::{deserialize, serialize};
use kernel::{ResultPointer, SomeDataStruct};
use wasmtime::{Config, Engine, Linker, Module, Result, Store};
use wasmtime_wasi::{
    WasiCtxBuilder,
    p1::{WasiP1Ctx, add_to_linker_async},
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model_async(true);
    let engine = Engine::new(&config).unwrap();
    let module = Module::from_file(&engine, "../wasm-lib/wasm_lib.wasm").unwrap();

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    add_to_linker_async(&mut linker, |t| t).unwrap();
    let pre = linker.instantiate_pre(&module)?;
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build_p1();

    let mut store = Store::new(&engine, wasi_ctx);
    let instance = pre.instantiate_async(&mut store).await.unwrap();

    let data_struct = SomeDataStruct {
        names: vec!["Alice".to_string(), "Bob".to_string()],
        name: "Charlie".to_string(),
    };
    let serialized = serialize(&data_struct).unwrap();

    let malloc = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "ns_malloc")
        .unwrap();
    let input_data_ptr = malloc
        .call_async(&mut store, (serialized.len() as i32, 0))
        .await
        .unwrap();
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    memory
        .write(&mut store, input_data_ptr as usize, &serialized)
        .unwrap();

    let entry_point = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "entry_point")
        .unwrap();
    let ret = entry_point
        .call_async(&mut store, (input_data_ptr, serialized.len() as i32))
        .await
        .unwrap();

    let mut result_buffer = Vec::with_capacity(size_of::<ResultPointer>());
    for _ in 0..size_of::<ResultPointer>() {
        result_buffer.push(0);
    }
    memory
        .read(&mut store, ret as usize, &mut result_buffer)
        .unwrap();
    let result_struct = unsafe {
        &from_raw_parts::<ResultPointer>(result_buffer.as_ptr() as *const ResultPointer, 1)[0]
    };

    let mut output_buffer = Vec::with_capacity(result_struct.len as usize);
    output_buffer.resize(result_struct.len as usize, 0);
    memory
        .read(&mut store, result_struct.ptr as usize, &mut output_buffer)
        .unwrap();
    let output = deserialize::<SomeDataStruct>(&output_buffer).unwrap();
    println!("Output contract: {:?}", output);

    let free = instance
        .get_typed_func::<(i32, i32, i32), ()>(&mut store, "ns_free")
        .unwrap();
    free.call_async(&mut store, (input_data_ptr, serialized.len() as i32, 0))
        .await
        .unwrap();
    free.call_async(&mut store, (result_struct.ptr, result_struct.len, 0))
        .await
        .unwrap();
    free.call_async(&mut store, (ret, size_of::<ResultPointer>() as i32, 0))
        .await
        .unwrap();

    Ok(())
}
