use crate::event::Event;

/// A synchronous paging event
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Paging{
    pub(super) event: Event
}
impl Paging {
    /// The updated CR3 value.
    /// The lower 5 bit have been zeroed out.
    /// The upper bits have been zeroed out depending on the maximum possible address.
    #[must_use]
    pub fn cr3(&self) -> u64 {
        unsafe { self.event.0.variant.paging.cr3 }
    }

    /// A flag indicating whether the cpu is operating in
    /// vmx non-root (guest) mode.
    #[must_use]
    pub fn non_root(&self) -> bool {
        (unsafe { self.event.0.variant.paging.non_root() }) > 0
    }
}

/// An asynchronous paging event
#[derive(Clone, Copy, Debug)]
pub struct AsyncPaging{
    pub(super) event: Event
}
impl AsyncPaging {
    /// The updated CR3 value.
    ///
    /// The lower 5 bit have been zeroed out.
    /// The upper bits have been zeroed out depending on the
    /// maximum possible address.
    #[must_use]
    pub const fn cr3(&self) -> u64 {
        unsafe { self.event.0.variant.async_paging.cr3 }
    }

    /// A flag indicating whether the cpu is operating in
    /// vmx non-root (guest) mode.
    #[must_use]
    pub fn non_root(&self) -> bool {
        (unsafe { self.event.0.variant.async_paging.non_root() }) > 0
    }

    /// The address at which the event is effective
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.async_paging.ip }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_async_paging, pt_event_type_ptev_paging};
    use std::mem;

    #[test]
    fn test_paging_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_paging;
        evt.variant.paging.cr3 = 11;
        unsafe {
            evt.variant.async_paging.set_non_root(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Paging(e) => {
                assert_eq!(e.cr3(), 11);
                assert!(e.non_root());
            }
            _ => unreachable!("oof"),
        }
    }

    #[test]
    fn test_async_paging_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_paging;

        evt.variant.async_paging.cr3 = 11;
        evt.variant.async_paging.ip = 12;
        unsafe {
            evt.variant.async_paging.set_non_root(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::AsyncPaging(e) => {
                assert_eq!(e.cr3(), 11);
                assert_eq!(e.ip(), 12);
                assert!(e.non_root());
            }
            _ => unreachable!("oof"),
        }
    }
}
