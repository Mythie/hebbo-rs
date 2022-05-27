use bytes::BytesMut;

use crate::{encoding, game::room::RoomModelStaticItem, message::outgoing::ROOM_STATIC_ITEMS};

pub async fn make_room_model_static_items(
    room_model_static_items: Vec<RoomModelStaticItem>,
) -> BytesMut {
    let mut bytes = BytesMut::with_capacity(1024);

    bytes.extend_from_slice(encoding::base64::encode(ROOM_STATIC_ITEMS, 2).as_slice());

    // Count
    bytes.extend_from_slice(
        encoding::wire::encode_i32(room_model_static_items.len() as i32).as_slice(),
    );

    // println!("static_items header: {:?}", &bytes);

    for item in room_model_static_items {
        // ???
        bytes.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

        // ???
        bytes.extend_from_slice(encoding::base64::encode(item.id, 2).as_slice());
        bytes.extend_from_slice(&[2]);

        // ???
        bytes.extend_from_slice(item.name.as_bytes());
        bytes.extend_from_slice(&[2]);

        bytes.extend_from_slice(encoding::wire::encode_i32(item.position_x).as_slice());

        bytes.extend_from_slice(encoding::wire::encode_i32(item.position_y).as_slice());

        bytes.extend_from_slice(encoding::wire::encode_i32(item.position_z).as_slice());

        bytes.extend_from_slice(encoding::wire::encode_i32(item.rotation).as_slice());

        bytes.extend_from_slice(encoding::wire::encode_bool(item.is_seat).as_slice());
    }

    // bytes.extend_from_slice("SOHa016\x02crl_lamp\x02PDHHHHy017\x02crl_sofa2c\x02QDHHPAHw018\x02crl_sofa2b\x02RDHHPAHv019\x02crl_sofa2a\x02SDHHPAHa020\x02crl_lamp\x02PEHHHHb116\x02crl_chair\x02PDIHJHa27\x02crl_lamp\x02SAJHHHa211\x02crl_lamp\x02SBJHHHb216\x02crl_chair\x02PDJHJHc35\x02crl_pillar\x02QAKHHHb37\x02crl_chair\x02SAKHJHu39\x02crl_table1b\x02QBKHHHs311\x02crl_sofa1c\x02SBKHRAHb316\x02crl_chair\x02PDKHJHA319\x02crl_table2b\x02SDKHHHz320\x02crl_table2a\x02PEKHHHa40\x02crl_lamp\x02HPAHHHy41\x02crl_sofa2c\x02IPAHPAHw42\x02crl_sofa2b\x02JPAHPAHv43\x02crl_sofa2a\x02KPAHPAHa44\x02crl_lamp\x02PAPAHHHt49\x02crl_table1a\x02QBPAHHHr411\x02crl_sofa1b\x02SBPAHRAHh415\x02crl_wall2a\x02SCPAHHHa416\x02crl_lamp\x02PDPAHHHb50\x02crl_chair\x02HQAHJHb57\x02crl_chair\x02SAQAHJHq511\x02crl_sofa1a\x02SBQAHRAHA62\x02crl_table2b\x02JRAHHHz63\x02crl_table2a\x02KRAHHHa611\x02crl_lamp\x02SBRAHHHb70\x02crl_chair\x02HSAHJHa80\x02crl_lamp\x02HPBHHHD81\x02crl_sofa3c\x02IPBHHHC82\x02crl_sofa3b\x02JPBHHHB83\x02crl_sofa3a\x02KPBHHHa84\x02crl_lamp\x02PAPBHHHo819\x02crl_barchair2\x02SDPBHHHp820\x02crl_tablebar\x02PEPBHHHo821\x02crl_barchair2\x02QEPBHHHE95\x02crl_pillar2\x02QAQBHHHc99\x02crl_pillar\x02QBQBHHHP158\x02crl_desk1a\x02PBSCHHHO159\x02crl_deski\x02QBSCHHHN1510\x02crl_deskh\x02RBSCHHHM1610\x02crl_deskg\x02RBPDHHHL1710\x02crl_deskf\x02RBQDHHHK1810\x02crl_deske\x02RBRDHHHK1910\x02crl_deske\x02RBSDHHHK2010\x02crl_deske\x02RBPEHHHK2110\x02crl_deske\x02RBQEHHHK2210\x02crl_deske\x02RBREHHHK2310\x02crl_deske\x02RBSEHHHG247\x02crl_wallb\x02SAPFHHHK2410\x02crl_deske\x02RBPFHHHF257\x02crl_walla\x02SAQFHHHH258\x02crl_desk1b\x02PBQFHHHI259\x02crl_desk1c\x02QBQFHHHJ2510\x02crl_desk1d\x02RBQFHHHd2712\x02crl_lamp2\x02PCSFHHHf2713\x02crl_cabinet2\x02QCSFHHHe2714\x02crl_cabinet1\x02RCSFHHHd2715\x02crl_lamp2\x02SCSFHH".as_bytes());

    bytes.extend_from_slice(&[1]);

    bytes
}
