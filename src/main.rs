use itertools::Itertools;
use std::io;

fn main() {
    let mut modules: Vec<(f32, i32, bool)> = Vec::new();
    loop {
        println!("Enter module grade, credits, whether can be s-coded (true/false). Leave blank to complete");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.retain(|c| !c.is_whitespace());
        if buffer.is_empty() {
            break;
        }
        let splits: Vec<_> = buffer.split(",").collect();
        if splits.len() == 3 {
            if let (Ok(grade), Ok(credits), Ok(s_code)) = (splits[0].parse(), splits[1].parse(), splits[2].parse()) {
                modules.push((grade, credits, s_code));
                continue;
            }
        }
        println!("Incorrect line");
    }


    modules.sort_unstable_by(|(a, _, _), (b, _, _) | a.partial_cmp(b).unwrap());

    let (s_codeable, base) : (Vec<_>, Vec<_>) = modules.iter().partition(|x| x.2);
    for i in 0..=s_codeable.len() {
        for combination in s_codeable.iter().combinations(i) {
            let mut modules = base.clone();
            let s_coded: Vec<_> = s_codeable.iter().filter(|x| !combination.contains(x)).collect();
            modules.extend(combination);
            if modules.len() == 0 {
                break;
            }

            let (total_grade, total_credits) = modules.iter().fold((0., 0), |(total_grade, total_credits), (grade, credits, _)| (total_grade + *grade * *credits as f32, total_credits + *credits));
            let mean = total_grade / total_credits as f32;
            let mid = modules.len() / 2;
            let median = if modules.len() % 2 == 1 {
                modules[mid].0
            } else {
                (modules[mid - 1].0 + modules[mid].0) / 2.0
            };
            println!("mean: {}, median: {}, total credits: {}, s-code: {:?}", mean, median, total_credits, s_coded);
        }
    }
}