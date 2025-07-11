use crate::event::Event;

/// A core:bus ratio event
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Cbr{
    pub(super) event: Event
}
impl Cbr {
    /// The core:bus ratio.
    #[must_use]
    pub fn ratio(&self) -> u16 {
        unsafe { self.event.0.variant.cbr.ratio }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_cbr};
    use std::mem;

    #[test]
    fn test_cbr_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_cbr;
        evt.variant.cbr.ratio = 18;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Cbr(e) => {
                assert_eq!(e.ratio(), 18);
            }
            _ => unreachable!("oof"),
        }
    }
}
