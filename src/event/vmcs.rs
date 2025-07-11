use crate::event::Event;

/// A synchronous vmcs event
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Vmcs{
    pub(super) event: Event
}
impl Vmcs {
    /// The VMCS base address.
    ///
    /// The address is zero-extended with the lower 12 bits all zero
    #[must_use]
    pub fn base(self) -> u64 {
        unsafe { self.event.0.variant.async_vmcs.base }
    }
}

/// An asynchronous vmcs event
#[derive(Clone, Copy, Debug)]
pub struct AsyncVmcs{
    pub(super) event: Event
}
impl AsyncVmcs {
    /// The VMCS base address.
    ///
    /// The address is zero-extended with the lower 12 bits all zero
    #[must_use]
    pub fn base(self) -> u64 {
        unsafe { self.event.0.variant.async_vmcs.base }
    }

    /// The address at which the event is effective.
    #[must_use]
    pub fn ip(self) -> u64 {
        unsafe { self.event.0.variant.async_vmcs.ip }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_async_vmcs, pt_event_type_ptev_vmcs};
    use std::mem;

    #[test]
    fn test_vmcs_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_vmcs;
        evt.variant.vmcs.base = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Vmcs(e) => {
                assert_eq!(e.base(), 11);
            }
            _ => unreachable!("oof"),
        }
    }

    #[test]
    fn test_async_vmcs_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_vmcs;
        evt.variant.async_vmcs.base = 11;
        evt.variant.async_vmcs.ip = 12;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::AsyncVmcs(e) => {
                assert_eq!(e.base(), 11);
                assert_eq!(e.ip(), 12);
            }
            _ => unreachable!("oof"),
        }
    }
}
