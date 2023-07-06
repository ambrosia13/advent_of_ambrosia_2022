#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
struct RockPaperScissorsMoveError;

#[derive(Debug)]
struct MatchResultError;

impl Move {
    // Returns the move associated with the given char
    fn parse(character: char) -> Result<Self, RockPaperScissorsMoveError> {
        match character {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(RockPaperScissorsMoveError),
        }
    }

    // Returns the result of the match between two moves
    fn get_match_result(self, other: Self) -> MatchResult {
        let win_move: Move;
        let lose_move: Move;

        match self {
            Move::Rock => {
                win_move = Move::Scissors;
                lose_move = Move::Paper;
            }
            Move::Paper => {
                win_move = Move::Rock;
                lose_move = Move::Scissors;
            }
            Move::Scissors => {
                win_move = Move::Paper;
                lose_move = Move::Rock;
            }
        };

        if other == win_move {
            return MatchResult::Win;
        } else if other == lose_move {
            return MatchResult::Loss;
        }

        MatchResult::Draw
    }

    fn eval_score(self, other: Self) -> i32 {
        let score = match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        match Self::get_match_result(self, other) {
            MatchResult::Win => score + 6,
            MatchResult::Draw => score + 3,
            _ => score,
        }
    }
}

impl MatchResult {
    fn parse(character: char) -> Result<Self, MatchResultError> {
        match character {
            'X' => Ok(MatchResult::Loss),
            'Y' => Ok(MatchResult::Draw),
            'Z' => Ok(MatchResult::Win),
            _ => Err(MatchResultError),
        }
    }
}

pub fn part_one(input: &str) -> i32 {
    let lines = input.lines();

    let mut score = 0;

    for line in lines {
        let line = line.trim();

        let mut chars = line.chars();
        let enemy_move = chars.next().expect("failed to read enemy input");
        let my_move = chars.nth(1).expect("failed to read my input");

        let enemy_move = Move::parse(enemy_move).expect("Failed to parse enemy input into Move");
        let my_move = Move::parse(my_move).expect("Failed to parse my input into Move");

        score += Move::eval_score(my_move, enemy_move);
    }

    score
}

pub fn part_two(input: &str) -> i32 {
    let lines = input.lines();

    let mut score = 0;

    for line in lines {
        let line = line.trim();

        let mut chars = line.chars();
        let enemy_move = chars.next().expect("failed to read enemy input");
        let match_result = chars.nth(1).expect("failed to read match result");

        let enemy_move = Move::parse(enemy_move).expect("failed to parse");
        let match_result = MatchResult::parse(match_result).expect("failed to parse match result");

        let my_move = match enemy_move {
            Move::Rock => match match_result {
                MatchResult::Win => Move::Paper,
                MatchResult::Draw => Move::Rock,
                MatchResult::Loss => Move::Scissors,
            },
            Move::Paper => match match_result {
                MatchResult::Win => Move::Scissors,
                MatchResult::Draw => Move::Paper,
                MatchResult::Loss => Move::Rock,
            },
            Move::Scissors => match match_result {
                MatchResult::Win => Move::Rock,
                MatchResult::Draw => Move::Scissors,
                MatchResult::Loss => Move::Paper,
            },
        };

        score += Move::eval_score(my_move, enemy_move);
    }

    score
}

pub fn test() {
    println!("Day 2:");

    let input = include_str!("input.txt");

    println!("\tPart one: {}", part_one(input));
    println!("\tPart two: {}", part_two(input));

    println!();
}
