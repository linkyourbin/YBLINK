#[repr(C)]
///Register block
pub struct RegisterBlock {
    chn: (),
}
impl RegisterBlock {
    ///0x00..0x100 - no description available
    #[inline(always)]
    pub const fn chn(&self, n: usize) -> &Chn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x100 - no description available
    #[inline(always)]
    pub fn chn_iter(&self) -> impl Iterator<Item = &Chn> {
        (0..8).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() })
    }
}
///no description available
pub use self::chn::Chn;
///Cluster
///no description available
pub mod chn;
