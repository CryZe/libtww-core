#[no_mangle]
pub unsafe extern "C" fn sqrtf(x: f32) -> f32 {
    extern "C" {
        #[link_name = "std::sqrtf(f32)"]
        fn sqrtf(x: f32) -> f32;
    }
    sqrtf(x)
}

#[no_mangle]
pub extern "C" fn ceilf(var0: f32) -> f32 {
    libm::ceilf(var0)
}

#[no_mangle]
pub extern "C" fn floorf(var0: f32) -> f32 {
    libm::floorf(var0)
}
