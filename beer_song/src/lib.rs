pub fn beer_song() {
    for i in (3..100).rev() {
        println!("{i} bottles of beer on the wall, {i} bottles of beer.\nTake one down and pass it around, now there's {} more bottles of beer on the wall!", (i - 1));
    }
    println!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, now there's 1 more bottle of beer on the wall!", );
    println!("1 bottle of beer on the wall, 1 bottle of beer.\nTake one down and pass it around, there's no more bottles of beer on the wall!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        beer_song();
    }
}
