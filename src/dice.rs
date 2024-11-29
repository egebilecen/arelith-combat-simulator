use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Dice {
    pub rolls: i32,
    pub faces: i32,
}

impl Dice {
    pub fn new(rolls: i32, faces: i32) -> Self {
        Dice { rolls, faces }
    }

    pub fn roll(&self) -> i32 {
        let mut res = 0;

        if self.rolls == 0 || self.faces == 0 {
            return 0;
        }

        for _ in 0..self.rolls {
            res += thread_rng().gen_range(1..=self.faces);
        }

        res
    }

    pub fn roll_m(&self, count: i32) -> i32 {
        if count < 1 {
            return 0;
        }

        let mut res = 0;

        for _ in 0..count {
            res += self.roll();
        }

        res
    }
}

impl ToString for Dice {
    fn to_string(&self) -> String {
        format!("{}d{}", self.rolls, self.faces)
    }
}

impl Default for Dice {
    fn default() -> Self {
        Dice::from(0)
    }
}

impl From<i32> for Dice {
    fn from(num: i32) -> Self {
        Dice::new(num, 1)
    }
}

impl From<&str> for Dice {
    fn from(dice: &str) -> Self {
        // TODO: Add support for "d20" instead of "1d20".
        let dice_split: Vec<&str> = dice.split("d").collect();

        if dice_split.len() != 2 {
            return Dice::new(0, 0);
        }

        let rolls = dice_split[0].parse::<i32>().unwrap_or(0);
        let faces = dice_split[1].parse::<i32>().unwrap_or(0);

        Dice::new(rolls, faces)
    }
}

impl From<String> for Dice {
    fn from(dice: String) -> Self {
        Dice::from(dice.as_str())
    }
}

#[cfg(test)]
mod test {
    use crate::dice::Dice;

    #[test]
    fn dice() {
        let dice = Dice::new(0, 6);
        assert_eq!(dice.rolls, 0);
        assert_eq!(dice.faces, 6);
        assert_eq!(dice.roll(), 0);
        assert_eq!(dice.to_string(), "0d6");

        let dice = Dice::new(3, 0);
        assert_eq!(dice.rolls, 3);
        assert_eq!(dice.faces, 0);
        assert_eq!(dice.roll(), 0);
        assert_eq!(dice.to_string(), "3d0");

        let dice = Dice::from("1d6");
        assert_eq!(dice.rolls, 1);
        assert_eq!(dice.faces, 6);
        assert_eq!(dice.to_string(), "1d6");

        let dice = Dice::from("2d4");
        assert_eq!(dice.rolls, 2);
        assert_eq!(dice.faces, 4);
        assert_eq!(dice.to_string(), "2d4");

        let dice = Dice::from("4d1");
        assert_eq!(dice.rolls, 4);
        assert_eq!(dice.faces, 1);
        assert_eq!(dice.roll(), 4);
        assert_eq!(dice.to_string(), "4d1");

        let dice = Dice::from("0d3");
        assert_eq!(dice.rolls, 0);
        assert_eq!(dice.faces, 3);
        assert_eq!(dice.roll(), 0);
        assert_eq!(dice.to_string(), "0d3");

        let dice = Dice::from("2d0");
        assert_eq!(dice.rolls, 2);
        assert_eq!(dice.faces, 0);
        assert_eq!(dice.roll(), 0);
        assert_eq!(dice.to_string(), "2d0");

        let dice = Dice::from(6);
        assert_eq!(dice.rolls, 6);
        assert_eq!(dice.faces, 1);
        assert_eq!(dice.roll(), 6);
        assert_eq!(dice.to_string(), "6d1");

        let dice = Dice::from(0);
        assert_eq!(dice.rolls, 0);
        assert_eq!(dice.faces, 1);
        assert_eq!(dice.roll(), 0);
        assert_eq!(dice.to_string(), "0d1");
    }
}
