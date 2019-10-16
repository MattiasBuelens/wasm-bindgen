use js_sys::Promise;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{
    ReadableStream, ReadableStreamDefaultController, ReadableStreamReadResult, UnderlyingSource,
};

#[wasm_bindgen_test]
async fn test_readable_stream_new() {
    let readable = ReadableStream::new().unwrap();
    assert!(readable.is_instance_of::<ReadableStream>());
}

#[wasm_bindgen_test]
async fn test_readable_stream_new_with_underlying_source() {
    let start_called = Rc::new(Cell::new(false));
    let start_cb = {
        let start_called = start_called.clone();
        Closure::once(move || {
            start_called.set(true);
        })
    };

    let mut source = UnderlyingSource::new();
    source.start(Some(start_cb.as_ref().unchecked_ref()));

    let readable = ReadableStream::new_with_underlying_source(&source).unwrap();
    assert!(readable.is_instance_of::<ReadableStream>());

    // constructor must call source.start()
    assert!(start_called.get());
}

#[wasm_bindgen_test]
async fn test_readable_stream_locked() {
    let readable = ReadableStream::new().unwrap();
    assert!(!readable.locked());
}

#[wasm_bindgen_test]
async fn test_readable_stream_cancel() {
    let readable = ReadableStream::new().unwrap();
    let cancel_promise = readable.cancel();
    assert!(cancel_promise.is_instance_of::<Promise>());
}

#[wasm_bindgen_test]
async fn test_readable_stream_get_reader() {
    let readable = ReadableStream::new().unwrap();
    let reader = readable.get_reader().unwrap();
    let closed_promise = reader.closed();
    assert!(closed_promise.is_instance_of::<Promise>());
}

#[wasm_bindgen_test]
async fn test_readable_stream_default_reader_read() {
    let start_cb = Closure::once(move |controller_js: JsValue| {
        let controller = ReadableStreamDefaultController::from(controller_js);
        // enqueue one chunk, then close the stream
        controller.enqueue(&"hello".into()).unwrap();
        controller.close().unwrap();
    });
    let mut source = UnderlyingSource::new();
    source.start(Some(start_cb.as_ref().unchecked_ref()));

    let readable = ReadableStream::new_with_underlying_source(&source).unwrap();
    let reader = readable.get_reader().unwrap();

    // read the chunk
    let read_promise = reader.read();
    let read_result = JsFuture::from(read_promise).await.unwrap();
    let read_result = ReadableStreamReadResult::from(read_result);
    assert!(!read_result.done());
    assert_eq!(read_result.value().as_string().unwrap(), "hello");

    // read the end of the stream
    let read_promise = reader.read();
    let read_result = JsFuture::from(read_promise).await.unwrap();
    let read_result = ReadableStreamReadResult::from(read_result);
    assert!(read_result.done());
    assert!(read_result.value().is_undefined());
}

#[wasm_bindgen_test]
async fn test_readable_stream_tee() {
    let readable = ReadableStream::new().unwrap();
    let tee_result = readable.tee().unwrap();
    assert_eq!(tee_result.length(), 2);
    let tee_first = ReadableStream::from(tee_result.get(0));
    let tee_second = ReadableStream::from(tee_result.get(1));
    assert!(tee_first.is_instance_of::<ReadableStream>());
    assert!(tee_second.is_instance_of::<ReadableStream>());
}
