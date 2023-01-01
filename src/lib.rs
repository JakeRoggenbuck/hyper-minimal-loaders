#[derive(Default)]
pub struct Count {
    pub percent: i8,
}

pub trait Counter {
    fn tick(&mut self);
    fn check(&mut self, op: &dyn Fn() -> bool) -> bool;
    fn done(&mut self);
    fn render(&self) -> String;
    fn show(&self);
    fn finished(&self) -> bool;
}

impl Counter for Count {
    fn tick(&mut self) {
        if self.percent < 100 {
            self.percent += 1;
        }
    }

    fn check(&mut self, op: &dyn Fn() -> bool) -> bool {
        if op() {
            self.percent += 1;
        } else {
            return true;
        }

        return false;
    }

    fn done(&mut self) {
        self.percent = 100;
    }

    fn render(&self) -> String {
        format!("\x1b[1A\x1b[2K{}%\n", self.percent)
    }

    fn show(&self) {
        print!("{}", self.render());
    }

    fn finished(&self) -> bool {
        self.percent == 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_test() {
        let mut count = Count::default();
        for i in 0..100 {
            assert_eq!(count.render(), format!("\u{1b}[1A\u{1b}[2K{}%\n", i));
            count.tick();
        }
    }

    #[test]
    fn done_test() {
        let mut count = Count::default();
        for _ in 0..10 {
            count.done();
            assert_eq!(count.render(), "\u{1b}[1A\u{1b}[2K100%\n");
        }
    }

    #[test]
    fn op_test() {
        let mut count = Count::default();
        let mut expected_percent = 0;
        for i in 0..100 {
            assert_eq!(
                count.render(),
                format!("\u{1b}[1A\u{1b}[2K{}%\n", expected_percent)
            );

            count.check(&|| -> bool {
                if i % 10 == 0 {
                    return true;
                }

                false
            });

            if i % 10 == 0 {
                expected_percent += 1;
            }
        }
    }
}
