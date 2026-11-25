//! Dev probe: list ggml backend devices after LlamaBackend::init (US-030 experiment).
//!
//! ```text
//! cargo run -p uie-loader-gguf --example list_ggml_devices
//! ```

fn main() {
    let backend = llama_cpp_2::llama_backend::LlamaBackend::init().expect("llama backend init");
    println!("supports_gpu_offload={}", backend.supports_gpu_offload());
    let devices = llama_cpp_2::list_llama_ggml_backend_devices();
    for d in &devices {
        println!(
            "idx={} name={} backend={} type={:?} mem_total={} mem_free={}",
            d.index, d.name, d.backend, d.device_type, d.memory_total, d.memory_free
        );
    }
    println!("device_count={}", devices.len());
}
