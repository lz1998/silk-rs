#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

pub mod sdk {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    use crate::sdk;

    #[test]
    fn it_works() {
        unsafe {
            a();
        }
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    unsafe fn a() {
        let mut a: std::os::raw::c_int = 1;
        let raw_mut = &mut a as *mut i32;
        sdk::SKP_Silk_SDK_Get_Decoder_Size(raw_mut);
        println!("{}", a);
    }
}
