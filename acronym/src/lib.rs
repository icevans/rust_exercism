/// An enum representing the current state of what we are seeking in the string as we build up
/// our abbreviation. The logic of building an abbreviation can be clearly expressed as a Finite
/// State Machine
enum SeekState {
    /// Seeking an alphabetical character that signals the start of the next word. This is always
    /// the first state of our state machine.
    ///
    /// Valid next states: LowerCaseLetterInWord
    NextWordStart,

    /// Seeking a lower case letter in the current word -- we have already found the start of the
    /// current word and are checking that it is not uppercase like GNU
    ///
    /// Valid next states: EndOfCurrentWord, NextWordStart, CapitalLetterInWord
    LowerCaseLetterInWord,

    /// See the next capital letter in the current word -- CamelCase words should have all capital
    /// letters in them appear in the final acronym, so we must find them all
    ///
    /// Valid next states: NextWordStart
    CapitalLetterInWord,

    /// Seeking the first word separator (hyphen or whitespace) that signals the end of the current
    /// word. This state is only entered if we failed to find a lower case letter after the start
    /// of the word (such as GNU). In that case, we don't want to include any other letters from
    /// the word in the final acroynm
    ///
    /// Valid next states: NextWordStart
    EndOfCurrentWord,
}

/// Returns an abbreviation of the given string slice. Words can be separated by white space or
/// hyphens, only the first letter from already contained acronyms makes it into the final
/// abbreviation, and CamelCase words will get all of their uppercase letters included
pub fn abbreviate(phrase: &str) -> String {
    let mut acronym_pieces = String::new();
    let mut state = SeekState::NextWordStart;

    for c in phrase.chars() {
        match state {
            SeekState::NextWordStart => {
                if !c.is_alphabetic() {
                    continue;
                }

                acronym_pieces.push(c.to_ascii_uppercase());

                state = SeekState::LowerCaseLetterInWord
            }
            SeekState::LowerCaseLetterInWord => {
                if c.is_whitespace() || c == '-' {
                    state = SeekState::NextWordStart;
                    continue;
                }

                if c.is_ascii_uppercase() {
                    state = SeekState::EndOfCurrentWord;
                    continue;
                }

                state = SeekState::CapitalLetterInWord
            }
            SeekState::EndOfCurrentWord => {
                if c.is_whitespace() || c == '-' {
                    state = SeekState::NextWordStart;
                }
            }
            SeekState::CapitalLetterInWord => {
                if c.is_whitespace() || c == '-' {
                    state = SeekState::NextWordStart;
                    continue;
                }

                if c.is_ascii_uppercase() {
                    acronym_pieces.push(c);
                }
            }
        }
    }

    acronym_pieces
}
