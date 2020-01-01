use std::marker::PhantomData;
use libipt_sys::pt_packet_unknown;

// TODO: expose custom context field
// maybe make this generic?
// now sure what to do here tbh

/// An unknown packet decodable by the optional decoder callback.
/// Packet: unknown
#[derive(Clone, Copy)]
pub struct Unknown<'a> (pt_packet_unknown, PhantomData<&'a ()>);
impl<'a> Unknown<'a> {
    #[inline]
    pub fn packet(self) -> &'a u8 {
        unsafe { &*self.0.packet }
    }

    #[inline]
    pub fn set_packet(&mut self, pck: &'a [u8]) {
        self.0.packet = pck.as_ptr()
    }
}