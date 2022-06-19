use std::{collections::HashMap, fs};

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let decoded = decode(input);

    fs::write("output.txt", decoded.to_string()).unwrap();
}

/// Состояние символа внутри строки
#[derive(Clone)]
struct CharState {
    /// Количество вхождений символа в строке
    occurrences: usize,
    /// Количество способов вычеркнуть буквы так, чтобы остался палиндром с данной буквой в центре
    variants: usize,
}

impl CharState {
    fn new(occurrences: usize, variants: usize) -> Self {
        CharState {
            occurrences,
            variants,
        }
    }
}

fn decode(input: impl Into<String>) -> usize {
    let without_spaces = input.into().replace(' ', "");
    let mut prev_chars = HashMap::<char, usize>::new();
    let mut states = vec![CharState::new(0, 0); without_spaces.len()];

    for (i, c) in without_spaces.chars().enumerate() {
        states[i] = match prev_chars.get(&c) {
            Some(prev_index) => {
                let prev_state = &states[*prev_index];
                CharState::new(
                    prev_state.occurrences + 1,
                    prev_state.variants + // прошлое количество способов вычёркивания
                    prev_state.occurrences * (i - prev_index) // + все предыдущие входждения этого символа, поскольку они валидны для отрезка prev_index..i
                     + (i - prev_index - 1), // + варианты вычеркивания букв для вышеупомянутого отрезка
                )
            }
            None => CharState::new(0, 0),
        };
        prev_chars.insert(c, i);
    }

    states.iter().map(|state| state.variants).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(decode("treasure"), 8);
        assert_eq!(decode("you will never find the treasure"), 146);
        assert_eq!(decode("e".repeat(300_000)), 4499955000100000);
    }
}
