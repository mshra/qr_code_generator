use image::Luma;
use qrcode::QrCode;
use std::io;
use std::process::Command;

fn main() {
    println!("Welcome to QR code generator!");

    loop {
        println!("Enter url to generate QR code(or type quit):");

        let url = get_link();

        if url == "quit\n" {
            println!("program exited succesfully!");
            std::process::exit(0);
        }

        save_qr_code(url);
        open_qr_code();
        println!("QR code generated and opened!\n-------");
    }
}

fn get_link() -> String {
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("failed to read url!");
    url
}

fn save_qr_code(url: String) {
    let code = QrCode::new(url).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("./output/output.png").unwrap();
}

fn open_qr_code() {
    Command::new("open")
        .arg("output/output.png")
        .spawn()
        .expect("failed to open qr code.");
}
