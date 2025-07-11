use libipt_sys::{
    pt_event, pt_event_type_ptev_async_branch, pt_event_type_ptev_async_disabled,
    pt_event_type_ptev_async_paging, pt_event_type_ptev_async_vmcs, pt_event_type_ptev_cbr,
    pt_event_type_ptev_disabled, pt_event_type_ptev_enabled, pt_event_type_ptev_exec_mode,
    pt_event_type_ptev_exstop, pt_event_type_ptev_mnt, pt_event_type_ptev_mwait,
    pt_event_type_ptev_overflow, pt_event_type_ptev_paging, pt_event_type_ptev_ptwrite,
    pt_event_type_ptev_pwre, pt_event_type_ptev_pwrx, pt_event_type_ptev_stop,
    pt_event_type_ptev_tick, pt_event_type_ptev_tsx, pt_event_type_ptev_vmcs,
};

mod enabled;
pub use enabled::*;
mod disabled;
pub use disabled::*;
mod branch;
pub use branch::*;
mod paging;
pub use paging::*;
mod overflow;
pub use overflow::*;
mod exec_mode;
pub use exec_mode::*;
mod tsx;
pub use tsx::*;
mod vmcs;
pub use vmcs::*;
mod exstop;
pub use exstop::*;
mod mwait;
pub use mwait::*;
mod pwre;
pub use pwre::*;
mod pwrx;
pub use pwrx::*;
mod ptwrite;
pub use ptwrite::*;
mod tick;
pub use tick::*;
mod mnt;
pub use mnt::*;
mod cbr;
pub use cbr::*;

mod qry;
pub use qry::*;

#[derive(Debug, Clone)]
pub enum EventType {
    Enabled(Enabled),
    Disabled(Disabled),
    AsnycDisabled(AsyncDisabled),
    AsyncBranch(AsyncBranch),
    Paging(Paging),
    AsyncPaging(AsyncPaging),
    Overflow(Overflow),
    ExecMode(ExecMode),
    Tsx(Tsx),
    Vmcs(Vmcs),
    AsyncVmcs(AsyncVmcs),
    Exstop(Exstop),
    Mwait(Mwait),
    Pwre(Pwre),
    Pwrx(Pwrx),
    Ptwrite(Ptwrite),
    Tick(Tick),
    Mnt(Mnt),
    Cbr(Cbr),
    Stop(Stop),
}

impl From<Event> for EventType {
    #[allow(non_upper_case_globals)]
    fn from(event: Event) -> EventType {
        match event.0.type_ {
            pt_event_type_ptev_async_branch => EventType::AsyncBranch(AsyncBranch{event}),
            pt_event_type_ptev_async_disabled => EventType::AsnycDisabled(AsyncDisabled{event}),
            pt_event_type_ptev_async_paging => EventType::AsyncPaging(AsyncPaging{event}),
            pt_event_type_ptev_async_vmcs => EventType::AsyncVmcs(AsyncVmcs{event}),
            pt_event_type_ptev_cbr => EventType::Cbr(Cbr{event}),
            pt_event_type_ptev_disabled => EventType::Disabled(Disabled{event}),
            pt_event_type_ptev_enabled => EventType::Enabled(Enabled{event}),
            pt_event_type_ptev_exec_mode => EventType::ExecMode(ExecMode{event}),
            pt_event_type_ptev_exstop => EventType::Exstop(Exstop{event}),
            pt_event_type_ptev_mnt => EventType::Mnt(Mnt{event}),
            pt_event_type_ptev_mwait => EventType::Mwait(Mwait{event}),
            pt_event_type_ptev_overflow => EventType::Overflow(Overflow{event}),
            pt_event_type_ptev_paging => EventType::Paging(Paging{event}),
            pt_event_type_ptev_ptwrite => EventType::Ptwrite(Ptwrite{event}),
            pt_event_type_ptev_pwre => EventType::Pwre(Pwre{event}),
            pt_event_type_ptev_pwrx => EventType::Pwrx(Pwrx{event}),
            pt_event_type_ptev_tick => EventType::Tick(Tick{event}),
            pt_event_type_ptev_tsx => EventType::Tsx(Tsx{event}),
            pt_event_type_ptev_vmcs => EventType::Vmcs(Vmcs{event}),
            pt_event_type_ptev_stop => EventType::Stop(Stop{event}),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Event(pub(crate) pt_event);
impl Event {
    /// A flag indicating that the event IP has been suppressed.
    #[must_use]
    pub fn ip_suppressed(&self) -> bool {
        self.0.ip_suppressed() > 0
    }

    /// A flag indicating that the event is for status update.
    #[must_use]
    pub fn status_update(&self) -> bool {
        self.0.status_update() > 0
    }

    /// A flag indicating that the event has timing information.
    #[must_use]
    pub fn has_tsc(&self) -> bool {
        self.0.has_tsc() > 0
    }

    /// The time stamp count of the event.
    /// This field is only valid if @has_tsc is set.
    #[must_use]
    pub const fn tsc(&self) -> u64 {
        self.0.tsc
    }

    /// The number of lost mtc packets.
    ///
    /// This gives an idea about the quality of the \@tsc.
    /// The more packets were dropped, the less precise timing is.
    #[must_use]
    pub const fn lost_mtc(&self) -> u32 {
        self.0.lost_mtc
    }

    /// The number of lost cyc packets.
    ///
    /// This gives an idea about the quality of the \@tsc.
    /// The more packets were dropped, the less precise timing is.
    #[must_use]
    pub const fn lost_cyc(&self) -> u32 {
        self.0.lost_cyc
    }

    /// Event specific data.
    #[must_use]
    pub fn event_type(self) -> EventType {
        self.into()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Stop{
    pub(super) event: Event
}

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::pt_event_type_ptev_stop;
    use std::mem;

    #[test]
    fn test_create_event() {
        let evt = pt_event {
            type_: pt_event_type_ptev_stop,
            tsc: 1,
            lost_mtc: 2,
            lost_cyc: 3,
            _bitfield_1: pt_event::new_bitfield_1(1, 0, 1),
            variant: unsafe { mem::zeroed() },
            reserved: [0; 2],
            _bitfield_align_1: [],
        };

        let evt = Event(evt);
        assert!(evt.ip_suppressed());
        assert!(!evt.status_update());
        assert!(evt.has_tsc());

        assert_eq!(evt.tsc(), 1);
        assert_eq!(evt.lost_mtc(), 2);
        assert_eq!(evt.lost_cyc(), 3);

        assert!(matches!(EventType::from(evt), EventType::Stop(Stop{event: evt})));
    }
}
