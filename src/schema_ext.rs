use diesel_derive_enum::*;

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Item_walkable"]
pub enum ItemWalkable {
    No,
    Yes,
    Swim,
}

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Item_type"]
pub enum ItemType {
    Club,
    Effect,
    Floor,
    Pet,
    Wall,
}

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Item_stacking_behaviour"]
pub enum ItemStackingBehaviour {
    Disable,
    Ignore,
    Initiator,
    Normal,
    Terminator,
}

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Item_behaviour"]
pub enum ItemBehaviour {
    Alert,
    Autoswitch,
    Bed,
    Dice,
    Dispenser,
    Effectgenerator,
    Exchange,
    Fireworks,
    Floor,
    Football,
    Gate,
    Habbowheel,
    HoloDice,
    Landscape,
    Loveshuffler,
    Moodlight,
    Musicdisk,
    Onewaygate,
    Pet,
    Prizetrophy,
    Rental,
    Roller,
    Scoreboard,
    Seat,
    Spinningbottle,
    Static,
    Stickynote,
    Stickypole,
    Switch,
    Teleporter,
    Traxplayer,
    Wallpaper,
}

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Navigator_official_display_type"]
pub enum NavigatorOfficialDisplayType {
    Banner,
    Details,
}

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Navigator_official_banner_type"]
pub enum NavigatorOfficialBannerType {
    Internal,
    External,
}

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Room_type"]
pub enum RoomType {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq, DbEnum)]
#[DieselType = "Room_state"]
pub enum RoomState {
    Open,
    Doorbell,
    Password,
}
