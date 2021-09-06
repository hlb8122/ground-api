#![feature(generic_const_exprs, adt_const_params)]

mod sha256;

use sha256::sha256;

/// A long list of nothing.
const ZERO: [u8; 32] = [0; 32];

/// It is all ground up.
const PASSCODE_DIGEST: [u8; 32] = [
    200, 132, 175, 41, 130, 94, 133, 45, 223, 232, 43, 115, 66, 33, 215, 35, 248, 205, 77, 134,
    172, 248, 52, 64, 196, 178, 179, 171, 246, 94, 156, 214,
];

/// Might be hard to crack.
pub struct Safe;

impl Safe {
    /// It's on the front on the safe.
    pub fn keypad(&self) -> Keypad<ZERO, 0> {
        Keypad
    }

    /// It opens!
    #[doc(hidden)]
    #[allow(unused_variables)]
    pub fn turn_handle(self, entry: Entry<PASSCODE_DIGEST>) -> Treasure {
        Treasure
    }
}

/// It has buttons that can be pressed.
#[non_exhaustive]
pub struct Keypad<const INPUT: [u8; 32], const INDEX: usize>;

/// [`u8`]s are generally gentle creatures and don't deserve to be pushed around.
pub const fn push_key(char: u8, mut input: [u8; 32], index: usize) -> [u8; 32] {
    if index < 32 {
        input[index] = char;
    }
    input
}

impl<const INPUT: [u8; 32], const INDEX: usize> Keypad<INPUT, INDEX> {
    /// Beep.
    pub fn press_key<const KEY: u8>(
        self,
    ) -> Keypad<{ push_key(KEY, INPUT, INDEX) }, { INDEX + 1 }> {
        Keypad
    }

    /// Boop.
    pub fn press_enter(self) -> Entry<{ sha256(INPUT) }> {
        Entry
    }

    /// Beep boop.
    pub fn reset(self) -> Keypad<ZERO, 0> {
        Keypad
    }
}

/// Fingers crossed...
#[non_exhaustive]
#[must_use]
pub struct Entry<const DIGEST: [u8; 32]>;

/// It glimmers.
#[non_exhaustive]
#[must_use]
pub struct Treasure;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mission_improbable() {
        let safe = Safe;
        let entry = safe
            .keypad()
            .press_key::<b'Z'>()
            .press_key::<b'i'>()
            .press_key::<b'o'>()
            .press_key::<b'n'>()
            .press_key::<b'5'>()
            .press_enter();
        // let treasure = safe.turn_handle(entry);
    }

    #[test]
    fn mission_impossible() {
        let safe = Safe;
        // let entry = safe.keypad().press_key();
        // safe.turn_handle(entry);
    }
}
