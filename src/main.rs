use rayon::prelude::*;
use regex::{self, Regex};

fn main() {
    let file = std::fs::read_to_string("russian_nouns.txt").unwrap();

    let russian_nouns: Vec<_> = file.split('\n').collect();
    let regexes: Vec<Regex> = russian_nouns
        .par_iter()
        .map(|&w| {
            let mut pattern = w
                .chars()
                .rev()
                .take(w.chars().count() - 1)
                // "(" + cur + acc + ")?"
                // .fold("", |acc, cur| format!("(?:{}{})?", cur, acc));
                .fold(r"".to_string(), |acc, cur| {
                    ["(", cur.to_string().as_str(), acc.as_str(), ")?"].concat()
                });
            pattern.insert(0, w.chars().nth(0).unwrap());
            pattern.push('$');

            Regex::new(pattern.as_str()).unwrap()
        })
        .collect();

    println!("{:?}", regexes[0]);

    println!("{:?}", russian_nouns.len());

    let zipped: Vec<_> = russian_nouns.iter().zip(regexes.iter()).collect();

    // .for_each(|(&w, r)| {})

    russian_nouns.par_iter().for_each(|&w| {
        // dbg!(w);
        zipped
            .par_iter()
            .filter(|o| *o.0 != w)
            .for_each(|(&ow, re)| {
                // dbg!(ow);
                let contamination = re.find(w);
                if let Some(c) = contamination {
                    // let l = w.chars().count();
                    let l = (c.end() - c.start()) / 2;

                    if l >= 4 && l <= 6 && l < w.chars().count() && l < ow.chars().count() {
                        dbg!(l, w, ow);
                    }
                }
                // let is_contamination = re.is_match(w);
                // if is_contamination {
                //     dbg!(w, ow);
                // }
            })
    })
}
