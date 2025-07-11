use crate::event::Event;

/// Execution has stopped
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Exstop{
    pub(super) event: Event
}
impl Exstop {
    /// The address at which execution has stopped. This is the last instruction that did not complete.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    #[must_use]
    pub const fn ip(&self) -> u64 {
        unsafe { self.event.0.variant.exstop.ip }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_exstop};
    use std::mem;

    #[test]
    fn test_exstop_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_exstop;
        evt.variant.exstop.ip = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Exstop(e) => {
                assert_eq!(e.ip(), 11);
            }
            _ => unreachable!("oof"),
        }
    }
}
