use piston::input::keyboard::Key;

pub struct Input {
  keys: [bool; 16]
}

impl Input {
  pub fn new() -> Input {
    Input { keys: [false; 16] }
  }

  pub fn pressed(&mut self, index: usize) -> bool {
    self.keys[index]
  }

  pub fn press(&mut self, key: Key, state: bool) {
    match key {
      Key::NumPad1  => self.set_key(0x1, state),
      Key::NumPad2  => self.set_key(0x2, state),
      Key::NumPad3  => self.set_key(0x3, state),
      Key::NumPad4  => self.set_key(0xc, state),
      Key::Q        => self.set_key(0x4, state),
      Key::W        => self.set_key(0x5, state),
      Key::E        => self.set_key(0x6, state),
      Key::R        => self.set_key(0xd, state),
      Key::A        => self.set_key(0x7, state),
      Key::S        => self.set_key(0x8, state),
      Key::D        => self.set_key(0x9, state),
      Key::F        => self.set_key(0xe, state),
      Key::Z        => self.set_key(0xa, state),
      Key::X        => self.set_key(0x0, state),
      Key::C        => self.set_key(0xb, state),
      Key::V        => self.set_key(0xf, state),
      _             => ()
    }
  }

  fn set_key(&mut self, index: usize, state: bool) {
    self.keys[index] = state;
  }
}