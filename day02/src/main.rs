use std::str::FromStr;

use anyhow::{anyhow, Context};

#[allow(unused)]
const SAMPLE_1: &str = include_str!("../sample1.txt");
#[allow(unused)]
const INPUT: &str = include_str!("../input.txt");

const CONSTRAINTS: [usize; 3] = [12, 13, 14];
const fn get_max(color: &Color) -> usize {
    match color {
        Color::Red => CONSTRAINTS[0],
        Color::Green => CONSTRAINTS[1],
        Color::Blue => CONSTRAINTS[2],
    }
}

fn main() {
    color_eyre::install().unwrap();
    {
        let sample_games = parse(SAMPLE_1).unwrap();
        let p1 = solve_p1(&sample_games);
        assert_eq!(p1, 8);

        let p2 = solve_p2(&sample_games);
        assert_eq!(p2, 2286);
    }

    let games = parse(INPUT).unwrap();

    let ans = solve_p1(&games);
    println!("p1 {ans}");

    let ans = solve_p2(&games);
    println!("p2 {ans}");
}

fn solve_p1(games: &[Game]) -> usize {
    let mut success = 0;

    for game in games {
        let mut possible = true;
        for set in &game.sets {
            for show in &set.shows {
                let max = get_max(&show.color);
                if show.amnt > max {
                    possible = false;
                }
            }
        }
        if possible {
            success += game.id;
        }
    }

    success
}

const fn idx_by_color(color: &Color) -> usize {
    match color {
        Color::Red => 0,
        Color::Green => 1,
        Color::Blue => 2,
    }
}

fn solve_p2(games: &[Game]) -> usize {
    let mut ret = 0;

    for game in games {
        let mut max_by_color = [0, 0, 0];
        for set in &game.sets {
            for show in &set.shows {
                let last_max = max_by_color[idx_by_color(&show.color)];
                if show.amnt > last_max {
                    max_by_color[idx_by_color(&show.color)] = show.amnt;
                }
            }
        }
        let pwr = max_by_color.into_iter().product::<usize>();
        ret += pwr;
    }

    ret
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            _ => Err(anyhow!("invalid color {s}")),
        }
    }
}

#[derive(Debug)]
struct Show {
    amnt: usize,
    color: Color,
}

#[derive(Default, Debug)]
struct Set {
    shows: Vec<Show>,
}

#[derive(Default, Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

fn parse(s: &str) -> anyhow::Result<Vec<Game>> {
    let mut games = Vec::with_capacity(1024);
    for line in s.split('\n') {
        let (prefix, plays) = line.split_once(": ").context("invalid beginning of line")?;
        let (_prefix, id) = prefix.split_once(' ').context("Invalid prefix")?;
        let id = id.parse().context("invalid game id")?;
        let mut g = Game {
            id,
            ..Default::default()
        };

        for set_str in plays.split("; ") {
            let mut set = Set::default();

            for play in set_str.split(", ") {
                let (amnt, color_str) = play
                    .split_once(' ')
                    .context(anyhow!("invalid show {play}"))?;
                let color = color_str.parse().context("invalid color")?;
                let amnt = amnt.parse().context("invalid amnt")?;

                set.shows.push(Show { amnt, color })
            }
            g.sets.push(set)
        }
        games.push(g);
    }

    Ok(games)
}
