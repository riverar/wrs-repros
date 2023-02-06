#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::all
)]
#[inline]
pub unsafe fn DoSomething() -> ::windows::core::Result<()> {
    #[link(name = "samplestatic", kind = "static")]
    extern "system" {
        fn DoSomething() -> ::windows::core::HRESULT;
    }
    DoSomething().ok()
}
