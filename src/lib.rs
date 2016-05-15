#[macro_use] mod macros;
#[cfg(test)] mod test;

pub struct Stepper {
    start: i64,
    end: i64,
    step: i64,
    down: bool,
}

impl Stepper {
    pub fn new(start: i64, end: i64, step: i64) -> Self {
        if end >= start {
            Stepper { start: start, end: end, step: step, down: false }
        } else {
            Stepper { start: -start, end: -end, step: -step, down: true }
        }
    }
}

impl Iterator for Stepper {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.start >= self.end {
            return None;
        }

        let next = self.start;
        self.start += self.step;

        if self.down {
            Some(-next)
        } else {
            Some(next)
        }
    }
}
