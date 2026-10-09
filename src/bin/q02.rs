#[derive(Clone, Debug)]
struct ComplexNumber {
    x: i64,
    y: i64,
}

impl ComplexNumber {
    const fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    const fn mult(&mut self, other: &Self) {
        let new_x = self.x * other.x - self.y * other.y;
        let new_y = self.x * other.y + self.y * other.x;

        self.x = new_x;
        self.y = new_y;
    }

    const fn div(&mut self, other: &Self) {
        self.x /= other.x;
        self.y /= other.y;
    }

    fn good(x: i64, y: i64) -> bool {
        fn in_range(x: i64, y: i64) -> bool {
            (-1_000_000..=1_000_000).contains(&x) && (-1_000_000..=1_000_000).contains(&y)
        }

        let mut res = Self { x: 0, y: 0 };
        for _ in 0..100 {
            res.mult(&res.clone());
            res.div(&Self {
                x: 100_000,
                y: 100_000,
            });
            res.add(&Self { x, y });

            if !in_range(res.x, res.y) {
                return false;
            }
        }

        true
    }
}

impl ToString for ComplexNumber {
    fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }
}

fn part1(_input: &str) -> String {
    let A = ComplexNumber { x: 146, y: 55 };
    let mut result = ComplexNumber { x: 0, y: 0 };

    for _ in 0..3 {
        result.mult(&result.clone());
        result.div(&ComplexNumber { x: 10, y: 10 });
        result.add(&A);
    }

    result.to_string()
}

fn part2(_input: &str) -> String {
    let a = ComplexNumber {
        x: -79_037,
        y: 14_068,
    };

    let mut eng_points = 0;

    for x in (a.x..=(a.x + 1000)).step_by(10) {
        for y in (a.y..=(a.y + 1000)).step_by(10) {
            if ComplexNumber::good(x, y) {
                eng_points += 1;
            }
        }
    }

    eng_points.to_string()
}

fn part3(_input: &str) -> String {
    let a = ComplexNumber {
        x: -79_037,
        y: 14_068,
    };

    let mut eng_points = 0;

    for x in (a.x..=(a.x + 1000)).step_by(1) {
        for y in (a.y..=(a.y + 1000)).step_by(1) {
            if ComplexNumber::good(x, y) {
                eng_points += 1;
            }
        }
    }

    eng_points.to_string()
}

fn main() {
    everybody_codes_2025::run(env!("CARGO_BIN_NAME"), [part1, part2, part3]);
}
