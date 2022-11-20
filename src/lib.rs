pub mod algorithms; 

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    //play six rounds where it invokes guesser each round
    let mut history = Vec::new();
    //Wordle only allows six guesses.
    //we allow more to avoid chopping off the score distribution for stats purposes.
    for i in 0..=32 {
        let guess = guesser.guess(&history);
        if guess == answer {
            return Some(i);
        }
        let Correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            word: guess,
            mask: Correctness,
        });
    }
    None
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}
impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(answer.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        // Mark things green
        for(i, (a,g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            } 
        }
        //Mark things yellow
        let mut used = [false; 5];
        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }
        
        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                //Already marked as green
                continue;
            }
       
            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        
        c
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5]
}
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
#[cfg(test)]
mod tests {
    mod compute {
        use crate::Correctness;
        #[test]
        fn basic() {
            assert_eq!(
                Correctness::compute("abcde", "abcde"),
                [Correctness::Correct; 5]
            );
        }

    }
}