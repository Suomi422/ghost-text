use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::fs::{File, OpenOptions, self};
use rand::{distributions::Alphanumeric, Rng, rngs::OsRng};

use dirs::home_dir;
use bytebuffer::ByteBuffer;
use base64::{Engine as _, engine::general_purpose};
use aes::cipher::{KeyIvInit, block_padding::Pkcs7, BlockEncryptMut, BlockDecryptMut};

use crate::errors::EncryptorError;


type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
type LibraryErrors = errors::LibraryErrors;

mod errors;

// Caution: Depending on the value this can set permissions for whole filesystem
// Change only if your are 100% sure what you are doing 
static KEY_FILE: &str = ".passkey";



#[derive(PartialEq, Clone)]
pub enum OperationType {
    Encrypt,
    Decrypt,
    None
}

#[derive(Clone,)]
pub struct Secret {
    content: String,
    secret_key: String,
    operation_type: OperationType
}


impl Secret {
    pub fn init(mut key: String, op_type: OperationType) -> Self {
        if key.len() < 32 {
            let random_size = 32 - key.len();
            let key_to_append = Self::random_generator(random_size);
            key = key + &key_to_append;

        } else if key.len() > 32 {
            key = key[..32].to_string();
        }

        Self {
            content: String::new(),
            secret_key: key,
            operation_type: op_type
        }
    }


    pub fn generate(&mut self, content: String) -> Result<String, EncryptorError> {
        self.content = content;
        if self.operation_type == OperationType::Encrypt {
            self.encrypt()
        } else if self.operation_type == OperationType::Decrypt {
            self.decrypt()
        } else {
            Err(EncryptorError::new(LibraryErrors::NoOperationTypeError))
        }
    }


    fn encrypt(&self) -> Result<String, EncryptorError> {
        let iv_str = Self::random_generator(16);
        let iv = iv_str.as_bytes();
        let cipher = Aes256CbcEnc::new(self.secret_key
            .as_bytes()
            .into(), iv.into())
            .encrypt_padded_vec_mut::<Pkcs7>(self.content.as_bytes());

        let mut buffer = ByteBuffer::from_bytes(iv);
        buffer.write_bytes(&cipher);
        Ok(general_purpose::STANDARD.encode(buffer.as_bytes()))
    }


    fn decrypt(&self) -> Result<String, EncryptorError> {
        let bytes = general_purpose::STANDARD
            .decode(self.content.clone())
            .map_err(|_| EncryptorError::new(LibraryErrors::UTF8Error))?;
        
        if bytes.is_empty() || bytes.len() < 16 {
            return Err(EncryptorError::new(LibraryErrors::ValueError))
        }

        let res = Aes256CbcDec::new(self.secret_key.as_bytes().into(), bytes[0..16].into())
            .decrypt_padded_vec_mut::<Pkcs7>(&bytes[16..]);

        let res = res.map_err(|_| EncryptorError::new(LibraryErrors::KeyError))?;
        return Ok(String::from_utf8(res).map_err(|_| EncryptorError::new(LibraryErrors::UTF8Error))?)
    }


    pub fn get_key(&self) -> &str {
        &self.secret_key
    }


    fn random_generator(random_size: usize) -> String {
        let mut rng = OsRng;
        let random_string: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(random_size)
            .map(char::from)
            .collect();
        random_string
    }
}



// Utilities
pub fn save_passkey_to_file(passkey: String) -> Result<(), EncryptorError> {
    if let Some(home_path) = home_dir() {
        // In a case of an accident to not blocking whole file-system.
        if KEY_FILE.len() > 3 {
            let file_path = home_path.join(KEY_FILE);
            let file = File::create(&file_path);

            let mut file = file.map_err(|_| EncryptorError::new(LibraryErrors::PermissionError))?;
            file.write_all(passkey.as_bytes())
                .map_err(|_| EncryptorError::new(LibraryErrors::FileError))?;

            match file.sync_all() {
                Ok(_) => {
                    let mut perms = fs::metadata(&file_path).unwrap()
                    .permissions();
                    perms.set_mode(0o600);
                    
                    fs::set_permissions(&file_path, perms)
                        .map_err(|_| EncryptorError::new(LibraryErrors::PermissionError))?;
                },
                Err(_) =>  return Err(EncryptorError::new(LibraryErrors::FileError))
            }
        } else {
            return Err(EncryptorError::new(LibraryErrors::FileNameError))
        }
    } else {
        return Err(EncryptorError::new(LibraryErrors::FileError))
    }
    Ok(())
}


pub fn read_passkey_from_file() -> Result<String, EncryptorError> {
    if let Some(home_path) = home_dir() {
        let file_path = home_path.join(KEY_FILE);
        if file_path.exists() {
            let mut passkey = String::new();

            let file = OpenOptions::new().read(true).open(&file_path);
            let mut file = file.map_err(|_| EncryptorError::new(LibraryErrors::PermissionError))?;
            file.read_to_string(&mut passkey)
                .map_err(|_| EncryptorError::new(LibraryErrors::PermissionError))?;
            return Ok(passkey)
        }
    }
    Err(EncryptorError::new(LibraryErrors::PermissionError))
}
