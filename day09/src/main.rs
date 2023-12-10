#[allow(unused)]
const SAMPLE_01: &str = include_str!("../sample01.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    {
        let p = parse(INPUT);
        let ans: i32 = p.into_iter().map(|x| solve_line_forward(&x)).sum();
        assert_eq!(2_105_961_943, ans);
    }
    {
        let p = parse(INPUT);
        let ans: i32 = p.into_iter().map(|x| solve_line_backward(&x)).sum();
        assert_eq!(1_019, ans);
    }
}

fn find_diffs(l: &[i32]) -> Vec<Vec<i32>> {
    let mut diffs = Vec::new();
    diffs.push(l.to_vec());
    loop {
        let d: Vec<_> = diffs
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        diffs.push(d.clone());
        if d.into_iter().all(|x| x == 0) {
            break;
        }
    }

    diffs
}

#[inline]
fn solve_line_intern<CalcFn, PushFn>(l: &[i32], calc_fn: CalcFn, push_fn: PushFn) -> i32
where
    CalcFn: Fn(&[i32], &[i32]) -> i32,
    PushFn: Fn(&mut Vec<i32>, i32),
{
    let mut diffs = find_diffs(l);
    let mut y = 0;
    for i in (1..diffs.len()).rev() {
        let line = &diffs[i];
        let prev_line = &diffs[i - 1];
        let x = calc_fn(line, prev_line);
        push_fn(&mut diffs[i - 1], x);
        y = x;
    }
    y
}

fn solve_line_backward(l: &[i32]) -> i32 {
    solve_line_intern(
        l,
        |a, b| *b.first().unwrap() - *a.first().unwrap(),
        |v, x| v.insert(0, x),
    )
}
fn solve_line_forward(l: &[i32]) -> i32 {
    solve_line_intern(
        l,
        |a, b| *a.last().unwrap() + *b.last().unwrap(),
        std::vec::Vec::push,
    )
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for l in s.lines() {
        let numbers = l.split(' ').filter_map(|s| s.parse().ok()).collect();
        res.push(numbers);
    }

    res
}
