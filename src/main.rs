// use contractions::Contractions;
use std::error::Error;

use contractions::Contractions;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // let contractions = Contractions::default()?;
    // contractions.list();
    // println!("{}", "Huhu".replace("u", "you"));

    // Contractions::generate(
    //     contractions::CONTRACTIONS_PARTIAL_JSON,
    //     "data/generated/contractions_partial.json",
    // )?;

    let input = "I'm happy to meet you";
    // let replace_with = "i am";

    // println!(
    //     "Direct: {}",
    //     Regex::new(r"\b(?i)i'm(?-i)\b")
    //         .unwrap()
    //         .replace_all(input, replace_with)
    //         .into_owned(),
    // );

    // let e_short = format!(r"\b(?i){}(?-i)\b", "I'm");
    // let regex = Regex::new(&e_short).unwrap();
    // println!(
    //     "As var: {}",
    //     regex.replace_all(input, replace_with).into_owned(),
    // );

    // println!(
    //     "replacen: {}",
    //     input.replacen(r"\b(?i)i'm(?-i)\b", replace_with, 0)
    // );

    let contractions = Contractions::default()?;
    print!("Library: {}", contractions.expand(input));

    Ok(())
}
