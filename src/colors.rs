#![allow(dead_code)]

use sdl2::pixels::Color;

pub const COLOR_NAMES: &[(&str, Color)] = &[
    ("ALICE_BLUE", ALICE_BLUE),
    ("ANTIQUE_WHITE", ANTIQUE_WHITE),
    ("AQUA", AQUA),
    ("AQUAMARINE", AQUAMARINE),
    ("AZURE", AZURE),
    ("BEIGE", BEIGE),
    ("BISQUE", BISQUE),
    ("BLACK", BLACK),
    ("BLANCHED_ALMOND", BLANCHED_ALMOND),
    ("BLUE", BLUE),
    ("BLUE_VIOLET", BLUE_VIOLET),
    ("BROWN", BROWN),
    ("BURLY_WOOD", BURLY_WOOD),
    ("CADET_BLUE", CADET_BLUE),
    ("CHARTREUSE", CHARTREUSE),
    ("CHOCOLATE", CHOCOLATE),
    ("CORAL", CORAL),
    ("CORNFLOWER_BLUE", CORNFLOWER_BLUE),
    ("CORNSILK", CORNSILK),
    ("CRIMSON", CRIMSON),
    ("CYAN", CYAN),
    ("DARK_BLUE", DARK_BLUE),
    ("DARK_CYAN", DARK_CYAN),
    ("DARK_GOLDEN_ROD", DARK_GOLDEN_ROD),
    ("DARK_GRAY", DARK_GRAY),
    ("DARK_GREEN", DARK_GREEN),
    ("DARK_KHAKI", DARK_KHAKI),
    ("DARK_MAGENTA", DARK_MAGENTA),
    ("DARK_OLIVE_GREEN", DARK_OLIVE_GREEN),
    ("DARK_ORANGE", DARK_ORANGE),
    ("DARK_ORCHID", DARK_ORCHID),
    ("DARK_RED", DARK_RED),
    ("DARK_SALMON", DARK_SALMON),
    ("DARK_SEA_GREEN", DARK_SEA_GREEN),
    ("DARK_SLATE_BLUE", DARK_SLATE_BLUE),
    ("DARK_SLATE_GRAY", DARK_SLATE_GRAY),
    ("DARK_TURQUOISE", DARK_TURQUOISE),
    ("DARK_VIOLET", DARK_VIOLET),
    ("DEEP_PINK", DEEP_PINK),
    ("DEEP_SKY_BLUE", DEEP_SKY_BLUE),
    ("DIM_GRAY", DIM_GRAY),
    ("DODGER_BLUE", DODGER_BLUE),
    ("FIRE_BRICK", FIRE_BRICK),
    ("FLORAL_WHITE", FLORAL_WHITE),
    ("FOREST_GREEN", FOREST_GREEN),
    ("FUCHSIA", FUCHSIA),
    ("GAINSBORO", GAINSBORO),
    ("GHOST_WHITE", GHOST_WHITE),
    ("GOLD", GOLD),
    ("GOLDEN_ROD", GOLDEN_ROD),
    ("GRAY", GRAY),
    ("GREEN", GREEN),
    ("GREEN_YELLOW", GREEN_YELLOW),
    ("HONEY_DEW", HONEY_DEW),
    ("HOT_PINK", HOT_PINK),
    ("INDIAN_RED", INDIAN_RED),
    ("INDIGO", INDIGO),
    ("IVORY", IVORY),
    ("KHAKI", KHAKI),
    ("LAVENDER", LAVENDER),
    ("LAVENDER_BLUSH", LAVENDER_BLUSH),
    ("LAWN_GREEN", LAWN_GREEN),
    ("LEMON_CHIFFON", LEMON_CHIFFON),
    ("LIGHT_BLUE", LIGHT_BLUE),
    ("LIGHT_CORAL", LIGHT_CORAL),
    ("LIGHT_CYAN", LIGHT_CYAN),
    ("LIGHT_GOLDEN_ROD_YELLOW", LIGHT_GOLDEN_ROD_YELLOW),
    ("LIGHT_GRAY", LIGHT_GRAY),
    ("LIGHT_GREEN", LIGHT_GREEN),
    ("LIGHT_PINK", LIGHT_PINK),
    ("LIGHT_SALMON", LIGHT_SALMON),
    ("LIGHT_SEA_GREEN", LIGHT_SEA_GREEN),
    ("LIGHT_SKY_BLUE", LIGHT_SKY_BLUE),
    ("LIGHT_SLATE_GRAY", LIGHT_SLATE_GRAY),
    ("LIGHT_STEEL_BLUE", LIGHT_STEEL_BLUE),
    ("LIGHT_YELLOW", LIGHT_YELLOW),
    ("LIME", LIME),
    ("LIME_GREEN", LIME_GREEN),
    ("LINEN", LINEN),
    ("MAGENTA", MAGENTA),
    ("MAROON", MAROON),
    ("MEDIUM_AQUA_MARINE", MEDIUM_AQUA_MARINE),
    ("MEDIUM_BLUE", MEDIUM_BLUE),
    ("MEDIUM_ORCHID", MEDIUM_ORCHID),
    ("MEDIUM_PURPLE", MEDIUM_PURPLE),
    ("MEDIUM_SEA_GREEN", MEDIUM_SEA_GREEN),
    ("MEDIUM_SLATE_BLUE", MEDIUM_SLATE_BLUE),
    ("MEDIUM_SPRING_GREEN", MEDIUM_SPRING_GREEN),
    ("MEDIUM_TURQUOISE", MEDIUM_TURQUOISE),
    ("MEDIUM_VIOLET_RED", MEDIUM_VIOLET_RED),
    ("MIDNIGHT_BLUE", MIDNIGHT_BLUE),
    ("MINT_CREAM", MINT_CREAM),
    ("MISTY_ROSE", MISTY_ROSE),
    ("MOCCASIN", MOCCASIN),
    ("NAVAJO_WHITE", NAVAJO_WHITE),
    ("NAVY", NAVY),
    ("OLD_LACE", OLD_LACE),
    ("OLIVE", OLIVE),
    ("OLIVE_DRAB", OLIVE_DRAB),
    ("ORANGE", ORANGE),
    ("ORANGE_RED", ORANGE_RED),
    ("ORCHID", ORCHID),
    ("PALE_GOLDEN_ROD", PALE_GOLDEN_ROD),
    ("PALE_GREEN", PALE_GREEN),
    ("PALE_TURQUOISE", PALE_TURQUOISE),
    ("PALE_VIOLET_RED", PALE_VIOLET_RED),
    ("PAPAYA_WHIP", PAPAYA_WHIP),
    ("PEACH_PUFF", PEACH_PUFF),
    ("PERU", PERU),
    ("PINK", PINK),
    ("PLUM", PLUM),
    ("POWDER_BLUE", POWDER_BLUE),
    ("PURPLE", PURPLE),
    ("RED", RED),
    ("ROSY_BROWN", ROSY_BROWN),
    ("ROYAL_BLUE", ROYAL_BLUE),
    ("SADDLE_BROWN", SADDLE_BROWN),
    ("SALMON", SALMON),
    ("SANDY_BROWN", SANDY_BROWN),
    ("SEA_GREEN", SEA_GREEN),
    ("SEA_SHELL", SEA_SHELL),
    ("SIENNA", SIENNA),
    ("SILVER", SILVER),
    ("SKY_BLUE", SKY_BLUE),
    ("SLATE_BLUE", SLATE_BLUE),
    ("SLATE_GRAY", SLATE_GRAY),
    ("SNOW", SNOW),
    ("SPRING_GREEN", SPRING_GREEN),
    ("STEEL_BLUE", STEEL_BLUE),
    ("TAN", TAN),
    ("TEAL", TEAL),
    ("THISTLE", THISTLE),
    ("TOMATO", TOMATO),
    ("TURQUOISE", TURQUOISE),
    ("VIOLET", VIOLET),
    ("WHEAT", WHEAT),
    ("WHITE", WHITE),
    ("WHITE_SMOKE", WHITE_SMOKE),
    ("YELLOW", YELLOW),
    ("YELLOW_GREEN", YELLOW_GREEN),
];

pub const ALICE_BLUE: Color = Color::RGB(240, 248, 255);
pub const ANTIQUE_WHITE: Color = Color::RGB(250, 235, 215);
pub const AQUA: Color = Color::RGB(0, 255, 255);
pub const AQUAMARINE: Color = Color::RGB(127, 255, 212);
pub const AZURE: Color = Color::RGB(240, 255, 255);
pub const BEIGE: Color = Color::RGB(245, 245, 220);
pub const BISQUE: Color = Color::RGB(255, 228, 196);
pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const BLANCHED_ALMOND: Color = Color::RGB(255, 235, 205);
pub const BLUE: Color = Color::RGB(0, 0, 255);
pub const BLUE_VIOLET: Color = Color::RGB(138, 43, 226);
pub const BROWN: Color = Color::RGB(165, 42, 42);
pub const BURLY_WOOD: Color = Color::RGB(222, 184, 135);
pub const CADET_BLUE: Color = Color::RGB(95, 158, 160);
pub const CHARTREUSE: Color = Color::RGB(127, 255, 0);
pub const CHOCOLATE: Color = Color::RGB(210, 105, 30);
pub const CORAL: Color = Color::RGB(255, 127, 80);
pub const CORNFLOWER_BLUE: Color = Color::RGB(100, 149, 237);
pub const CORNSILK: Color = Color::RGB(255, 248, 220);
pub const CRIMSON: Color = Color::RGB(220, 20, 60);
pub const CYAN: Color = Color::RGB(0, 255, 255);
pub const DARK_BLUE: Color = Color::RGB(0, 0, 139);
pub const DARK_CYAN: Color = Color::RGB(0, 139, 139);
pub const DARK_GOLDEN_ROD: Color = Color::RGB(184, 134, 11);
pub const DARK_GRAY: Color = Color::RGB(169, 169, 169);
pub const DARK_GREEN: Color = Color::RGB(0, 100, 0);
pub const DARK_KHAKI: Color = Color::RGB(189, 183, 107);
pub const DARK_MAGENTA: Color = Color::RGB(139, 0, 139);
pub const DARK_OLIVE_GREEN: Color = Color::RGB(85, 107, 47);
pub const DARK_ORANGE: Color = Color::RGB(255, 140, 0);
pub const DARK_ORCHID: Color = Color::RGB(153, 50, 204);
pub const DARK_RED: Color = Color::RGB(139, 0, 0);
pub const DARK_SALMON: Color = Color::RGB(233, 150, 122);
pub const DARK_SEA_GREEN: Color = Color::RGB(143, 188, 143);
pub const DARK_SLATE_BLUE: Color = Color::RGB(72, 61, 139);
pub const DARK_SLATE_GRAY: Color = Color::RGB(47, 79, 79);
pub const DARK_TURQUOISE: Color = Color::RGB(0, 206, 209);
pub const DARK_VIOLET: Color = Color::RGB(148, 0, 211);
pub const DEEP_PINK: Color = Color::RGB(255, 20, 147);
pub const DEEP_SKY_BLUE: Color = Color::RGB(0, 191, 255);
pub const DIM_GRAY: Color = Color::RGB(105, 105, 105);
pub const DODGER_BLUE: Color = Color::RGB(30, 144, 255);
pub const FIRE_BRICK: Color = Color::RGB(178, 34, 34);
pub const FLORAL_WHITE: Color = Color::RGB(255, 250, 240);
pub const FOREST_GREEN: Color = Color::RGB(34, 139, 34);
pub const FUCHSIA: Color = Color::RGB(255, 0, 255);
pub const GAINSBORO: Color = Color::RGB(220, 220, 220);
pub const GHOST_WHITE: Color = Color::RGB(248, 248, 255);
pub const GOLD: Color = Color::RGB(255, 215, 0);
pub const GOLDEN_ROD: Color = Color::RGB(218, 165, 32);
pub const GRAY: Color = Color::RGB(128, 128, 128);
pub const GREEN: Color = Color::RGB(0, 128, 0);
pub const GREEN_YELLOW: Color = Color::RGB(173, 255, 47);
pub const HONEY_DEW: Color = Color::RGB(240, 255, 240);
pub const HOT_PINK: Color = Color::RGB(255, 105, 180);
pub const INDIAN_RED: Color = Color::RGB(205, 92, 92);
pub const INDIGO: Color = Color::RGB(75, 0, 130);
pub const IVORY: Color = Color::RGB(255, 255, 240);
pub const KHAKI: Color = Color::RGB(240, 230, 140);
pub const LAVENDER: Color = Color::RGB(230, 230, 250);
pub const LAVENDER_BLUSH: Color = Color::RGB(255, 240, 245);
pub const LAWN_GREEN: Color = Color::RGB(124, 252, 0);
pub const LEMON_CHIFFON: Color = Color::RGB(255, 250, 205);
pub const LIGHT_BLUE: Color = Color::RGB(173, 216, 230);
pub const LIGHT_CORAL: Color = Color::RGB(240, 128, 128);
pub const LIGHT_CYAN: Color = Color::RGB(224, 255, 255);
pub const LIGHT_GOLDEN_ROD_YELLOW: Color = Color::RGB(250, 250, 210);
pub const LIGHT_GRAY: Color = Color::RGB(211, 211, 211);
pub const LIGHT_GREEN: Color = Color::RGB(144, 238, 144);
pub const LIGHT_PINK: Color = Color::RGB(255, 182, 193);
pub const LIGHT_SALMON: Color = Color::RGB(255, 160, 122);
pub const LIGHT_SEA_GREEN: Color = Color::RGB(32, 178, 170);
pub const LIGHT_SKY_BLUE: Color = Color::RGB(135, 206, 250);
pub const LIGHT_SLATE_GRAY: Color = Color::RGB(119, 136, 153);
pub const LIGHT_STEEL_BLUE: Color = Color::RGB(176, 196, 222);
pub const LIGHT_YELLOW: Color = Color::RGB(255, 255, 224);
pub const LIME: Color = Color::RGB(0, 255, 0);
pub const LIME_GREEN: Color = Color::RGB(50, 205, 50);
pub const LINEN: Color = Color::RGB(250, 240, 230);
pub const MAGENTA: Color = Color::RGB(255, 0, 255);
pub const MAROON: Color = Color::RGB(128, 0, 0);
pub const MEDIUM_AQUA_MARINE: Color = Color::RGB(102, 205, 170);
pub const MEDIUM_BLUE: Color = Color::RGB(0, 0, 205);
pub const MEDIUM_ORCHID: Color = Color::RGB(186, 85, 211);
pub const MEDIUM_PURPLE: Color = Color::RGB(147, 112, 219);
pub const MEDIUM_SEA_GREEN: Color = Color::RGB(60, 179, 113);
pub const MEDIUM_SLATE_BLUE: Color = Color::RGB(123, 104, 238);
pub const MEDIUM_SPRING_GREEN: Color = Color::RGB(0, 250, 154);
pub const MEDIUM_TURQUOISE: Color = Color::RGB(72, 209, 204);
pub const MEDIUM_VIOLET_RED: Color = Color::RGB(199, 21, 133);
pub const MIDNIGHT_BLUE: Color = Color::RGB(25, 25, 112);
pub const MINT_CREAM: Color = Color::RGB(245, 255, 250);
pub const MISTY_ROSE: Color = Color::RGB(255, 228, 225);
pub const MOCCASIN: Color = Color::RGB(255, 228, 181);
pub const NAVAJO_WHITE: Color = Color::RGB(255, 222, 173);
pub const NAVY: Color = Color::RGB(0, 0, 128);
pub const OLD_LACE: Color = Color::RGB(253, 245, 230);
pub const OLIVE: Color = Color::RGB(128, 128, 0);
pub const OLIVE_DRAB: Color = Color::RGB(107, 142, 35);
pub const ORANGE: Color = Color::RGB(255, 165, 0);
pub const ORANGE_RED: Color = Color::RGB(255, 69, 0);
pub const ORCHID: Color = Color::RGB(218, 112, 214);
pub const PALE_GOLDEN_ROD: Color = Color::RGB(238, 232, 170);
pub const PALE_GREEN: Color = Color::RGB(152, 251, 152);
pub const PALE_TURQUOISE: Color = Color::RGB(175, 238, 238);
pub const PALE_VIOLET_RED: Color = Color::RGB(219, 112, 147);
pub const PAPAYA_WHIP: Color = Color::RGB(255, 239, 213);
pub const PEACH_PUFF: Color = Color::RGB(255, 218, 185);
pub const PERU: Color = Color::RGB(205, 133, 63);
pub const PINK: Color = Color::RGB(255, 192, 203);
pub const PLUM: Color = Color::RGB(221, 160, 221);
pub const POWDER_BLUE: Color = Color::RGB(176, 224, 230);
pub const PURPLE: Color = Color::RGB(128, 0, 128);
pub const RED: Color = Color::RGB(255, 0, 0);
pub const ROSY_BROWN: Color = Color::RGB(188, 143, 143);
pub const ROYAL_BLUE: Color = Color::RGB(65, 105, 225);
pub const SADDLE_BROWN: Color = Color::RGB(139, 69, 19);
pub const SALMON: Color = Color::RGB(250, 128, 114);
pub const SANDY_BROWN: Color = Color::RGB(244, 164, 96);
pub const SEA_GREEN: Color = Color::RGB(46, 139, 87);
pub const SEA_SHELL: Color = Color::RGB(255, 245, 238);
pub const SIENNA: Color = Color::RGB(160, 82, 45);
pub const SILVER: Color = Color::RGB(192, 192, 192);
pub const SKY_BLUE: Color = Color::RGB(135, 206, 235);
pub const SLATE_BLUE: Color = Color::RGB(106, 90, 205);
pub const SLATE_GRAY: Color = Color::RGB(112, 128, 144);
pub const SNOW: Color = Color::RGB(255, 250, 250);
pub const SPRING_GREEN: Color = Color::RGB(0, 255, 127);
pub const STEEL_BLUE: Color = Color::RGB(70, 130, 180);
pub const TAN: Color = Color::RGB(210, 180, 140);
pub const TEAL: Color = Color::RGB(0, 128, 128);
pub const THISTLE: Color = Color::RGB(216, 191, 216);
pub const TOMATO: Color = Color::RGB(255, 99, 71);
pub const TURQUOISE: Color = Color::RGB(64, 224, 208);
pub const VIOLET: Color = Color::RGB(238, 130, 238);
pub const WHEAT: Color = Color::RGB(245, 222, 179);
pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const WHITE_SMOKE: Color = Color::RGB(245, 245, 245);
pub const YELLOW: Color = Color::RGB(255, 255, 0);
pub const YELLOW_GREEN: Color = Color::RGB(154, 205, 50);
