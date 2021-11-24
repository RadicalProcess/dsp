pub use dsp::hold::Hold;
pub use dsp::processor::Processor;

#[no_mangle]
pub extern "C" fn hold_create() -> *mut Hold {
    Box::into_raw(Box::new(Hold::new()))
}

#[no_mangle]
pub unsafe extern "C" fn hold_destroy( hold: *mut Hold) {
    assert!(!hold.is_null());
    Box::from_raw(hold);
}

#[no_mangle]
pub unsafe extern "C" fn hold_process( hold: &mut Hold, buffer : *mut f32, block_size : usize) {
    hold.process(std::slice::from_raw_parts_mut(buffer, block_size));
}

#[no_mangle]
pub unsafe extern "C" fn hold_set_gate( hold: &mut Hold, gate:f32) {
    hold.set_gate(gate);
}

#[no_mangle]
pub unsafe extern "C" fn hold_set_thresh( hold: &mut Hold, thresh:f32) {
    hold.set_thresh(thresh);
}

#[no_mangle]
pub unsafe extern "C" fn hold_set_boost( hold: &mut Hold, boost:f32) {
    hold.set_boost(boost);
}
