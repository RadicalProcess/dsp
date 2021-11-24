pub use dsp::xfader::XFader;

#[no_mangle]
pub extern "C" fn xfader_create( mix: f32 ) -> *mut XFader {
    Box::into_raw(Box::new(XFader::new(mix)))
}

#[no_mangle]
pub unsafe extern "C" fn xfader_destroy( xfader: *mut XFader) {
    assert!(!xfader.is_null());
    Box::from_raw(xfader);
}

#[no_mangle]
pub unsafe extern "C" fn xfader_set_dry_wet( xfader: &mut XFader, dry_wet: f32) {
    xfader.mix = dry_wet;
}

#[no_mangle]
pub unsafe extern "C" fn xfader_process( xfader: &mut XFader, buffer : *mut f32, reference : *const f32, block_size : usize) {
    let buffer = std::slice::from_raw_parts_mut(buffer, block_size);
    let reference = std::slice::from_raw_parts(reference, block_size);
    xfader.process(buffer, reference);
}
