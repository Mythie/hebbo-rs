#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    achievements (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        level -> Int4,
        reward_pixels -> Int4,
        reward_points -> Int4,
        progress_needed -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    items (id) {
        sprite_id -> Varchar,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Item_type,
        size_x -> Int4,
        size_y -> Int4,
        size_z -> Int4,
        allow_recycling -> Bool,
        allow_trading -> Bool,
        allow_selling -> Bool,
        allow_gifting -> Bool,
        allow_inventory_stacking -> Bool,
        walkable -> Item_walkable,
        room_limit -> Int4,
        behaviour_states -> Int4,
        id -> Int4,
        behaviour -> Item_behaviour,
        stacking_behaviour -> Item_stacking_behaviour,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    navigator_categories (id) {
        id -> Int4,
        title -> Varchar,
        order -> Int4,
        is_visible -> Bool,
        is_enabled -> Bool,
        allows_trading -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    navigator_official (id) {
        id -> Int4,
        enabled -> Bool,
        name -> Varchar,
        description -> Varchar,
        order -> Int4,
        display -> Navigator_official_display_type,
        banner_type -> Navigator_official_banner_type,
        banner_source -> Varchar,
        banner_label -> Varchar,
        is_expanded -> Bool,
        is_visible -> Bool,
        is_enabled -> Bool,
        is_category -> Bool,
        allows_trading -> Bool,
        room_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    room_items (id) {
        position_x -> Int4,
        position_y -> Int4,
        extra_data -> Nullable<Jsonb>,
        room_id -> Int4,
        item_id -> Int4,
        id -> Int4,
        position_wall -> Float8,
        position_z -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    room_models (id) {
        id -> Int4,
        name -> Varchar,
        enabled -> Bool,
        #[sql_name = "type"]
        type_ -> Room_type,
        heightmap -> Array<Text>,
        door_x -> Int4,
        door_y -> Int4,
        door_z -> Int4,
        door_direction -> Int4,
        required_rank -> Int4,
        max_users -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    room_model_static_items (id) {
        id -> Int4,
        name -> Varchar,
        position_x -> Int4,
        position_y -> Int4,
        position_z -> Int4,
        rotation -> Int4,
        is_seat -> Bool,
        model_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    rooms (id) {
        id -> Int4,
        state -> Room_state,
        #[sql_name = "type"]
        type_ -> Room_type,
        name -> Varchar,
        description -> Varchar,
        password -> Varchar,
        current_users -> Int4,
        max_users -> Int4,
        score -> Int4,
        tags -> Array<Text>,
        swfs -> Array<Text>,
        icon_foreground -> Int4,
        icon_background -> Int4,
        wallpaper -> Varchar,
        landscape -> Varchar,
        floor -> Varchar,
        wall_thickness -> Int4,
        floor_thickness -> Int4,
        allow_pets -> Bool,
        allow_pet_feeding -> Bool,
        allow_blocking -> Bool,
        hidden_walls -> Bool,
        navigator_category_id -> Nullable<Int4>,
        owner_id -> Nullable<Int4>,
        model_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    user_achievements (id) {
        id -> Int4,
        level -> Int4,
        progress -> Int4,
        achievement_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    user_friends (id) {
        id -> Int4,
        accepted -> Bool,
        user_id -> Int4,
        friend_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    user_ignores (id) {
        id -> Int4,
        user_id -> Int4,
        ignore_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    user_items (id) {
        extra_data -> Nullable<Jsonb>,
        user_id -> Int4,
        item_id -> Int4,
        id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        rank -> Int2,
        gender -> Int2,
        figure -> Varchar,
        credits -> Int4,
        pixels -> Int4,
        home_room -> Int4,
        sso_ticket -> Varchar,
        last_ip_addr -> Varchar,
        last_online -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::schema_ext::*;

    user_subscriptions (id) {
        id -> Int4,
        rank -> Int2,
        acquired_on -> Timestamptz,
        expires_at -> Timestamptz,
        user_id -> Int4,
    }
}

joinable!(navigator_official -> rooms (room_id));
joinable!(room_items -> items (item_id));
joinable!(room_items -> rooms (room_id));
joinable!(room_model_static_items -> room_models (model_id));
joinable!(rooms -> navigator_categories (navigator_category_id));
joinable!(rooms -> room_models (model_id));
joinable!(rooms -> users (owner_id));
joinable!(user_achievements -> achievements (achievement_id));
joinable!(user_achievements -> users (user_id));
joinable!(user_items -> items (item_id));
joinable!(user_items -> users (user_id));
joinable!(user_subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    achievements,
    items,
    navigator_categories,
    navigator_official,
    room_items,
    room_models,
    room_model_static_items,
    rooms,
    user_achievements,
    user_friends,
    user_ignores,
    user_items,
    users,
    user_subscriptions,
);
