use std::{collections::HashMap, fmt::Display, ops::Not};

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MatchResult {
    Won,
    Drawn,
    Lost,
}

impl MatchResult {
    fn from(s: &str) -> Option<Self> {
        match s {
            "win" => Some(MatchResult::Won),
            "draw" => Some(MatchResult::Drawn),
            "loss" => Some(MatchResult::Lost),
            _ => None,
        }
    }
}

impl Not for MatchResult {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            MatchResult::Won => MatchResult::Lost,
            MatchResult::Drawn => MatchResult::Drawn,
            MatchResult::Lost => MatchResult::Won,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TeamInfo {
    name: String,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    points: i32,
}

impl TeamInfo {
    fn new(name: &str) -> Self {
        TeamInfo {
            name: String::from(name),
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            points: 0,
        }
    }

    fn play(&mut self, result: MatchResult) {
        self.played += 1;
        match result {
            MatchResult::Won => {
                self.won += 1;
                self.points += 3
            }
            MatchResult::Drawn => {
                self.drawn += 1;
                self.points += 1
            }
            MatchResult::Lost => self.lost += 1,
        }
    }
}

impl PartialOrd for TeamInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (-self.points, &self.name).partial_cmp(&(-other.points, &other.name))
    }
}

impl Ord for TeamInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (-self.points, &self.name).cmp(&(-other.points, &other.name))
    }
}

impl Display for TeamInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            self.name, self.played, self.won, self.drawn, self.lost, self.points
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut results: HashMap<&str, TeamInfo> = HashMap::new();
    for line in match_results.lines() {
        let mut parts = line.split(';');
        let team1 = parts.next().unwrap();
        let team2 = parts.next().unwrap();
        let result = MatchResult::from(parts.next().unwrap()).unwrap();
        results
            .entry(team1)
            .or_insert_with(|| TeamInfo::new(team1))
            .play(result);
        results
            .entry(team2)
            .or_insert_with(|| TeamInfo::new(team2))
            .play(!result);
    }
    let mut ret = String::from(HEADER);
    let mut info: Vec<TeamInfo> = results.into_values().collect();
    info.sort();
    for team_info in info.into_iter() {
        ret.push_str(&team_info.to_string());
    }
    ret
}
