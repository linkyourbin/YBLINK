#[repr(C)]
///Register block
pub struct RegisterBlock {
    channel: (),
}
impl RegisterBlock {
    ///0x00..0x38 - no description available
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CHANNELchn0` cluster.</div>
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x38 - no description available
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32 * n).cast() })
    }
    ///0x00..0x1c - no description available
    #[inline(always)]
    pub const fn channelchn0(&self) -> &Channel {
        self.channel(0)
    }
    ///0x20..0x3c - no description available
    #[inline(always)]
    pub const fn channelchn1(&self) -> &Channel {
        self.channel(1)
    }
}
///no description available
pub use self::channel::Channel;
///Cluster
///no description available
pub mod channel;
