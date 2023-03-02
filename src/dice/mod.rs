use anyhow::anyhow;
use rand::{thread_rng, Rng};
use std::{sync::mpsc::channel, thread};

pub fn roll(expression: impl Into<String>) -> Result<i32, anyhow::Error> {
    let expression_string = expression.into();
    // Uses the From<String> impl below
    let expression: Expression = expression_string.into();
    // The '?' will return early if the Result is an Err
    let count = expression.get_count()?;
    let sides = expression.get_sides()?;
    let mut result = 0;
    let mut loops = 0;
    let (tx, rx) = channel();

    loop {
        if loops == count {
            break
        }
        let tx = tx.clone();
        thread::spawn(move || {
            let mut rng = thread_rng();
            let roll = rng.gen_range(1..=sides);
            match tx.send(roll) {
                Ok(_) => {},
                Err(_) => panic!("could not send roll"),
            }
        });
        loops += 1;
    }
    drop(tx);

    while let Ok(n) = rx.recv() {
        result += n;
    }

    Ok(result)
}

struct Expression(String);

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Expression {
    fn get_count(&self) -> Result<i32, anyhow::Error> {
        let (count, _) = self.0.split_once('d').unwrap();
        match count.parse() {
            Ok(c) => Ok(c),
            Err(e) => {
                if count.is_empty() {
                    Ok(1)
                } else {
                    Err(anyhow!("Error: {e}"))
                }
            },
        }
    }

    fn get_sides(&self) -> Result<i32, anyhow::Error> {
        let (_, sides) = self.0.split_once('d').unwrap();
        sides.parse().map_err(|e| anyhow!("Error: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_count_works() {
        let expr = "4d6".to_string();
        let expression: Expression = expr.into();

        let result = expression.get_count().unwrap();

        assert_eq!(result, 4);
    }

    #[test]
    fn get_sides_works() {
        let expr = "4d6".to_string();
        let expression: Expression = expr.into();

        let result = expression.get_sides().unwrap();

        assert_eq!(result, 6);
    }

    #[test]
    fn roll_works() {
        let tests = 1;
        let mut count = 0;
        let mut rng = thread_rng();

        loop {
            if count == tests {
                break;
            }
            let dice_count = rng.gen_range(1..=100);
            let dice_sides = rng.gen_range(4..=20);
            let expr = format!("{dice_count}d{dice_sides}");

            let roll = roll(&expr).unwrap();

            assert!(roll >= dice_count);
            assert!(roll <= dice_count * dice_sides);

            count += 1;
        }
    }
}
