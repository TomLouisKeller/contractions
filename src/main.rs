use std::error::Error;

use contractions::Contractions;

fn main() -> Result<(), Box<dyn Error>> {
    let contractions = Contractions::default();
    print!("Library: {}", contractions.expand("I'm happy to meet you"));
    Ok(())
}
