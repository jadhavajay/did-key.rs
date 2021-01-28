use crate::*;
use ffi_support;

#[no_mangle]
pub extern "C" fn generate_new(key_type: DIDKeyType) -> DIDKey {
    c_impl!(key_type)
}
