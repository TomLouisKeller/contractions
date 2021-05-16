#![allow(non_snake_case)]
use super::*;

// expand()

// TODO: use a framework to turn these into one test with a single setup
// for Contractions::default()

#[test]
fn expand__single_apostroph() {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.expand("I'm happy to meet you"),
        "I am happy to meet you"
    );
}

#[test]
fn expand__double_apostroph() {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.expand("I'm'a head out"),
        "I am about to head out"
    );
}

#[test]
fn expand__period_after_non_contraction() {
    let contractions = Contractions::default();
    assert_eq!(contractions.expand("Hello there."), "Hello there.");
}

// #[test]
// fn expand__period_after_contraction() {
//     let contractions = Contractions::default();
//     assert_eq!(contractions.expand("I can't."), "I can not.");
// }

/// Possessives are "Tom's car", "England's navy" - make sure we don't remove those 's
#[test]
fn expand__dont_replace_possessives() {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.expand("Your brother's son"),
        "Your brother's son"
    );
}

// #[test]
// fn test_replace() {
//     let x = "No, you can't.";

//     assert_eq!(x.replace("can't", "can not"), "No, you can not.");
// }
