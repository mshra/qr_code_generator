use image::Luma;
use qrcode::QrCode;
use std::process::Command;

fn main() {
    let link = QrCode::new(b"https://twitter.com/home").unwrap();

    let image = link.render::<Luma<u8>>().build();

    image.save("./output/qrcode.png").unwrap();

    Command::new("open")
        .arg("./output/qrcode.png")
        .spawn()
        .expect("failed to open qr code.");
}
