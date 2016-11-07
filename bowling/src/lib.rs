use std::collections::HashMap;

const FRAMES: usize = 10;

#[derive(Clone)]
struct Frame {
    n: usize,
    points: Vec<u8>,
    bonus: Vec<u8>,
    is_spare: bool,
    is_strike: bool,
}

impl Frame {
    pub fn new(n: usize) -> Self {
        Frame {
            n: n,
            points: Vec::new(),
            bonus: Vec::new(),
            is_spare: false,
            is_strike: false,
        }
    }

    pub fn is_completed(&self) -> bool {
        let max_attempts: u8 = if self.is_strike { 1 } else { 2 };
        max_attempts == self.points.len() as u8
    }

    pub fn is_spare(&self) -> bool {
        self.is_spare
    }

    pub fn is_strike(&self) -> bool {
        self.is_strike
    }

    pub fn number(&self) -> usize {
        self.n as usize
    }

    fn roll_score(&self) -> u8 {
        self.points.iter().sum()
    }

    fn bonus_score(&self) -> u8 {
        self.bonus.iter().sum()
    }

    pub fn score(&self) -> u8 {
        self.roll_score() + self.bonus_score()
    }

    pub fn roll(&mut self, p: u8) -> Result<(), ()> {
        if p > 10 {
            return Err(());
        }

        self.points.push(p);
        if 10 == self.score() {
            match self.points.len() {
                1 => self.is_strike = true,
                2 => self.is_spare = true,
                _ => {}
            }
        };

        if self.roll_score() > 10 {
            return Err(());
        }

        Ok(())
    }

    pub fn bonus_roll(&mut self, p: u8) -> Result<(), ()> {
        self.bonus.push(p);
        if self.is_last() {
            if self.bonus[0] != 10 && self.bonus_score() > 10 {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn is_last(&self) -> bool {
        FRAMES == self.n as usize
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    bonus: HashMap<usize, u8>,
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut frames = Vec::with_capacity(FRAMES);
        frames.push(Frame::new(1));
        BowlingGame {
            frames: frames,
            bonus: HashMap::new(),
        }
    }

    pub fn roll(&mut self, point: u8) -> Result<(), ()> {
        if self.is_done() {
            return Err(());
        }

        let last_frame = self.frames.pop().unwrap();
        let mut current_frame = if last_frame.is_completed() {
            self.frames.push(last_frame.clone());
            Frame::new(last_frame.number() + 1)
        } else {
            last_frame
        };

        try!(current_frame.roll(point));

        for (for_frame, remaining_bonus) in self.bonus.iter_mut() {
            if *remaining_bonus > 0 {
                try!(self.frames[for_frame - 1].bonus_roll(point));
                *remaining_bonus -= 1;
            }
        }

        if self.frames.len() < 10 {
            if current_frame.is_strike() {
                self.bonus.insert(current_frame.number(), 2);
            } else if current_frame.is_spare() {
                self.bonus.insert(current_frame.number(), 1);
            }

            self.frames.push(current_frame);
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u16, ()> {
        if !self.is_done() {
            return Err(());
        }

        Ok(self.frames.iter().map(|f| f.score() as u16).sum())
    }

    fn is_done(&self) -> bool {
        FRAMES == self.frames.len() && self.frames.last().unwrap().is_completed() &&
        self.bonus.values().all(|&remaining_bonus| remaining_bonus == 0)
    }
}
