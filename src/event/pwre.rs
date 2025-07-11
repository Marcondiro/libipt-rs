use crate::event::Event;

/// A power state was entered
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Pwre{
    pub(super) event: Event
}
impl Pwre {
    /// The resolved thread C-state.
    #[must_use]
    pub const fn state(&self) -> u8 {
        unsafe { self.event.0.variant.pwre.state }
    }

    /// The resolved thread sub C-state
    #[must_use]
    pub const fn sub_state(&self) -> u8 {
        unsafe { self.event.0.variant.pwre.sub_state }
    }

    /// A flag indicating whether the C-state entry was
    /// initiated by h/w.
    #[must_use]
    pub fn hw(&self) -> bool {
        (unsafe { self.event.0.variant.pwre.hw() }) > 0
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_pwre};
    use std::mem;

    #[test]
    fn test_pwre_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_pwre;
        evt.variant.pwre.state = 11;
        evt.variant.pwre.sub_state = 22;
        unsafe {
            evt.variant.pwre.set_hw(1);
        }

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Pwre(e) => {
                assert_eq!(e.state(), 11);
                assert_eq!(e.sub_state(), 22);
                assert!(e.hw())
            }
            _ => unreachable!("oof"),
        }
    }
}
