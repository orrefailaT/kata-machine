fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    let step = (breaks.len() as f32).sqrt() as usize;

    let Some(floor) = breaks.iter().step_by(step).position(|b| *b) else {
        return None
    };

    let start = step * (floor - 1) + 1;
    Some(breaks[start..].iter().position(|b| *b).unwrap() + start)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};

    #[test]
    fn test_two_crystal_balls() {
        let mut data = [false; 10000];
        let idx = rand::thread_rng().gen_range(0..data.len());
        println!("{}", idx);

        for i in idx..data.len() {
            data[i] = true;
        }

        assert_eq!(two_crystal_balls(&data), Some(idx));
    }

    #[test]
    fn test_no_breaks() {
        let data = [false; 821];
        assert_eq!(two_crystal_balls(&data), None);
    }
}
