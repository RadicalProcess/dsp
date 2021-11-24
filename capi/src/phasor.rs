use dsp::phasor::Phasor;
use dsp::processor::Processor;

#[no_mangle]
pub extern "C" fn phasor_create() -> *mut Phasor {
    Box::into_raw(Box::new(Phasor::new()))
}

#[no_mangle]
pub unsafe extern "C" fn phasor_destroy( phasor: *mut Phasor) {
    assert!(!phasor.is_null());
    Box::from_raw(phasor);
}

#[no_mangle]
pub unsafe extern "C" fn phasor_process( phasor: &mut Phasor, buffer : *mut f32, block_size : usize) {
    phasor.process( std::slice::from_raw_parts_mut(buffer, block_size));
}

#[no_mangle]
pub unsafe extern "C" fn phasor_set_duty_cycle( phasor: &mut Phasor, duty_cycle : f32) {
    phasor.set_duty_cycle(duty_cycle);
}