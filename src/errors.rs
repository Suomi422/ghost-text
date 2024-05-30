#[allow(dead_code)]
#[derive(Debug)]
pub enum LibraryErrors {
    Base64DecodingError,
    KeyError,
    UTF8Error,
    ValueError,
    PermissionError,
    KeyFileError,
    PlatformError,
    NoOperationTypeError,
    KeyFileNameError,
    BinaryFileDecryptingError,
    BinaryFileReadError,
    BinaryFileWriteError,
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct EncryptorError {
    code: LibraryErrors,
    pub error_msg: String,
}


impl EncryptorError {
    pub fn new(code: LibraryErrors) -> EncryptorError {
        let error_type = match code {
            LibraryErrors::Base64DecodingError => String::from("Could not decode value → Base64 decoding error"),
            LibraryErrors::KeyError => String::from("Secret key is wrong"),
            LibraryErrors::UTF8Error => String::from("Could not parse decoded data to UTF-8"),
            LibraryErrors::ValueError => String::from("Value is not usable"),
            LibraryErrors::PermissionError => String::from("Unable to create/read a key file. Make sure program has permissions to home folder"),
            LibraryErrors::KeyFileError => String::from("Unable to write a key file. Make sure data are right and file exists"),
            LibraryErrors::PlatformError => String::from("Unable to get home directory. Not supported platform"),
            LibraryErrors::NoOperationTypeError => String::from("No operation type was set"),
            LibraryErrors::KeyFileNameError => String::from("Secret Key filename must have more than 3 characters"),
            LibraryErrors::BinaryFileDecryptingError => String::from("File set for decoding is corrupted or Secret key is wrong"),
            LibraryErrors::BinaryFileReadError => String::from("Could not open/read the file"),
            LibraryErrors::BinaryFileWriteError => String::from("Could not create encrypted file. Make sure program has rights to writing"),
        };

        EncryptorError {
            code,
            error_msg:error_type
        }
    }
}


impl std::fmt::Display for EncryptorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Encryptor error occured! Details → {}", self.error_msg)
    }
}
