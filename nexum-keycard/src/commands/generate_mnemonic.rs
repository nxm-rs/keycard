use bip39::{Language, Mnemonic};
use nexum_apdu_globalplatform::constants::status::*;
use nexum_apdu_macros::apdu_pair;

use crate::Error;

use super::CLA_GP;

apdu_pair! {
    /// GENERATE MNEMONIC command for Keycard
    pub struct GenerateMnemonic {
        command {
            cla: CLA_GP,
            ins: 0xD2,
            required_security_level: SecurityLevel::enc_mac(),

            builders {
                /// Create a GENERATE MNEMONIC command with a given number of words (12, 15, 18, 21, 24)
                pub fn with_words(words: u8) -> Result<Self, GenerateMnemonicError> {
                    match words {
                        12 | 15 | 18 | 21 | 24 => Ok(Self::new(words / 3, 0x00).with_le(0)),
                        _ => Err(GenerateMnemonicError::IncorrectChecksumSize),
                    }
                }
            }
        }

        response {
            ok {
                /// Success response
                #[sw(SW_NO_ERROR)]
                #[payload(field = "words")]
                Success {
                    /// An array of u16 representing the mnemonic words
                    words: Vec<u8>
                }
            }

            errors {
                /// Incorrect P1/P2: Checksum is out of range (between 4 and 8)
                #[sw(SW_INCORRECT_P1P2)]
                #[error("Incorrect P1/P2: Checksum is out of range (between 4 and 8)")]
                IncorrectChecksumSize,
            }
        }
    }
}

impl GenerateMnemonicOk {
    /// Decode the card's u16 word-index array into a `Mnemonic` using
    /// the given language's wordlist.
    pub fn to_phrase(&self, language: Language) -> Result<Mnemonic, Error> {
        match self {
            Self::Success { words: words_u16 } => {
                let wordlist = language.word_list();
                let mut words = Vec::with_capacity(words_u16.len() / 2);

                for chunk in words_u16.chunks_exact(2) {
                    let index = u16::from_be_bytes([chunk[0], chunk[1]]) as usize;
                    words.push(*wordlist.get(index).ok_or_else(|| {
                        Error::InvalidDerivationArguments(format!(
                            "wordlist index {index} out of range"
                        ))
                    })?);
                }

                Mnemonic::parse_in_normalized(language, &words.join(" ")).map_err(Into::into)
            }
        }
    }
}
