//! Provides a tile type and associated functions.

use itertools::Itertools;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Suit {
    Man,
    Pin,
    Sou,
    Hon,
}

/// TODO: docstrings
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Tile {
    suit: Suit,
    number: u8,
}

impl Tile {
    pub fn number(&self) -> u8 { self.number }

    pub fn is_man(&self) -> bool { self.suit == Suit::Man }

    pub fn is_pin(&self) -> bool { self.suit == Suit::Pin }

    pub fn is_sou(&self) -> bool { self.suit == Suit::Sou }

    pub fn is_wind(&self) -> bool { self.suit == Suit::Hon && self.number <= 4 }

    pub fn is_colour(&self) -> bool { self.suit == Suit::Hon && self.number >= 5 }

    pub fn is_number(&self) -> bool { self.is_man() || self.is_pin() || self.is_sou() }

    pub fn is_honour(&self) -> bool { self.suit == Suit::Hon }

    pub fn is_terminal(&self) -> bool { self.is_number() && (self.number == 1 || self.number == 9) }

    pub fn is_simple(&self) -> bool { self.is_number() && !self.is_terminal() }

    /// Returns the next tile in the ordering, for the same suit.
    /// Panics on the last tile of each suit.
    pub(crate) fn next(&self) -> Tile {
        if (self.is_number() && self.number == 9)
            || (self.is_wind() && self.number == 4)
            || (self.is_colour() && self.number == 7)
        {
            panic!("No next tile");
        }
        Tile { suit: self.suit, number: self.number + 1 }
    }

    /// Returns the previous tile in the ordering, for the same suit.
    /// Panics on the first tile of each suit.
    pub(crate) fn prev(&self) -> Tile {
        if (self.is_number() && self.number == 1)
            || (self.is_wind() && self.number == 1)
            || (self.is_colour() && self.number == 5)
        {
            panic!("No previous tile");
        }
        Tile { suit: self.suit, number: self.number - 1 }
    }

    fn type_char(&self) -> char {
        match self.suit {
            Suit::Man => 'm',
            Suit::Pin => 'p',
            Suit::Sou => 's',
            Suit::Hon => 'z',
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.type_char())
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.type_char())
    }
}

impl Default for Tile {
    fn default() -> Tile { tile_from_string("1z") }
}

/// Converts a Tenhou-style string to a tile.
///
/// # Example
///
/// ```rust
/// use toitoi::tile::tile_from_string;
///
/// let tile = tile_from_string("9m");
///
/// assert!(tile.is_man());
/// assert_eq!(tile.number(), 9);
/// ```
pub fn tile_from_string(string: &str) -> Tile {
    let chars: Vec<char> = string.chars().collect();
    let number = match chars[0].to_digit(10) {
        Some(n) => n as u8,
        None => panic!("Invalid number: {}", chars[0]),
    };
    let suit = match chars[1] {
        'm' => Suit::Man,
        'p' => Suit::Pin,
        's' => Suit::Sou,
        'z' => Suit::Hon,
        _ => panic!("Invalid suit: {}", chars[1]),
    };
    Tile { suit, number }
}

/// Converts a Tenhou-style string to a list of tiles.
///
/// # Example
///
/// ```rust
/// use toitoi::tile::tiles_from_string;
///
/// let tiles = tiles_from_string("12m34p56s17z");
///
/// assert_eq!(tiles.len(), 8);
///
/// assert!(tiles[0].is_man());
/// assert_eq!(tiles[0].number(), 1);
///
/// assert!(tiles[3].is_pin());
/// assert_eq!(tiles[3].number(), 4);
///
/// assert!(tiles[4].is_sou());
/// assert_eq!(tiles[4].number(), 5);
///
/// assert!(tiles[7].is_honour());
/// assert_eq!(tiles[7].number(), 7);
/// ```
pub fn tiles_from_string(string: &str) -> Vec<Tile> {
    let mut tiles = vec![];
    let mut number_stack: Vec<u8> = vec![];
    for s in string.chars() {
        if s.is_digit(10) {
            number_stack.push(s.to_digit(10).unwrap() as u8);
        } else {
            tiles.extend(number_stack.iter().map(|n| tile_from_string(&format!("{}{}", n, s))));
            number_stack.clear();
        }
    }
    tiles
}

/// Convert a tile to a Tenhou-stye string.
pub(crate) fn tile_to_string(tile: &Tile) -> String {
    format!("{}{}", tile.number, tile.type_char())
}

/// Converts a list of tiles to a Tenhou-style string.
pub(crate) fn tiles_to_string(tiles: &[Tile]) -> String {
    let mut string = String::new();
    if tiles.len() == 0 {
        return string;
    }
    let mut current_type = tiles[0].type_char();
    for tile in tiles {
        if tile.type_char() != current_type {
            string.push(current_type);
            current_type = tile.type_char();
        }
        string.push_str(&tile.number.to_string());
    }
    string.push(current_type);
    string
}

/// Returns a list of all 34 unique tiles.
pub(crate) fn all_tiles() -> Vec<Tile> {
    tiles_from_string("123456789m123456789p123456789s1234567z")
}

/// Returns a copy of `tiles` with the members of `tiles_to_remove` removed.
pub(crate) fn with_tiles_removed(tiles: &[Tile], tiles_to_remove: &[Tile]) -> Vec<Tile> {
    let mut result = tiles.to_vec();
    // TODO: Implement with `remove_item()` when stable
    for i in 0..(tiles_to_remove.len()) {
        for j in 0..(result.len()) {
            if result[j] == tiles_to_remove[i] {
                result.remove(j);
                break;
            }
        }
    }
    result
}

/// Checks if `tiles` contains all tiles in `tiles_to_find`.
pub(crate) fn has_tiles(tiles: &[Tile], tiles_to_find: &[Tile]) -> bool {
    //let unique_tiles_to_find: Vec<Tile> = tiles_to_find.sorted().dedup();
    for tile in tiles_to_find.iter().sorted().dedup() {
        let tiles_present = tiles.iter().filter(|&t| t == tile).count();
        let tiles_needed = tiles_to_find.iter().filter(|&t| t == tile).count();
        if tiles_present < tiles_needed {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_tile_from_string() {
        assert_eq!(tile_from_string("1m"), Tile { suit: Suit::Man, number: 1 });
        assert_eq!(tile_from_string("2p"), Tile { suit: Suit::Pin, number: 2 });
        assert_eq!(tile_from_string("9s"), Tile { suit: Suit::Sou, number: 9 });
        assert_eq!(tile_from_string("4z"), Tile { suit: Suit::Hon, number: 4 });
        assert_eq!(tile_from_string("5z"), Tile { suit: Suit::Hon, number: 5 });
    }

    #[test]
    fn test_tiles_from_string() {
        assert_eq!(
            tiles_from_string("123456789m123456789p123456789s1234567z"),
            all_tiles().to_vec()
        );
        assert_eq!(
            tiles_from_string("5p5z"),
            [Tile { suit: Suit::Pin, number: 5 }, Tile { suit: Suit::Hon, number: 5 }]
        );
    }

    #[test]
    fn test_tiles_to_string() {
        assert_eq!(tiles_to_string(&all_tiles()), "123456789m123456789p123456789s1234567z",);
        assert_eq!(
            tiles_to_string(&[
                t("1m"),
                t("2m"),
                t("3p"),
                t("4p"),
                t("5s"),
                t("6s"),
                t("1z"),
                t("7z")
            ]),
            "12m34p56s17z"
        );
    }

    #[test]
    fn test_tile_ordering() {
        assert_eq!(t("1m").cmp(&t("1m")), Ordering::Equal);
        assert_eq!(t("7z").cmp(&t("7z")), Ordering::Equal);
        assert_eq!(t("1m").cmp(&t("2m")), Ordering::Less);
        assert_eq!(t("2m").cmp(&t("1p")), Ordering::Less);
        assert_eq!(t("1z").cmp(&t("9s")), Ordering::Greater);
        assert_eq!(t("5z").cmp(&t("4z")), Ordering::Greater);
    }

    #[test]
    fn test_tile_next() {
        assert_eq!(t("1m").next(), t("2m"));
        assert_eq!(t("3p").next(), t("4p"));
        assert_eq!(t("5s").next(), t("6s"));
        assert_eq!(t("1z").next(), t("2z"));
        assert_eq!(t("6z").next(), t("7z"));
    }
    #[test]
    fn test_tile_prev() {
        assert_eq!(t("9m").prev(), t("8m"));
        assert_eq!(t("7p").prev(), t("6p"));
        assert_eq!(t("5s").prev(), t("4s"));
        assert_eq!(t("4z").prev(), t("3z"));
        assert_eq!(t("6z").prev(), t("5z"));
    }

    #[test]
    #[should_panic]
    fn test_tile_no_next_man() { t("9m").next(); }
    #[test]
    #[should_panic]
    fn test_tile_no_next_pin() { t("9p").next(); }
    #[test]
    #[should_panic]
    fn test_tile_no_next_sou() { t("9s").next(); }
    #[test]
    #[should_panic]
    fn test_tile_no_next_wind() { t("4z").next(); }
    #[test]
    #[should_panic]
    fn test_tile_no_next_colour() { t("7z").next(); }
    #[test]
    #[should_panic]
    fn test_tile_no_prev_man() { t("1m").prev(); }
    #[test]
    #[should_panic]
    fn test_tile_no_prev_pin() { t("1p").prev(); }
    #[test]
    #[should_panic]
    fn test_tile_no_prev_sou() { t("1s").prev(); }
    #[test]
    #[should_panic]
    fn test_tile_no_prev_wind() { t("1z").prev(); }
    #[test]
    #[should_panic]
    fn test_tile_no_prev_colour() { t("5z").prev(); }

    #[test]
    fn test_with_tiles_removed() {
        assert_eq!(with_tiles_removed(&ts("1m22p3s446z"), &ts("1m2p3s4z"),), ts("2p46z"),);
    }

    fn t(tile: &str) -> Tile { tile_from_string(tile) }

    fn ts(tiles: &str) -> Vec<Tile> { tiles_from_string(tiles) }
}
