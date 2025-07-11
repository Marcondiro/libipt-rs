use crate::event::Event;

/// A timing event
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Tick{
    pub(super) event: Event
}
impl Tick {
    /// The instruction address near which the tick occured.
    ///
    /// A timestamp can sometimes be attributed directly to
    /// an instruction (e.g. to an indirect branch that
    /// receives CYC + TIP) and sometimes not (e.g. MTC).
    ///
    /// This field is not valid, if @ip_suppressed is set.
    #[must_use]
    pub fn ip(self) -> u64 {
        unsafe { self.event.0.variant.tick.ip }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_tick};
    use std::mem;

    #[test]
    fn test_tick_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_tick;
        evt.variant.tick.ip = 11;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Tick(e) => {
                assert_eq!(e.ip(), 11);
            }
            _ => unreachable!("oof"),
        }
    }
}
