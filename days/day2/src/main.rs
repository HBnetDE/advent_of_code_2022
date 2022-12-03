fn main() {
    let mut total_score_round_one = 0;
    let mut total_score_round_two = 0;
    let input = include_str!("../input");

    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let first = parts[0];
        let second = parts[1];

        let win_state = determine_win_state_round_one(first, second);
        let response = determine_response_round_two(first, second);

        total_score_round_one += match second {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!(),
        };
        total_score_round_one += match win_state {
            WinState::Lost => 0,
            WinState::Draw => 3,
            WinState::Won => 6,
        };

        total_score_round_two += match response {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!(),
        };
        total_score_round_two += match second {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => unreachable!(),
        }
    }

    println!("{total_score_round_one}");
    println!("{total_score_round_two}");
}

enum WinState {
    Won,
    Draw,
    Lost,
}

fn determine_win_state_round_one(first: &str, second: &str) -> WinState {
    match (first, second) {
        ("A", "Z") | ("B", "X") | ("C", "Y") => WinState::Lost,
        ("A", "Y") | ("B", "Z") | ("C", "X") => WinState::Won,
        _ => WinState::Draw,
    }
}

fn determine_response_round_two<'a>(first: &'a str, second: &str) -> &'a str {
    match (first, second) {
        ("A", "X") => "C",
        ("A", "Z") => "B",
        ("B", "X") => "A",
        ("B", "Z") => "C",
        ("C", "X") => "B",
        ("C", "Z") => "A",
        (x, "Y") => x,
        _ => unreachable!(),
    }
}
