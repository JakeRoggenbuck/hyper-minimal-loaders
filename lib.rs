struct Percent {
    percent: i8,
}

trait Counter {
    fn tick(&mut self);
    fn check(&mut self, op: &dyn Fn() -> bool);
    fn done(&mut self);
    fn render(&self) -> String;
    fn show(&self);
}

impl Counter for Percent {
    fn tick(&mut self) {
        if self.percent < 100 {
            self.percent += 1;
        }
    }

    fn check(&mut self, op: &dyn Fn() -> bool) {
        if op() {
            self.percent += 1;
        }
    }

    fn done(&mut self) {
        self.percent = 100;
    }

    fn render(&self) -> String {
        format!("\x1b[1A\x1b[2K{}%", self.percent)
    }

    fn show(&self) {
        print!("{}", self.render());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_test() {
        let mut count = Percent { percent: 0 };
        for i in 0..100 {
            assert_eq!(count.render(), format!("\u{1b}[1A\u{1b}[2K{}%", i));
            count.tick();
        }
    }

    #[test]
    fn done_test() {
        let mut count = Percent { percent: 0 };
        for _ in 0..10 {
            count.done();
            assert_eq!(count.render(), "\u{1b}[1A\u{1b}[2K100%");
        }
    }
}
