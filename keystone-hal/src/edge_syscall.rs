#[repr(C)]
pub enum SyscallReq {
    Write { fd: u64, len: u64 },
}

const STRUCT_LEN: usize = core::mem::size_of::<SyscallReq>();

static_assertions::const_assert!(STRUCT_LEN <= 256);

// note: maybe unsafe since these functions operate on the
// underlying binary representation of the object
impl SyscallReq {
    pub fn write_to(&self, dest: &mut [u8]) {
        let self_as_data =
            unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, STRUCT_LEN) };
        dest[0..STRUCT_LEN].copy_from_slice(self_as_data);
    }

    pub unsafe fn read_from(src: &[u8]) -> SyscallReq {
        let mut result = core::mem::MaybeUninit::<SyscallReq>::uninit();
        let result_as_data =
            core::slice::from_raw_parts_mut(result.as_mut_ptr() as *mut u8, STRUCT_LEN);
        result_as_data.copy_from_slice(&src[0..STRUCT_LEN]);
        result.assume_init()
    }
}
