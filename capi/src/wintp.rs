use dsp::wintp::WaveInterpolator;
use dsp::processor::Processor;

#[no_mangle]
pub extern "C" fn wintp_create() -> *mut WaveInterpolator {
    Box::into_raw(Box::new(WaveInterpolator::new()))
}

#[no_mangle]
pub unsafe extern "C" fn wintp_destroy( wave_interpolator: *mut WaveInterpolator) {
    assert!(!wave_interpolator.is_null());
    Box::from_raw(wave_interpolator);
}

#[no_mangle]
pub unsafe extern "C" fn wintp_process( wave_interpolator: &mut WaveInterpolator, buffer : *mut f32, block_size : usize) {
    wave_interpolator.process(std::slice::from_raw_parts_mut(buffer, block_size));
}

#[no_mangle]
pub unsafe extern "C" fn wintp_set( wave_interpolator: &mut WaveInterpolator, x :f32, y:f32) {
    wave_interpolator.set(x,y);
}
