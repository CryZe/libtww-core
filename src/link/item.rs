macro_rules! items {
    ($($name:ident: $id:expr)*) => {
        $(
            pub const $name: u8 = $id;
        )*
    }
}

items! {
    HEART_DROP: 0x00
    GREEN_RUPEE_DROP: 0x01
    BLUE_RUPEE_DROP: 0x02
    YELLOW_RUPEE_DROP: 0x03
    RED_RUPEE_DROP: 0x04
    PURPLE_RUPEE_DROP: 0x05
    ORANGE_RUPEE_DROP: 0x06
    HEART_PIECE: 0x07
    HEART_CONTAINER: 0x08
    SMALL_MAGIC_DROP: 0x09
    LARGE_MAGIC_DROP: 0x0A
    BOMB_DROP: 0x0B
    SILVER_RUPEE_DROP: 0x0F
    SINGLE_ARROW_DROP: 0x10
    DOUBLE_ARROW_DROP: 0x11
    TRIPLE_ARROW_DROP: 0x12
    KEY_DROP: 0x15
    FAIRY_DROP: 0x16
    TRIPLE_HEART_DROP: 0x1E
    JOY_PENDANT: 0x1F
    TELESCOPE: 0x20
    TINGLE_TUNER: 0x21
    WIND_WAKER: 0x22
    PICTO_BOX: 0x23
    SPOILS_BAG: 0x24
    GRAPPLING_HOOK: 0x25
    DELUXE_PICTO_BOX: 0x26
    BOW: 0x27
    POWER_BRACELETS: 0x28
    IRON_BOOTS: 0x29
    MAGIC_ARMOR: 0x2A
    WATER_BOOTS: 0x2B
    BAIT_BAG: 0x2C
    BOOMERANG: 0x2D
    HOOKSHOT: 0x2F
    DELIVERY_BAG: 0x30
    BOMBS: 0x31
    TUNIC: 0x32
    SKULL_HAMMER: 0x33
    DEKU_LEAF: 0x34
    BOW_WITH_FIRE_AND_ICE_ARROWS: 0x35
    BOW_WITH_LIGHT_ARROWS: 0x36
    HEROS_SWORD: 0x38
    UNCHARGED_MASTER_SWORD: 0x39
    HALF_CHARGED_MASTER_SWORD: 0x3A
    HEROS_SHIELD: 0x3B
    MIRROR_SHIELD: 0x3C
    FULLY_CHARGED_MASTER_SWORD: 0x3E
    PIRATES_CHARM: 0x42
    HEROS_CHARM: 0x43
    SKULL_NECKLACE: 0x45
    BOKO_BABA_SEED: 0x46
    GOLDEN_FEATHER: 0x47
    KNIGHTS_CREST: 0x48
    RED_CHU_JELLY: 0x49
    GREEN_CHU_JELLY: 0x4A
    BLUE_CHU_JELLY: 0x4B
    MAP: 0x4C
    COMPASS: 0x4D
    BIG_KEY: 0x4E
    EMPTY_BOTTLE: 0x50
    RED_POTION: 0x51
    GREEN_POTION: 0x52
    BLUE_POTION: 0x53
    ELIXIR_SOUP_HALF: 0x54
    ELIXIR_SOUP: 0x55
    WATER: 0x56
    FAIRY: 0x57
    FOREST_FIREFLY: 0x58
    FOREST_WATER: 0x59
    TRIFORCE_PIECE_1: 0x61
    TRIFORCE_PIECE_2: 0x62
    TRIFORCE_PIECE_3: 0x63
    TRIFORCE_PIECE_4: 0x64
    TRIFORCE_PIECE_5: 0x65
    TRIFORCE_PIECE_6: 0x66
    TRIFORCE_PIECE_7: 0x67
    TRIFORCE_PIECE_8: 0x68
    NAYRUS_PEARL: 0x69
    DINS_PEARL: 0x6A
    FARORES_PEARL: 0x6B
    SAIL: 0x78
    TRIFORCE_CHART: 0x79
    ALL_PURPOSE_BAIT: 0x82
    HYOI_PEAR: 0x83
    TOWN_FLOWER: 0x8C
    SEA_FLOWER: 0x8D
    EXOTIC_FLOWER: 0x8E
    HEROS_FLAG: 0x8F
    BIG_CATCH_FLAG: 0x90
    BIG_SALE_FLAG: 0x91
    PINWHEEL: 0x92
    SICKLE_MOON_FLAG: 0x93
    SKULL_TOWER_IDOL: 0x94
    FOUNTAIN_IDOL: 0x95
    POSTMAN_STATUE: 0x96
    SHOP_GURU_STATUE: 0x97
    FATHERS_LETTER: 0x98
    NOTE_TO_MOM: 0x99
    MAGGIES_LETTER: 0x9A
    MOBLINS_LETTER: 0x9B
    CABANA_DEED: 0x9C
    COMPLIMENTARY_ID: 0x9D
    FILL_UP_COUPON: 0x9E
    GOLDEN_TINGLE_HEAD: 0xA3
    EMPTY: 0xFF
}

use crate::game::layer;
use crate::{Addr, Coord};
use core::mem::transmute;

pub fn spawn(coord: &Coord, item: u8) {
    layer::switch_to_safe_layer();

    let func =
        unsafe { transmute::<Addr, extern "C" fn(*const Coord, u8, u32, u32, u32)>(0x80026920) };
    func(coord, item, 0x7f, 0, 0);
}

pub fn item_id_to_str(item_id: u8) -> &'static str {
    match item_id {
        HEART_DROP => "Heart Drop",
        GREEN_RUPEE_DROP => "Green Rupee Drop",
        BLUE_RUPEE_DROP => "Blue Rupee Drop",
        YELLOW_RUPEE_DROP => "Yellow Rupee Drop",
        RED_RUPEE_DROP => "Red Rupee Drop",
        PURPLE_RUPEE_DROP => "Purple Rupee Drop",
        ORANGE_RUPEE_DROP => "Orange Rupee Drop",
        HEART_PIECE => "Heart Piece",
        HEART_CONTAINER => "Heart Container",
        SMALL_MAGIC_DROP => "Small Magic Drop",
        LARGE_MAGIC_DROP => "Large Magic Drop",
        BOMB_DROP => "Bomb Drop",
        SILVER_RUPEE_DROP => "Silver Rupee Drop",
        SINGLE_ARROW_DROP => "Single Arrow Drop",
        DOUBLE_ARROW_DROP => "Double Arrow Drop",
        TRIPLE_ARROW_DROP => "Triple Arrow Drop",
        KEY_DROP => "Key Drop",
        FAIRY_DROP => "Fairy Drop",
        TRIPLE_HEART_DROP => "Triple Heart Drop",
        JOY_PENDANT => "Joy Pendant",
        TELESCOPE => "Telescope",
        TINGLE_TUNER => "Tingle Tuner",
        WIND_WAKER => "Wind Waker",
        PICTO_BOX => "Picto Box",
        SPOILS_BAG => "Spoils Bag",
        GRAPPLING_HOOK => "Grappling Hook",
        DELUXE_PICTO_BOX => "Deluxe Picto Box",
        BOW => "Bow",
        POWER_BRACELETS => "Power Bracelets",
        IRON_BOOTS => "Iron Boots",
        MAGIC_ARMOR => "Magic Armor",
        WATER_BOOTS => "Water Boots",
        BAIT_BAG => "Bait Bag",
        BOOMERANG => "Boomerang",
        HOOKSHOT => "Hookshot",
        DELIVERY_BAG => "Delivery Bag",
        BOMBS => "Bombs",
        TUNIC => "Tunic",
        SKULL_HAMMER => "Skull Hammer",
        DEKU_LEAF => "Deku Leaf",
        BOW_WITH_FIRE_AND_ICE_ARROWS => "Bow With Fire And Ice Arrows",
        BOW_WITH_LIGHT_ARROWS => "Bow With Light Arrows",
        HEROS_SWORD => "Heros Sword",
        UNCHARGED_MASTER_SWORD => "Uncharged Master Sword",
        HALF_CHARGED_MASTER_SWORD => "Half Charged Master Sword",
        HEROS_SHIELD => "Heros Shield",
        MIRROR_SHIELD => "Mirror Shield",
        FULLY_CHARGED_MASTER_SWORD => "Fully Charged Master Sword",
        PIRATES_CHARM => "Pirates Charm",
        HEROS_CHARM => "Heros Charm",
        SKULL_NECKLACE => "Skull Necklace",
        BOKO_BABA_SEED => "Boko Baba Seed",
        GOLDEN_FEATHER => "Golden Feather",
        KNIGHTS_CREST => "Knights Crest",
        RED_CHU_JELLY => "Red Chu Jelly",
        GREEN_CHU_JELLY => "Green Chu Jelly",
        BLUE_CHU_JELLY => "Blue Chu Jelly",
        MAP => "Map",
        COMPASS => "Compass",
        BIG_KEY => "Big Key",
        EMPTY_BOTTLE => "Empty Bottle",
        RED_POTION => "Red Potion",
        GREEN_POTION => "Green Potion",
        BLUE_POTION => "Blue Potion",
        ELIXIR_SOUP_HALF => "Elixir Soup Half",
        ELIXIR_SOUP => "Elixir Soup",
        WATER => "Water",
        FAIRY => "Fairy",
        FOREST_FIREFLY => "Forest Firefly",
        FOREST_WATER => "Forest Water",
        TRIFORCE_PIECE_1 => "Triforce Piece 1",
        TRIFORCE_PIECE_2 => "Triforce Piece 2",
        TRIFORCE_PIECE_3 => "Triforce Piece 3",
        TRIFORCE_PIECE_4 => "Triforce Piece 4",
        TRIFORCE_PIECE_5 => "Triforce Piece 5",
        TRIFORCE_PIECE_6 => "Triforce Piece 6",
        TRIFORCE_PIECE_7 => "Triforce Piece 7",
        TRIFORCE_PIECE_8 => "Triforce Piece 8",
        NAYRUS_PEARL => "Nayrus Pearl",
        DINS_PEARL => "Dins Pearl",
        FARORES_PEARL => "Farores Pearl",
        SAIL => "Sail",
        TRIFORCE_CHART => "Triforce Chart",
        ALL_PURPOSE_BAIT => "All Purpose Bait",
        HYOI_PEAR => "Hyoi Pear",
        TOWN_FLOWER => "Town Flower",
        SEA_FLOWER => "Sea Flower",
        EXOTIC_FLOWER => "Exotic Flower",
        HEROS_FLAG => "Heros Flag",
        BIG_CATCH_FLAG => "Big Catch Flag",
        BIG_SALE_FLAG => "Big Sale Flag",
        PINWHEEL => "Pinwheel",
        SICKLE_MOON_FLAG => "Sickle Moon Flag",
        SKULL_TOWER_IDOL => "Skull Tower Idol",
        FOUNTAIN_IDOL => "Fountain Idol",
        POSTMAN_STATUE => "Postman Statue",
        SHOP_GURU_STATUE => "Shop Guru Statue",
        FATHERS_LETTER => "Fathers Letter",
        NOTE_TO_MOM => "Note To Mom",
        MAGGIES_LETTER => "Maggies Letter",
        MOBLINS_LETTER => "Moblins Letter",
        CABANA_DEED => "Cabana Deed",
        COMPLIMENTARY_ID => "Complimentary Id",
        FILL_UP_COUPON => "Fill Up Coupon",
        GOLDEN_TINGLE_HEAD => "Golden Tingle Head",
        EMPTY => "",
        _ => "???",
    }
}
