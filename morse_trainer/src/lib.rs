/*!
Convert a string to a iterator over `Morse` enums.
*/

enum Morse {
    Dit,            // 1 time unit on
    Dah,            // 3 dits on
    InterGap,       // 1 dit off between elements
    ShortGap,       // 3 dits off between letters
    MediumGap,      // 7 dits off between words
}
