use dsp::delay::Delay;
use dsp::processor::Processor;

#[no_mangle]
pub extern "C" fn delay_create( delay_time: usize ) -> *mut Delay {
    Box::into_raw(Box::new(Delay::new(delay_time)))
}

#[no_mangle]
pub unsafe extern "C" fn delay_destroy( delay: *mut Delay) {
    assert!(!delay.is_null());
    Box::from_raw(delay);
}

#[no_mangle]
pub unsafe extern "C" fn delay_process( delay: &mut Delay, buffer : *mut f32, block_size : usize) {
    let buf = std::slice::from_raw_parts_mut(buffer, block_size);
    delay.process(buf);
}
