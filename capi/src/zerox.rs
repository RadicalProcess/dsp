use dsp::zerox::ZeroX;
use dsp::processor::Processor;

#[no_mangle]
pub extern "C" fn zerox_create( timeout: usize ) -> *mut ZeroX {
    Box::into_raw(Box::new(ZeroX::new(timeout)))
}

#[no_mangle]
pub unsafe extern "C" fn zerox_destroy( zerox: *mut ZeroX) {
    assert!(!zerox.is_null());
    Box::from_raw(zerox);
}

#[no_mangle]
pub unsafe extern "C" fn zerox_process( zerox: &mut ZeroX, buffer : *mut f32, block_size : usize) {
    zerox.process(std::slice::from_raw_parts_mut(buffer, block_size));
}

#[no_mangle]
pub unsafe extern "C" fn zerox_set_skip( zerox: &mut ZeroX, skip : usize) {
    zerox.set_skip(skip);
}

#[no_mangle]
pub unsafe extern "C" fn zerox_set_min_length( zerox: &mut ZeroX, min_length : usize) {
    zerox.set_min_length(min_length);
}
