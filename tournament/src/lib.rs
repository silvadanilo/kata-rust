use std::collections::HashMap;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug)]
struct InvalidValue;

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = InvalidValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "win" => Ok(Outcome::Win),
            "loss" => Ok(Outcome::Loss),
            "draw" => Ok(Outcome::Draw),
            _ => Err(InvalidValue),
        }
    }
}

#[derive(Debug)]
struct PlayedMatch {
    home: String,
    visitor: String,
    outcome: Outcome,
}

impl FromStr for PlayedMatch {
    type Err = InvalidValue;
    fn from_str(s: & str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(';').collect();
        if 3 != splitted.len() {
            return Err(InvalidValue);
        }

        let outcome = try!(splitted[2].parse());
        Ok(PlayedMatch {
            home: splitted[0].to_string(),
            visitor: splitted[1].to_string(),
            outcome: outcome
        })
    }
}

impl PlayedMatch {
    pub fn process(&self, league: &mut HashMap<String, Team>) -> bool {
        {
            let ref mut home_team = league.entry(self.home.clone())
                .or_insert(Team::new(self.home.clone()));
            match self.outcome {
                Outcome::Win => {
                    home_team.win();
                }
                Outcome::Loss => {
                    home_team.lost();
                }
                Outcome::Draw => {
                    home_team.draw();
                }
            }
        }
        {
            let ref mut visitor_team = league.entry(self.visitor.clone())
                .or_insert(Team::new(self.visitor.clone()));
            match self.outcome {
                Outcome::Win => {
                    (*visitor_team).lost();
                }
                Outcome::Loss => {
                    (*visitor_team).win();
                }
                Outcome::Draw => {
                    (*visitor_team).draw();
                }
            }
        }

        true
    }
}

#[derive(Default, Debug, Eq, PartialEq, PartialOrd)]
struct Team {
    name: String,
    mp: u8,
    w: u8,
    d: u8,
    l: u8,
    point: u8,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            name: name,
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
            point: 0,
        }
    }

    pub fn win(&mut self) {
        self.mp += 1;
        self.w += 1;
        self.point += 3;
    }

    pub fn lost(&mut self) {
        self.mp += 1;
        self.l += 1;
    }

    pub fn draw(&mut self) {
        self.mp += 1;
        self.d += 1;
        self.point += 1;
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Team) -> Ordering {
        match other.point.cmp(&self.point) {
            Ordering::Equal => self.name.cmp(&other.name),
            order => order,
        }
    }
}

pub fn tally(played_matches: &String) -> String {
    let mut league: HashMap<String, Team> = HashMap::new();

    for played_match in played_matches.lines() {
        if let Ok(played_match) = played_match.parse::<PlayedMatch>() {
            played_match.process(&mut league);
        }
    }

    let mut league: Vec<_> = league.values().collect();

    league.sort();

    let mut s = format!("{:31}| MP |  W |  D |  L |  P", "Team");
    for team in league {
        let ref partial = format!("\n{:31}|  {} |  {} |  {} |  {} |  {}",
                       team.name,
                       team.mp,
                       team.w,
                       team.d,
                       team.l,
                       team.point);
        s.push_str(partial);
    }

    s
}
