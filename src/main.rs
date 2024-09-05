slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    //let ui_login = Login::new()?;


    //Working encryption code
    // use simple_crypt::encrypt;
    // let encrypted_data = encrypt(b"Super secret text", b"any_password").expect("Failed to encrypt");
    // println!("{:?}", encrypted_data);

    // This is the result of the encrypt() line above
    // let encrypted_data: Vec<u8> = vec![33, 0, 0, 0, 0, 0, 0, 0, 79, 145, 223, 115, 208, 22, 215, 55, 205, 42, 39, 209, 84, 141, 239, 212, 90, 140, 126, 241, 169, 211, 23, 99, 222, 80, 10, 232, 136, 62, 107, 220, 37, 93, 146, 172, 156, 203, 8, 69, 137, 113, 90, 151, 111, 125, 83, 114, 57, 30, 164, 115, 22, 110, 176, 211, 35, 200, 97, 206, 15, 53, 140, 183, 10, 187, 90, 146, 130, 49, 63, 14, 79, 120, 208, 86, 72];

    // use simple_crypt::decrypt;
    // let data = decrypt(&encrypted_data, b"USarmy3360!!").expect("Failed to decrypt");
    // let text = String::from_utf8_lossy(&data);
    // println!("{}", text);


    

    //Starter code
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set(false);
    //     }
    // });

    ui.run()
}
