use dsp::additive::Additive;
use dsp::processor::Processor;

#[no_mangle]
pub extern "C" fn additive_create( ) -> *mut Additive {
    Box::into_raw(Box::new(Additive::new()))
}

#[no_mangle]
pub unsafe extern "C" fn additive_destroy( additive: *mut Additive) {
    assert!(!additive.is_null());
    Box::from_raw(additive);
}

#[no_mangle]
pub unsafe extern "C" fn additive_process( additive: &mut Additive, buffer : *mut f32, block_size : usize) {
    additive.process(std::slice::from_raw_parts_mut(buffer, block_size));
}

#[no_mangle]
pub unsafe extern "C" fn additive_harm( additive: &mut Additive, index : usize, amp : f32 ) {
    additive.set_harmonic(index, amp);
}
