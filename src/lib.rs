use bits::BitArray;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct PointSet {
    members: BitArray,
}

impl PointSet {
    pub fn insert(&mut self, x: i64, y: i64) {
        self.members.set(cantor_pairing(x, y), true);
    }

    pub fn contains(&self, x: i64, y: i64) -> bool {
        let index = cantor_pairing(x, y);
        index < self.members.len() && self.members.is_set(index)
    }

    pub fn len(&self) -> u64 {
        self.members.count_bits_on()
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            members: &self.members | &other.members,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.members
            .iter()
            .enumerate()
            .filter(|(_, t)| *t)
            .map(|(i, _)| cantor_unpairing(i as u64))
    }
}

/// See http://szudzik.com/ElegantPairing.pdf for the formula
fn cantor_pairing(x: i64, y: i64) -> u64 {
    let x = naturalize(x);
    let y = naturalize(y);
    (x.pow(2) + x + 2 * x * y + 3 * y + y.pow(2)) / 2
}

fn cantor_unpairing(z: u64) -> (i64, i64) {
    let w = ((((8 * z + 1) as f64).sqrt() - 1.0) / 2.0).floor() as u64;
    let t = (w.pow(2) + w) / 2;
    let y = z - t;
    let x = w - y;
    (denaturalize(x), denaturalize(y))
}

fn naturalize(n: i64) -> u64 {
    let mut base = (n * 2).abs();
    if n < 0 {
        base -= 1;
    }
    base as u64
}

fn denaturalize(n: u64) -> i64 {
    let rem = n % 2;
    let abs = ((n + rem) / 2) as i64;
    if rem == 1 {
        -abs
    } else {
        abs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naturalize() {
        for (n, expected) in [(0, 0), (-1, 1), (1, 2), (-2, 3), (2, 4), (-3, 5), (3, 6)] {
            assert_eq!(naturalize(n), expected);
            assert_eq!(denaturalize(expected), n);
        }
    }

    #[test]
    fn test_pairing() {
        for ((x, y), expected) in [
            ((0, 0), 0),
            ((0, 1), 5),
            ((0, 0), 0),
            ((-1, 0), 1),
            ((0, -1), 2),
            ((1, 0), 3),
            ((-1, -1), 4),
            ((0, 1), 5),
            ((-2, 0), 6),
            ((1, -1), 7),
            ((-1, 1), 8),
            ((0, -2), 9),
            ((2, 0), 10),
            ((-2, -1), 11),
            ((1, 1), 12),
            ((-1, -2), 13),
            ((0, 2), 14),
            ((-3, 0), 15),
            ((2, -1), 16),
            ((-2, 1), 17),
            ((1, -2), 18),
            ((-1, 2), 19),
            ((0, -3), 20),
        ] {
            assert_eq!(expected, cantor_pairing(x, y));
            assert_eq!(cantor_unpairing(expected), (x, y));
        }
    }

    #[test]
    fn test_set() {
        let x_range = -100..=100;
        let y_range = -200..=200;
        let mut points = vec![];
        for x in x_range.clone() {
            for y in y_range.clone() {
                points.push((x, y));
            }
        }

        println!("Created {} points", points.len());

        let mut point_set = PointSet::default();
        assert_eq!(point_set.len(), 0);
        for (x, y) in points.iter() {
            point_set.insert(*x, *y);
        }

        println!("Inserted points");

        assert_eq!(points.len(), point_set.len() as usize);

        for (x, y) in points.iter() {
            assert!(point_set.contains(*x, *y));
        }

        println!("Checked affirmative points");

        for x in -1000..=1000 {
            for y in -2000..=2000 {
                assert_eq!(
                    point_set.contains(x, y),
                    x_range.contains(&x) && y_range.contains(&y)
                );
            }
        }

        println!("Checked broader points");
    }
}
