use std::{cell::RefCell, process::exit, rc::Rc};
use ghost_text::{OperationType, Secret};
use native_dialog::{FileDialog, MessageDialog, MessageType};


slint::include_modules!();

static DEFAULT_PASS: &str = "01234567012345670123456701234567";
static FATAL_MESSAGE: &str = "FATAL: User interface was not found";
static FIRST_TIME_MESSAGE: &str = "WARNING: Wasn't able to read Security Key file. If using first time please change Security Key in - Enter Key - menu. Using default Security key for now.";
static PASSKEY_FILE_MESSAGE: &str = "WARNING: Wasn't able to save Security Key! Please make sure software has access to home folder. Using default Security key for now.";
static PATH_MESSAGE: &str = "Warning: Wasn't able to get file path";
static FILE_ENCRYPT_EXTENSION: &str = "enc";


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    let ui_handle2 = ui_handle.clone();
    let ui_handle3 = ui_handle.clone();
    let ui_handle4 = ui_handle.clone();
    let ui_handle5 = ui_handle.clone();

    let ui = ui_handle.unwrap();
    ui.set_main_error_field(String::from("").into());

    let key = ghost_text::read_passkey_from_file();
    let key = match key {
        Ok(key) => key,
        Err(_) => { 
            ui.set_main_error_field(FIRST_TIME_MESSAGE.into());
            
            let password_saving_result = ghost_text::save_passkey_to_file(String::from(DEFAULT_PASS));

            let _ = match password_saving_result {
                Ok(()) => (),
                Err(_) =>  ui.set_main_error_field(PASSKEY_FILE_MESSAGE.into()),
            };
            
            String::from(DEFAULT_PASS)
        }
    };
    ui.set_key_value(key.clone().into());

    let shared_key = Rc::new(RefCell::new(key));
    let shared_key_for_encode = Rc::clone(&shared_key);
    let shared_key_for_decode = Rc::clone(&shared_key);
    let shared_key_for_set_key = Rc::clone(&shared_key);

    let shared_key_for_encode_binary = Rc::clone(&shared_key);
    let shared_key_for_decode_binary = Rc::clone(&shared_key);


    // Encrypt
    ui.on_encode(move |string| {
        let mut encrypter = Secret::init(shared_key_for_encode.borrow_mut().to_string(), OperationType::Encrypt);
        let encoded = encrypter.generate(string.to_string());

        match encoded {
            Ok(val) => {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_cloni(val.into());
                } else {
                    panic!("{}", FATAL_MESSAGE);
                }
            },
            Err(error) => if let Some(ui) = ui_handle.upgrade() {
                let error_msg = error.error_msg;
                ui.set_cloni(error_msg.into());
            } else {
                panic!("{}", FATAL_MESSAGE);
            }
        };
    });


    // Decrypt
    ui.on_decode(move |string| {
        let mut encrypter = Secret::init(shared_key_for_decode.borrow().clone(), OperationType::Decrypt);
        let decoded = encrypter.generate(string.to_string());

        if let Some(ui) = ui_handle2.upgrade() {
            match decoded {
                Ok(decoded_string) => ui.set_cloni_2(decoded_string.into()),
                Err(error) => ui.set_cloni_2(format!("[ ERR ] Decryptor Error: {}", error.error_msg).into()),
            }
        } else {
            panic!("{}", FATAL_MESSAGE);
        }
    });


    // Secret Key Set
    ui.on_set_key_msg(move |string| {
        let mut key = shared_key_for_set_key.borrow_mut();
            *key = string.to_string().clone();

            let encrypter = Secret::init(key.clone(), OperationType::None);
            *key = encrypter.get_key().to_string();

            let result = ghost_text::save_passkey_to_file(key.clone());

            if let Some(ui) = ui_handle3.upgrade() {
                if let Err(error) = result {
                    ui.set_key_message("Error occurred: ".into());
                    ui.set_key_value(error.error_msg.into());
                } else {
                    ui.set_key_message("DONE! Your key is:".into());
                    ui.set_key_value(key.clone().into());
                    ui.set_main_error_field("".into());
                }
            } else {
                panic!("{}", FATAL_MESSAGE);
            }
    });


    // File encryption
    ui.on_encrypt_file(move || {
        let unwrapped_ui = match ui_handle4.upgrade() {
            Some(ui) => ui,
            None => panic!("{}", FATAL_MESSAGE)
        };

        let path = FileDialog::new()
        .set_location("~/")
        .set_filename("")
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .add_filter("PNG Image", &["png"])
        .show_open_single_file()
        .unwrap();

        let mut path = match path {
            Some(path) => path,
            None => {
                unwrapped_ui.set_cloni(PATH_MESSAGE.into());
                return
            },
        };

        let confirmation_dialog = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Do you want to encrypt this file?")
            .set_text(&format!("{:#?}", path))
            .show_confirm();

        if confirmation_dialog.is_err() {
            panic!("{}", FATAL_MESSAGE)
            
        }

        let file_bytes = ghost_text::read_file_to_bytes(&path);
        let mut encrypter = Secret::init(shared_key_for_encode_binary.borrow().clone(), OperationType::Encrypt);
        let encrypted_bytes = encrypter.generate_bytes(file_bytes.unwrap());
        path.set_extension(format!("{}.{}", path.extension().unwrap_or_default().to_str().unwrap_or(""), FILE_ENCRYPT_EXTENSION));
        match encrypted_bytes {
            Ok(bytes) => {
                let writing_result = ghost_text::write_bytes_to_file(&path, &bytes);

                match writing_result {
                    Ok(_) => unwrapped_ui.set_cloni("File encrypted sucesfully".into()),
                    Err(error) => unwrapped_ui.set_cloni(error.error_msg.into()),
                }
            },
            Err(error) => unwrapped_ui.set_cloni(error.error_msg.into()),
        }
    });


    // File decryption
    ui.on_decrypt_file(move || {
        let unwrapped_ui = match ui_handle5.upgrade() {
            Some(ui) => ui,
            None => panic!("{}", FATAL_MESSAGE)
        };

        let path = FileDialog::new()
        .set_location("~/")
        .set_filename("")
        .add_filter("Encrypted Image", &[FILE_ENCRYPT_EXTENSION])
        .show_open_single_file()
        .unwrap();

        let mut path = match path {
            Some(path) => path,
            None => {
                unwrapped_ui.set_cloni_2(PATH_MESSAGE.into());
                return
            },
        };

        let confirmation_dialog = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Do you want to decrypt this file?")
            .set_text(&format!("{:#?}", path))
            .show_confirm();

        if confirmation_dialog.is_err() {
            panic!("{}", FATAL_MESSAGE)
            
        }

        let file_bytes = ghost_text::read_file_to_bytes(&path);
        let mut decrypter = Secret::init(shared_key_for_decode_binary.borrow().clone(), OperationType::Decrypt);
        let decrypted_bytes = decrypter.generate_bytes(file_bytes.unwrap());

        path.set_extension(format!("{}.{}", path.extension().unwrap_or_default().to_str().unwrap_or(""), "dec"));
        match decrypted_bytes {
            Ok(bytes) => {
                let writing_result = ghost_text::write_bytes_to_file(&path, &bytes);

                match writing_result {
                    Ok(_) => unwrapped_ui.set_cloni_2("File decrypted sucesfully".into()),
                    Err(error) => unwrapped_ui.set_cloni_2(error.error_msg.into()),
                }
            },
            Err(error) => unwrapped_ui.set_cloni_2(error.error_msg.into()),
        }
    });


    ui.on_exit(move || exit(0));

    ui.run() 
}
