#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    throws: Vec<u16>,
    current_throw: usize,
    current_frame: usize,
    pins: u16,
    bonus_rolls: u8,
    complete: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            throws: Vec::new(),
            current_throw: 0,
            current_frame: 0,
            pins: 10,
            bonus_rolls: 0,
            complete: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.complete {
            return Result::Err(Error::GameComplete);
        }
        if pins > self.pins {
            return Result::Err(Error::NotEnoughPinsLeft);
        }
        self.pins -= pins;
        if self.pins > 0 {
            if self.current_throw == 0 {
                self.current_throw = 1;
            } else {
                self.current_throw = 0;
                self.current_frame += 1;
                self.pins = 10;
            }
        } else if self.current_throw == 0 {
            // strike
            self.current_frame += 1;
            self.pins = 10;
            if self.current_frame == 10 {
                self.bonus_rolls = 2;
            }
        } else {
            // spare
            self.current_throw = 0;
            self.current_frame += 1;
            self.pins = 10;
            if self.current_frame == 10 {
                self.bonus_rolls = 1;
            }
        }
        if self.current_frame >= 10 {
            if self.bonus_rolls == 0 {
                self.complete = true;
            } else {
                self.bonus_rolls -= 1;
            }
        }
        self.throws.push(pins);
        Result::Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.complete {
            return None;
        }
        let mut ret = 0;
        let mut pins = 10;
        let mut current_throw = 0;
        let mut current_frame = 0;
        for (i, &throw) in self.throws.iter().enumerate() {
            pins -= throw;
            if pins > 0 {
                ret += throw;
                if current_throw == 0 {
                    current_throw = 1;
                } else {
                    current_frame += 1;
                    current_throw = 0;
                    pins = 10;
                }
            } else if current_throw == 0 {
                // strike
                ret += throw;
                ret += self.throws[i + 1];
                ret += self.throws[i + 2];
                current_frame += 1;
                pins = 10;
            } else {
                // spare
                ret += throw;
                ret += self.throws[i + 1];
                current_frame += 1;
                current_throw = 0;
                pins = 10;
            }
            if current_frame >= 10 {
                break;
            }
        }
        Some(ret)
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
