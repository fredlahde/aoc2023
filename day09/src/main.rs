#[allow(unused)]
const SAMPLE_01: &str = include_str!("../sample01.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    {
        let p = parse(INPUT);
        let ans: i32 = p.into_iter().map(|x| solve_line_forward(&x)).sum();
        println!("p1 {ans}");
        assert_eq!(2105961943, ans);
    }
    {
        let p = parse(INPUT);
        let ans: i32 = p.into_iter().map(|x| solve_line_backward(&x)).sum();
        println!("p2 {ans}");
        assert_eq!(1019, ans);
    }
}

fn find_diffs(l: &[i32]) -> Vec<Vec<i32>> {
    let mut diffs = Vec::new();
    diffs.push(l.to_vec());
    loop {
        let mut d = Vec::new();
        for i in 1..diffs.last().unwrap().len() {
            d.push(diffs.last().unwrap()[i] - diffs.last().unwrap()[i - 1])
        }
        diffs.push(d.clone());
        if d.into_iter().all(|x| x == 0) {
            break;
        }
    }

    diffs
}

fn solve_line_backward(l: &[i32]) -> i32 {
    let mut diffs = find_diffs(l);

    let mut y = 0;
    for i in (1..=diffs.len() - 1).rev() {
        let line = &diffs[i];
        let prev_line = &diffs[i - 1];
        let x = prev_line.first().unwrap() - line.first().unwrap();
        diffs[i - 1].insert(0, x);
        y = x;
    }
    y
}

fn solve_line_forward(l: &[i32]) -> i32 {
    let mut diffs = find_diffs(l);
    let mut y = 0;
    for i in (1..=diffs.len() - 1).rev() {
        let line = &diffs[i];
        let prev_line = &diffs[i - 1];
        let x = line.last().unwrap() + prev_line.last().unwrap();
        diffs[i - 1].push(x);
        y = x;
    }
    y
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for l in s.lines() {
        let numbers = l.split(' ').filter_map(|s| s.parse().ok()).collect();
        res.push(numbers);
    }

    res
}
