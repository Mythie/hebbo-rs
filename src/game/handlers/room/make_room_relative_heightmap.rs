use bytes::BytesMut;

use crate::{
    encoding,
    game::room::{Room, RoomModel},
    schema_ext::{RoomType},
    message::outgoing::ROOM_RELATIVE_HEIGHTMAP,
};

pub async fn make_room_relative_heightmap(_room: &Room, room_model: &RoomModel) -> BytesMut {
    let mut bytes = BytesMut::with_capacity(512);

    bytes.extend_from_slice(encoding::base64::encode(ROOM_RELATIVE_HEIGHTMAP, 2).as_slice());

    // Initialise the line here to so we aren't constantly alocating things on the heap
    let mut line: Vec<u8> = Vec::with_capacity(100);

    for (y_index, y_tile) in room_model.heightmap.iter().enumerate() {
        for (x_index, x_tile) in y_tile.chars().enumerate() {
            if room_model.type_ == RoomType::Private
                && y_index as i32 == room_model.door_x
                && x_index as i32 == room_model.door_y
            {
                println!("Adding door");

                line.push(
                    room_model
                        .door_z
                        .to_string()
                        .chars()
                        .next()
                        .map_or(0_u8, |c| c as u8),
                );
            } else {
                line.push(x_tile as u8);
            }
        }

        line.push(13);

        bytes.extend_from_slice(line.as_slice());

        line.clear();
    }

    bytes.extend_from_slice(&[2]);

    bytes.extend_from_slice(&[1]);

    bytes
}
