use std::{env, io::Read};

use anyhow::Context;

fn main() {
    let input_file_name = env::args().nth(1).unwrap();
    let mut input_file = std::fs::OpenOptions::new()
        .read(true)
        .open(input_file_name)
        .unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    compile(&input).unwrap();
}

fn parse_map_line(line: &str) -> anyhow::Result<(u64, u64, u64)> {
    let mut it = line.splitn(3, ' ');
    let dest: u64 = it
        .next()
        .context("line w/o destination")?
        .parse()
        .context("destination not numeric")?;
    let src: u64 = it
        .next()
        .context("line w/o src")?
        .parse()
        .context("destination not numeric")?;
    let amnt: u64 = it
        .next()
        .context("line w/o amnt")?
        .parse()
        .context("destination not numeric")?;

    Ok((dest, src, amnt))
}

fn compile_intern(lines: &[String], fn_name: &str) -> anyhow::Result<()> {
    let fn_name = fn_name.replace('-', "_");
    let t = " ".repeat(4);

    let (input_var_name, _) = fn_name
        .split_once("_to_")
        .context(format!("invalid fn name {fn_name}"))?;
    println!("pub fn {fn_name}({input_var_name}: u64) -> u64 {{");

    for line in lines {
        let (dest, src, amnt) = parse_map_line(line).context("failed to parse line")?;
        println!("{t}#[allow(clippy::identity_op)]");
        println!("{t}if ({src}..=({src}+{amnt}-1)).contains(&{input_var_name}) {{");
        println!("{t}{t}#[allow(clippy::identity_op)]");
        println!("{t}{t}return {dest} + ({input_var_name} - {src});");
        println!("{t}}}");
    }

    println!("{t}{input_var_name}");
    println!("}}");

    Ok(())
}

fn compile(input: &str) -> anyhow::Result<()> {
    let mut blocks = Vec::default();
    let mut curr_block = Vec::default();
    for line in input.lines() {
        if line.is_empty() {
            blocks.push(curr_block);
            curr_block = Vec::default();
            continue;
        }

        curr_block.push(line.to_owned());
    }
    blocks.push(curr_block);

    assert_eq!(8, blocks.len());

    for block in blocks.iter().skip(1) {
        let name = block[0].replace(" map:", "");
        compile_intern(&block[1..], &name)?;
    }

    let t = " ".repeat(4);

    println!("pub fn seeds() -> Vec<std::ops::RangeInclusive<u64>> {{");
    println!("{t}let mut seeds = Vec::default();");
    let mut seeds = blocks[0][0].split(' ').skip(1).collect::<Vec<_>>();
    for seed_pair in seeds.chunks_mut(2) {
        println!(
            "{t}seeds.push({}..=({}+{}));",
            seed_pair[0], seed_pair[1], seed_pair[0]
        );
    }
    println!("{t}seeds");
    println!("}}");

    Ok(())
}
