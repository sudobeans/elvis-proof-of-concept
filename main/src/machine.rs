/// A machine can send and receive messages.
pub trait Machine {
    fn send(&mut self, msg: u8);

    fn recv(&mut self) -> u8;
}
