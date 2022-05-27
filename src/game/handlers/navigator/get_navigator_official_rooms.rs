use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{
        navigator::NavigatorOfficial, room::Room, session::Session, session_manager::SessionManager,
    },
    message::outgoing::NAVIGATOR_OFFICIAL_ROOMS,
    schema_ext::{NavigatorOfficialBannerType, NavigatorOfficialDisplayType, RoomType},
};

pub async fn get_navigator_official_rooms(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(8192);

    result.extend_from_slice(encoding::base64::encode(NAVIGATOR_OFFICIAL_ROOMS, 2).as_slice());

    let official_items = NavigatorOfficial::fetch_all(session.get_pool()).await;

    result.extend_from_slice(encoding::wire::encode_i32(official_items.len() as i32).as_slice());

    for item in official_items.iter() {
        println!("{:?}", item);

        if item.parent_id.is_none() {
            let room = item.fetch_room(session.get_pool()).await;

            write_item(item, room, &mut result);
        }

        if item.is_category {
            for child in official_items
                .iter()
                .filter(|&i| i.parent_id.is_some() && i.parent_id == Some(item.id))
            {
                let room = child.fetch_room(session.get_pool()).await;

                write_item(item, room, &mut result);
            }
        }
    }

    result.extend_from_slice(&[1]);

    println!("get_nav_officials");
    Some(result)
}

pub fn write_item(item: &NavigatorOfficial, room: Option<Room>, bytes: &mut BytesMut) {
    let current_users = room.as_ref().map(|r| r.current_users).unwrap_or(0);

    let room_type = room.as_ref().map_or(RoomType::Private, |r| r.type_.clone());

    let display_type = match item.display {
        NavigatorOfficialDisplayType::Details => 1,
        NavigatorOfficialDisplayType::Banner => 0,
    };

    let internal_banner_src = match item.banner_type {
        NavigatorOfficialBannerType::Internal => item.banner_source.as_str(),
        NavigatorOfficialBannerType::External => "",
    };

    let external_banner_src = match item.banner_type {
        NavigatorOfficialBannerType::Internal => "",
        NavigatorOfficialBannerType::External => item.banner_source.as_str(),
    };

    let mut item_type = 3;

    if item.is_category {
        item_type = 4;
    } else if RoomType::Private == room_type {
        item_type = 2;
    }

    // ID
    bytes.extend_from_slice(encoding::wire::encode_i32(item.id).as_slice());

    // Name
    bytes.extend_from_slice(item.name.as_bytes());
    bytes.extend_from_slice(&[2]);

    // Description
    bytes.extend_from_slice(item.description.as_bytes());
    bytes.extend_from_slice(&[2]);

    // Display Type
    println!("display_type: {:?}, enum {:?}", display_type, item.display);
    bytes.extend_from_slice(encoding::wire::encode_i32(display_type).as_slice());

    // Banner Label
    bytes.extend_from_slice(item.banner_label.as_bytes());
    bytes.extend_from_slice(&[2]);

    // External Banner Source
    bytes.extend_from_slice(external_banner_src.as_bytes());
    bytes.extend_from_slice(&[2]);

    // Parent Id
    bytes.extend_from_slice(
        encoding::wire::encode_i32(item.parent_id.as_ref().map_or(0, |i| *i)).as_slice(),
    );

    // Current Users
    bytes.extend_from_slice(encoding::wire::encode_i32(current_users).as_slice());

    // Item Type
    bytes.extend_from_slice(encoding::wire::encode_i32(item_type).as_slice());

    if item.is_category {
        // Should Auto Expand?
        bytes.extend_from_slice(encoding::wire::encode_bool(item.is_expanded).as_slice());
    }

    println!("{} is_category {}", item.name.clone(), item.is_category);

    if !item.is_category && room.is_some() {
        let room = room.unwrap();

        match room.type_ {
            RoomType::Public => {
                // Internal Banner Source
                bytes.extend_from_slice(internal_banner_src.as_bytes());
                bytes.extend_from_slice(&[2]);

                // ???
                bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

                // Room Part
                bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

                // SWF (Public room Only)
                bytes.extend_from_slice(room.swfs.join(",").as_bytes());
                bytes.extend_from_slice(&[2]);

                // Max Users
                bytes.extend_from_slice(encoding::wire::encode_i32(room.max_users).as_slice());

                // Room ID
                bytes.extend_from_slice(encoding::wire::encode_i32(room.id).as_slice());
            }
            RoomType::Private => {
                bytes.extend_from_slice(
                    NavigatorOfficial::serialize_private_room(&room, false)
                        .to_vec()
                        .as_slice(),
                );
            }
        }
    }
}
