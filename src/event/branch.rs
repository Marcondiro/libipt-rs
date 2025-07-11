use crate::event::Event;
use std::ops::Deref;

/// An asynchronous branch, e.g. interrupt
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct AsyncBranch{
    pub(super) event: Event
}
impl AsyncBranch {
    /// The branch source address
    #[must_use]
    pub fn from(&self) -> u64 {
        unsafe { self.event.0.variant.async_branch.from }
    }

    /// The branch destination address.
    /// This field is not valid if @ip_suppressed is set.
    #[must_use]
    pub fn to(&self) -> u64 {
        unsafe { self.event.0.variant.async_branch.to }
    }
}

impl Deref for AsyncBranch {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_async_branch};
    use std::mem;

    #[test]
    fn test_branch_async_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_branch;
        evt.variant.async_branch.from = 1;
        evt.variant.async_branch.to = 2;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::AsyncBranch(e) => {
                assert_eq!(e.from(), 1);
                assert_eq!(e.to(), 2);
            }
            _ => unreachable!("oof"),
        }
    }
}
