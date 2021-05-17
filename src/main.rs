use contractions::Contractions;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let contractions = Contractions::default()?;
    // contractions.list();
    println!("{}", "Huhu".replace("u", "you"));
    Ok(())
}
