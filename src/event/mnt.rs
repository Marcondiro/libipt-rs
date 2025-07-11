use crate::event::Event;

/// A maintenance event.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Mnt{
    pub(super) event: Event
}
impl Mnt {
    /// The raw payload.
    #[must_use]
    pub fn payload(self) -> u64 {
        unsafe { self.event.0.variant.mnt.payload }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_mnt};
    use std::mem;

    #[test]
    fn test_mnt_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_mnt;
        evt.variant.mnt.payload = 17;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Mnt(e) => {
                assert_eq!(e.payload(), 17);
            }
            _ => unreachable!("oof"),
        }
    }
}
