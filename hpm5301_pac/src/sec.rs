#[repr(C)]
///Register block
pub struct RegisterBlock {
    secure_state: SecureState,
    secure_state_config: SecureStateConfig,
    violation_config: ViolationConfig,
    escalate_config: EscalateConfig,
    event: Event,
    lifecycle: Lifecycle,
}
impl RegisterBlock {
    ///0x00 - Secure state
    #[inline(always)]
    pub const fn secure_state(&self) -> &SecureState {
        &self.secure_state
    }
    ///0x04 - secure state configuration
    #[inline(always)]
    pub const fn secure_state_config(&self) -> &SecureStateConfig {
        &self.secure_state_config
    }
    ///0x08 - Security violation config
    #[inline(always)]
    pub const fn violation_config(&self) -> &ViolationConfig {
        &self.violation_config
    }
    ///0x0c - Escalate behavior on security event
    #[inline(always)]
    pub const fn escalate_config(&self) -> &EscalateConfig {
        &self.escalate_config
    }
    ///0x10 - Event and escalate status
    #[inline(always)]
    pub const fn event(&self) -> &Event {
        &self.event
    }
    ///0x14 - Lifecycle
    #[inline(always)]
    pub const fn lifecycle(&self) -> &Lifecycle {
        &self.lifecycle
    }
}
/**SECURE_STATE (rw) register accessor: Secure state

You can [`read`](crate::Reg::read) this register and get [`secure_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@secure_state`] module*/
#[doc(alias = "SECURE_STATE")]
pub type SecureState = crate::Reg<secure_state::SecureStateSpec>;
///Secure state
pub mod secure_state;
/**SECURE_STATE_CONFIG (rw) register accessor: secure state configuration

You can [`read`](crate::Reg::read) this register and get [`secure_state_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_state_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@secure_state_config`] module*/
#[doc(alias = "SECURE_STATE_CONFIG")]
pub type SecureStateConfig = crate::Reg<secure_state_config::SecureStateConfigSpec>;
///secure state configuration
pub mod secure_state_config;
/**VIOLATION_CONFIG (rw) register accessor: Security violation config

You can [`read`](crate::Reg::read) this register and get [`violation_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`violation_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@violation_config`] module*/
#[doc(alias = "VIOLATION_CONFIG")]
pub type ViolationConfig = crate::Reg<violation_config::ViolationConfigSpec>;
///Security violation config
pub mod violation_config;
/**ESCALATE_CONFIG (rw) register accessor: Escalate behavior on security event

You can [`read`](crate::Reg::read) this register and get [`escalate_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`escalate_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@escalate_config`] module*/
#[doc(alias = "ESCALATE_CONFIG")]
pub type EscalateConfig = crate::Reg<escalate_config::EscalateConfigSpec>;
///Escalate behavior on security event
pub mod escalate_config;
/**EVENT (rw) register accessor: Event and escalate status

You can [`read`](crate::Reg::read) this register and get [`event::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`event::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@event`] module*/
#[doc(alias = "EVENT")]
pub type Event = crate::Reg<event::EventSpec>;
///Event and escalate status
pub mod event;
/**LIFECYCLE (rw) register accessor: Lifecycle

You can [`read`](crate::Reg::read) this register and get [`lifecycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifecycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lifecycle`] module*/
#[doc(alias = "LIFECYCLE")]
pub type Lifecycle = crate::Reg<lifecycle::LifecycleSpec>;
///Lifecycle
pub mod lifecycle;
