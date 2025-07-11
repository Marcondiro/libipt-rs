use crate::event::Event;

/// A PTWRITE event.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Ptwrite{
    pub(super) event: Event
}
impl Ptwrite {
    /// The address of the ptwrite instruction.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    /// In this case, the address is obvious from the disassembly.
    #[must_use]
    pub fn ip(self) -> u64 {
        unsafe { self.event.0.variant.ptwrite.ip }
    }

    /// The size of the below @payload in bytes.
    #[must_use]
    pub fn size(self) -> u8 {
        unsafe { self.event.0.variant.ptwrite.size }
    }

    /// The ptwrite payload.
    #[must_use]
    pub fn payload(self) -> u64 {
        unsafe { self.event.0.variant.ptwrite.payload }
    }
}

#[cfg(test)]
mod test {
    use super::super::EventType;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_ptwrite};
    use std::mem;

    #[test]
    fn test_ptwrite_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_ptwrite;
        let ptwrite = unsafe { &mut evt.variant.ptwrite };
        ptwrite.ip = 11;
        ptwrite.size = 22;
        ptwrite.payload = 33;

        let payload: EventType = Event(evt).into();
        match payload {
            EventType::Ptwrite(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.size(), 22);
                assert_eq!(e.payload(), 33);
            }
            _ => unreachable!("oof"),
        }
    }
}
