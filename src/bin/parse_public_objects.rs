use std::{
    env,
    io::{self},
};

use bytes::{Buf, BytesMut};
use hebbo::encoding;

#[derive(Debug)]
struct StaticFurni {
    id: String,
    name: String,
    position_x: i32,
    position_y: i32,
    position_z: i32,
    rotation: i32,
    is_seat: i32,
}

fn main() -> io::Result<()> {
    let static_str = env::args().nth(1).map_or("".to_string(), |f| f);
    let model_id = env::args()
        .nth(2)
        .map_or(0, |f| f.parse::<i32>().unwrap_or(0));

    let static_str = static_str.replace("{2}", "\x02");

    let mut static_furni: Vec<StaticFurni> = Vec::with_capacity(500);

    let mut bytes = BytesMut::from(static_str.as_bytes());

    let (_len, bytes_read) = encoding::wire::decode_i32(bytes.as_ref());

    bytes.advance(bytes_read);

    // println!("{:?}", &bytes);
    // println!("{} elements to parse", len);

    let mut id = BytesMut::with_capacity(10);
    let mut name = BytesMut::with_capacity(20);

    while !bytes.is_empty() {
        let (is_seat, bytes_read) = encoding::wire::decode_i32(bytes.as_ref());
        bytes.advance(bytes_read);

        while !id.ends_with(&[2]) {
            match bytes.get(0) {
                Some(byte) => id.extend_from_slice(&[*byte]),
                None => break,
            }

            bytes.advance(1);
        }

        while !name.ends_with(&[2]) {
            match bytes.get(0) {
                Some(byte) => name.extend_from_slice(&[*byte]),
                None => break,
            }

            bytes.advance(1);
        }

        let (position_x, bytes_read) = encoding::wire::decode_i32(bytes.as_ref());
        bytes.advance(bytes_read);

        let (position_y, bytes_read) = encoding::wire::decode_i32(bytes.as_ref());
        bytes.advance(bytes_read);

        let (position_z, bytes_read) = encoding::wire::decode_i32(bytes.as_ref());
        bytes.advance(bytes_read);

        let (rotation, bytes_read) = encoding::wire::decode_i32(bytes.as_ref());
        bytes.advance(bytes_read);

        static_furni.push(StaticFurni {
            id: id
                .iter()
                .filter(|&b| *b != 2)
                .map(|b| *b as char)
                .collect::<String>(),
            name: name
                .iter()
                .filter(|&b| *b != 2)
                .map(|b| *b as char)
                .collect::<String>(),
            position_x,
            position_y,
            position_z,
            rotation,
            is_seat,
        });

        id.clear();
        name.clear();
    }

    for furni in &static_furni {
        println!(
            r#" INSERT INTO "room_model_static_items" ("name", "position_x", "position_y", "position_z", "rotation", "is_seat", "model_id") VALUES ('{}', {}, {}, {}, {}, {}, {}); "#,
            furni.name,
            furni.position_x,
            furni.position_y,
            furni.position_z,
            furni.rotation,
            furni.is_seat == 1,
            model_id,
        );
    }

    Ok(())
}
