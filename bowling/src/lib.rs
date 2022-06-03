use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Tabulation {
    OpenFrame,
    Spare,
    Strike,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Clone)]
struct Frame {
    throws: usize,
    first_throw_points: usize,
    second_throw_points: usize,
    tabulation: Tabulation,
}

impl Frame {
    fn new() -> Frame {
        Frame {
            throws: 0,
            first_throw_points: 0,
            second_throw_points: 0,
            tabulation: Tabulation::OpenFrame,
        }
    }
}

pub struct BowlingGame {
    atual_frame: usize,
    throw_frame: u8,
    frame: Frame,
    frames: HashMap<usize, Frame>,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            atual_frame: 1,
            throw_frame: 0,
            frame: Frame::new(),
            frames: HashMap::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.throw_frame > 1
            || self.frame.first_throw_points + self.frame.second_throw_points >= 10
        {
            self.frame = Frame::new();
            self.atual_frame += 1;
            self.throw_frame = 0;
        }
        self.throw_frame += 1;
        self.frame.throws += 1;
        match self.frame.throws {
            1 => self.frame.first_throw_points = pins as usize,
            2 => self.frame.second_throw_points = pins as usize,
            _ => {}
        }
        if self.frame.first_throw_points + self.frame.second_throw_points > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.frames.insert(self.atual_frame, self.frame.clone());
        self.update_tabluation();
        match self.check_endgame() {
            Ok(()) => return Ok(()),
            Err(error) => {
                return Err(error);
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.atual_frame > 9 {
            let mut score = 0;
            for (key, value) in self.frames.iter() {
                let mut points = value.first_throw_points + value.second_throw_points;
                if value.tabulation == Tabulation::Spare && *key <= 10 {
                    match self.extract_points_from_spare(&key) {
                        Some(value) => points += value,
                        None => return None,
                    }
                }
                if value.tabulation == Tabulation::Strike && *key <= 10 {
                    match self.extract_points_from_strike(&key) {
                        Some(value) => points += value,
                        None => return None,
                    }
                }
                if *key <= 10 {
                    score += points;
                }
            }
            return Some(score as u16);
        }
        None
    }

    fn update_tabluation(&mut self) {
        for (_key, value) in self.frames.iter_mut() {
            let points = value.first_throw_points + value.second_throw_points;
            if value.throws == 2 && points == 10 {
                value.tabulation = Tabulation::Spare;
            }
            if value.throws == 1 && points == 10 {
                value.tabulation = Tabulation::Strike;
            }
        }
    }

    fn check_endgame(&self) -> Result<(), Error> {
        if self.atual_frame > 10
            && self.frames.get(&10).unwrap().tabulation == Tabulation::OpenFrame
        {
            return Err(Error::GameComplete);
        }

        if self.frames.keys().len() == 11
            && self.frames.get(&10).unwrap().tabulation == Tabulation::Spare
            && self.frames.get(&11).unwrap().second_throw_points > 0
        {
            return Err(Error::GameComplete);
        }

        if self.atual_frame == 12
            && self.frames.get(&10).unwrap().tabulation == Tabulation::Strike
            && self.frames.get(&11).unwrap().tabulation != Tabulation::Strike
            && self.frames.get(&12).unwrap().first_throw_points > 0
        {
            return Err(Error::GameComplete);
        }

        Ok(())
    }

    fn extract_points_from_strike(&self, key: &usize) -> Option<usize> {
        let mut points = 0;
        match self.frames.get(&(key + 1)) {
            Some(frame) => {
                if frame.tabulation == Tabulation::Strike {
                    match self.frames.get(&(*key + 2)) {
                        Some(next_frame) => {
                            points += frame.first_throw_points + next_frame.first_throw_points;
                        }
                        None => return None,
                    }
                } else {
                    points += frame.first_throw_points + frame.second_throw_points;
                }
            }
            None => return None,
        }
        Some(points)
    }

    fn extract_points_from_spare(&self, key: &usize) -> Option<usize> {
        let mut points = 0;
        match self.frames.get(&(*key + 1)) {
            Some(frame) => {
                points += frame.first_throw_points;
            }
            None => return None,
        }
        Some(points)
    }
}
