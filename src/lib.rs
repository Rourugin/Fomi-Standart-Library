use std::mem;
use serde::Deserialize;
use serde_json::json;


#[derive(Deserialize)]
struct InputArgs {
    command: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[unsafe(no_mangle)]
pub extern "C" fn fomi_alloc(len: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn fomi_run(ptr: *mut u8, len: i32) -> *mut u8 {
    let input_slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    let input_str = String::from_utf8_lossy(input_slice);

    let args: InputArgs = serde_json::from_str(&input_str).unwrap_or(InputArgs {
        command: "error".to_string(),
        params: json!({"error": "Invalid JSON format"}),
    });

    let result = match args.command.as_str() {
        "get_time" => {
            let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            format!("Current time: {}", now)
        },
        "generate_uuid" => {
            let new_uuid = uuid::Uuid::new_v4().to_string();
            format!("Generated UUID: {}", new_uuid)
        },
        _ => format!("Unknown command: {}", args.command),
    };

    let response_bytes = result.as_bytes();
    let response_len = response_bytes.len() as i32;

    let mut output = Vec::new();
    output.extend_from_slice(&response_len.to_le_bytes());
    output.extend_from_slice(response_bytes);

    let response_ptr = output.as_mut_ptr();
    mem::forget(output);
    response_ptr
}