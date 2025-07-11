use crate::event::Event;

/// A transactional execution state change
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Tsx{
    pub(super) event: Event
}
impl Tsx {
    /// The address at which the event is effective.
    ///
    /// This field is not valid if @ip_suppressed is set.
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.tsx.ip }
    }

    /// A flag indicating speculative execution mode
    #[must_use]
    pub fn speculative(&self) -> bool {
        (unsafe { self.event.0.variant.tsx.speculative() }) > 0
    }

    /// A flag indicating speculative execution aborts
    #[must_use]
    pub fn aborted(&self) -> bool {
        (unsafe { self.event.0.variant.tsx.aborted() }) > 0
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_tsx};
    use std::mem;

    #[test]
    fn test_tsx_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_tsx;
        evt.variant.tsx.ip = 11;
        unsafe {
            evt.variant.tsx.set_speculative(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Tsx(e) => {
                assert_eq!(e.ip(), 11);
                assert!(e.speculative());
                assert!(!e.aborted());
            }
            _ => unreachable!("oof"),
        }
    }
}
