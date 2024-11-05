use chrono::{Local};
use fast_qr::qr::QRBuilder;
use fast_qr::QRCode;
use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};

fn genqr() -> QRCode{
    let now = Local::now();
    let bytes = now.time().to_string();
    let qrcode = QRBuilder::new(bytes).build().expect("Failed to generate QR CODE");
    return qrcode;
}

pub fn gensvg(){
    let qr =genqr();
    let _svg = SvgBuilder::default()
        .shape(Shape::RoundedSquare)
        .to_file(&qr, "out.svg");
}