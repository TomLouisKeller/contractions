#![allow(non_snake_case)]
use std::error::Error;

use contractions::Contractions;

// apply()

#[test]
fn apply__single_apostroph() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.apply("i’m happy to meet you"),
        "i am happy to meet you"
    );
    Ok(())
}

#[test]
fn apply__double_apostroph() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.apply("i’m’a head out"),
        "i am about to head out"
    );
    Ok(())
}

#[test]
fn apply__double_apostroph_would_have() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("it’d’ve"), "it would have");
    Ok(())
}

#[test]
fn apply__double_apostroph_will_have() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("he’ll’ve"), "he will have");
    Ok(())
}

#[test]
fn apply__double_apostroph_are() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("you’ren’t"), "you are not");
    Ok(())
}

#[test]
fn apply__multiple_terms() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.apply("i’m gonna make you an offer you can’t refuse"),
        "i am going to make you an offer you can not refuse"
    );
    Ok(())
}

#[test]
fn apply__period_after_contraction() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("I can’t."), "I can not.");
    Ok(())
}

#[test]
fn apply__capitalization() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("Can’t"), "Can not");
    Ok(())
}

/// Possessives are "Tom’s car", "England’s navy" - make sure we don’t remove those ’s
#[test]
fn apply__dont_replace_possessives() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.apply("Your brother’s son"),
        "Your brother’s son"
    );
    Ok(())
}

/// Possessives are "Tom’s car", "England’s navy" - make sure we don’t remove those ’s
#[test]
fn apply__slang() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("r u ok?"), "are you ok?");
    Ok(())
}

/// Don’t split Adelphe’s into Adelp he is
#[test]
fn apply__do_not_replace_partials_where_not_intended() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(
        contractions.apply("Adelphe’s car is broken"),
        "Adelphe’s car is broken"
    );
    Ok(())
}

#[test]
fn apply__single_quotation_mark() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("I'm fine"), "I am fine");
    Ok(())
}

#[test]
fn apply__grave_accent() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    assert_eq!(contractions.apply("I`m fine"), "I am fine");
    Ok(())
}

#[test]
fn apply__new() {
    let contractions = Contractions::new();
    assert_eq!(contractions.apply("I`m fine"), "I`m fine");
}

#[test]
fn from_json__by_hand() -> Result<(), Box<dyn Error>> {
    let contractions_as_json = r#"
    [
        {
            "find": "\\b(?i)i['’`]m['’`]a(?-i)\\b",
            "replace": {
            "\\bi['’`]m['’`]a\\b": "i am about to",
            "\\bI['’`]m['’`]a\\b": "I am about to",
            "\\bI['’`]M['’`]A\\b": "I AM ABOUT TO",
            "\\b(?i)i['’`]m['’`]a(?-i)\\b": "i am about to"
            }
        }
    ]
    "#;
    let contractions = Contractions::from_json(&[&contractions_as_json])?;
    assert_eq!(
        contractions.apply("I`m`a woop your butt"),
        "I am about to woop your butt"
    );
    Ok(())
}

#[test]
fn from_json__from_file() -> Result<(), Box<dyn Error>> {
    let contractions =
        Contractions::from_json(&[&contractions::CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON])?;
    assert_eq!(
        contractions.apply("You mustnt do that!"),
        "You must not do that!"
    );
    assert_eq!(
        contractions.apply("You mustn't do that!"),
        "You mustn't do that!"
    );
    Ok(())
}

#[test]
fn add_from_json__from_file() -> Result<(), Box<dyn Error>> {
    let mut contractions = Contractions::new();
    contractions.add_from_json(&contractions::CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON)?;
    assert_eq!(
        contractions.apply("You mustnt do that!"),
        "You must not do that!"
    );
    Ok(())
}

#[test]
fn remove() -> Result<(), Box<dyn Error>> {
    let mut contractions = Contractions::new();
    assert_eq!("I’m happy", contractions.apply("I’m happy"));
    contractions.add_from_json(contractions::CONTRACTIONS_SINGLE_JSON)?;
    assert_eq!("I am happy", contractions.apply("I’m happy"));
    contractions.remove(r#"\b(?i)i['’`]m(?-i)\b"#);
    assert_eq!("I’m happy", contractions.apply("I’m happy"));
    Ok(())
}

#[test]
fn remove__keep_others() -> Result<(), Box<dyn Error>> {
    let mut contractions = Contractions::new();
    assert_eq!("I’ve stuff", contractions.apply("I’ve stuff"));
    contractions.add_from_json(contractions::CONTRACTIONS_SINGLE_JSON)?;
    assert_eq!("I have stuff", contractions.apply("I’ve stuff"));
    contractions.remove(r#"\b(?i)i['’`]m(?-i)\b"#);
    assert_eq!("I have stuff", contractions.apply("I’ve stuff"));

    Ok(())
}

#[test]
fn add() -> Result<(), Box<dyn Error>> {
    let mut contractions = Contractions::new();
    assert_eq!("I’m happy", contractions.apply("I’m happy"));
    let find = r#"\b(?i)i['’`]m(?-i)\b"#;
    let mut replace = linked_hash_map::LinkedHashMap::new();
    replace.insert(find.clone(), "I am");
    contractions.add(find, replace)?;
    assert_eq!("I am happy", contractions.apply("I’m happy"));

    Ok(())
}

#[test]
fn partials() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::from_json(&[contractions::CONTRACTIONS_PARTIAL_JSON])?;
    assert_eq!("I am happy", contractions.apply("I’m happy"));
    Ok(())
}
