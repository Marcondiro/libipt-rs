use crate::event::Event;

/// Tracing has been enabled
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Enabled{
    pub(super) event: Event
}
impl Enabled {
    /// The address at which tracing resumes
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.enabled.ip }
    }

    /// A flag indicating that tracing resumes from the IP
    /// at which tracing had been disabled before.
    #[must_use]
    pub fn resumed(&self) -> bool {
        (unsafe { self.event.0.variant.enabled.resumed() }) > 0
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_enabled};
    use std::mem;

    #[test]
    fn test_enabled_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_enabled;
        evt.variant.enabled.ip = 11;
        unsafe {
            evt.variant.enabled.set_resumed(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Enabled(e) => {
                assert_eq!(e.ip(), 11);
                assert!(e.resumed())
            }
            _ => unreachable!("oof"),
        }
    }
}
