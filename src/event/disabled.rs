use crate::event::Event;

/// Tracing has been disabled
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Disabled{
    pub(super) event: Event
}
impl Disabled {
    /// The destination of the first branch inside a
    /// filtered area.
    ///
    /// This field is not valid if \@ip_suppressed is set.
    #[must_use]
    pub fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.disabled.ip }
    }
}

/// Tracing has been disabled asynchronously
#[derive(Clone, Copy, Debug)]
pub struct AsyncDisabled{
    pub(super) event: Event
}
impl AsyncDisabled {
    /// The source address of the asynchronous branch that disabled tracing
    #[must_use]
    pub const fn at(&self) -> u64 {
        unsafe { self.event.0.variant.async_disabled.at }
    }

    /// The destination of the first branch inside a filtered area.
    /// This field is not valid if @ip_suppressed is set.
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.async_disabled.ip }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_async_disabled, pt_event_type_ptev_disabled};
    use std::mem;

    #[test]
    fn test_disabled_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_disabled;
        evt.variant.disabled.ip = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Disabled(e) => {
                assert_eq!(e.ip(), 11);
            }
            _ => unreachable!("oof"),
        }
    }

    #[test]
    fn test_async_disabled_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_disabled;
        evt.variant.async_disabled.at = 1;
        evt.variant.async_disabled.ip = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::AsnycDisabled(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.at(), 1);
            }
            _ => unreachable!("oof"),
        }
    }
}
