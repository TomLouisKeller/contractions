#![allow(non_snake_case)]
use super::*;
use std::error::Error;

// expand()

#[test]
fn expand__single_apostroph() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(
        contractions.expand("I'm happy to meet you"),
        "I am happy to meet you"
    );
    Ok(())
}

#[test]
fn expand__double_apostroph() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(
        contractions.expand("I'm'a head out"),
        "I am about to head out"
    );
    Ok(())
}

#[test]
fn expand__double_apostroph_would_have() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(contractions.expand("it'd've"), "it would have");
    Ok(())
}

#[test]
fn expand__double_apostroph_will_have() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(contractions.expand("he'll've"), "he will have");
    Ok(())
}

#[test]
fn expand__double_apostroph_are() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(contractions.expand("you'ren't"), "you are not");
    Ok(())
}

#[test]
fn expand__multiple_terms() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(
        contractions.expand("I'm gonna make you an offer you can't refuse"),
        "I am going to make you an offer you can not refuse"
    );
    Ok(())
}

#[test]
fn expand__period_after_contraction() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(contractions.expand("I can't."), "I can not.");
    Ok(())
}

#[test]
fn expand__capitalization() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(contractions.expand("Can't"), "Can not");
    Ok(())
}

/// Possessives are "Tom's car", "England's navy" - make sure we don't remove those 's
#[test]
fn expand__dont_replace_possessives() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default()?;
    assert_eq!(
        contractions.expand("Your brother's son"),
        "Your brother's son"
    );
    Ok(())
}
