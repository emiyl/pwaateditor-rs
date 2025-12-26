use crate::offsets::*;

pub const SAVE_SIZE: usize = 0x16D730;
pub const GAME_COUNT: usize = 3;

#[derive(PartialEq, Clone, Copy)]
pub enum TextSkip {
    Off = 0,
    SingleBoxSkip,
    FullAutoSkip,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TextBoxTransparency {
    Off = 0,
    Low,
    High
}

#[derive(PartialEq, Clone, Copy)]
pub enum Language {
    Japanese = 0,
    English,
    French,
    German,
    Korean,
    ChineseSimplified,
    ChineseTraditional,
}

pub struct SaveFile {
    data: Vec<u8>,
}

const PLATFORM: Platform = Platform::Windows;

impl SaveFile {
    pub fn load(data: Vec<u8>) -> Self {
        assert_eq!(data.len(), SAVE_SIZE, "Invalid save file size");
        SaveFile { data }
    }

    pub fn save(&self) -> Vec<u8> { self.data.clone() }

    fn read_chapter_at(&self, offset: usize) -> u8 { (self.data[offset] >> 4) & 0x0F }
    fn write_chapter_at(&mut self, offset: usize, chapter: u8) {
        let low = self.data[offset] & 0xF;
        self.data[offset] = (chapter << 4) | low;
    }

    pub fn read_chapter_from_game(&self, game_index: usize) -> u8 {
        if (0..3).contains(&game_index) == false {
            panic!("Invalid game index");
        }
        let game_chapter_offsets = get_game_chapter_offsets(game_index);
        let offset = get_offset(PLATFORM, game_chapter_offsets);
        self.read_chapter_at(offset)
    }
    pub fn write_chapter_to_game(&mut self, game_index: usize, chapter: u8) {
        if (0..3).contains(&game_index) == false {
            panic!("Invalid game index");
        }
        if check_valid_chapter_for_game(game_index, chapter) == false {
            panic!("Invalid chapter for game index");
        }
        let game_chapter_offsets = get_game_chapter_offsets(game_index);
        let offset = get_offset(PLATFORM, game_chapter_offsets);
        self.write_chapter_at(offset, chapter);
    }
    
    pub fn read_bg_music_volume(&self) -> u8 { 
        let offset = get_offset(PLATFORM, Offset::BGMusicVolume);
        let value = self.data[offset] & 0x0F;
        if check_valid_music_volume(value) == false {
            panic!("Invalid BG music volume value in save file");
        }
        value
    }
    pub fn write_bg_music_volume(&mut self, volume: u8) {
        if check_valid_music_volume(volume) == false {
            panic!("Invalid BG music volume value to write");
        }
        let offset = get_offset(PLATFORM, Offset::BGMusicVolume);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | (volume & 0x0F);
    }

    pub fn read_se_volume(&self) -> u8 {
        let offset = get_offset(PLATFORM, Offset::SEVolume);
        let value = self.data[offset] & 0x0F;
        if check_valid_music_volume(value) == false {
            panic!("Invalid SE volume value in save file");
        }
        value
    }
    pub fn write_se_volume(&mut self, volume: u8) {
        if check_valid_music_volume(volume) == false {
            panic!("Invalid SE volume value to write");
        }
        let offset = get_offset(PLATFORM, Offset::SEVolume);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | (volume & 0x0F);
    }

    pub fn read_text_skip(&self) -> TextSkip { 
        let offset = get_offset(PLATFORM, Offset::TextSkip);
        convert_u8_to_text_skip(self.data[offset] & 0x0F)
    }
    pub fn write_text_skip(&mut self, skip: TextSkip) {
        let offset = get_offset(PLATFORM, Offset::TextSkip);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | (skip as u8 & 0x0F);
    }

    pub fn read_screen_shake(&self) -> bool {
        let offset = get_offset(PLATFORM, Offset::ScreenShake);
        (self.data[offset] & 0x0F) != 0
    }
    pub fn write_screen_shake(&mut self, enabled: bool) {
        let offset = get_offset(PLATFORM, Offset::ScreenShake);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | if enabled { 1 } else { 0 };
    }

    pub fn read_vibration(&self) -> bool {
        let offset = get_offset(PLATFORM, Offset::Vibration);
        (self.data[offset] & 0x0F) != 0
    }
    pub fn write_vibration(&mut self, enabled: bool) {
        let offset = get_offset(PLATFORM, Offset::Vibration);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | if enabled { 1 } else { 0 };
    }

    pub fn read_text_box_transparency(&self) -> TextBoxTransparency {
        let offset = get_offset(PLATFORM, Offset::TextBoxTransparency);
        convert_u8_to_text_box_transparency(self.data[offset] & 0x0F)
    }
    pub fn write_text_box_transparency(&mut self, transparency: TextBoxTransparency) {
        let offset = get_offset(PLATFORM, Offset::TextBoxTransparency);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | (transparency as u8 & 0x0F);
    }

    pub fn read_language(&self) -> Language {
        let offset = get_offset(PLATFORM, Offset::Language);
        convert_u8_to_language(self.data[offset] & 0x0F)
    }
    pub fn write_language(&mut self, language: Language) {
        let offset = get_offset(PLATFORM, Offset::Language);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | (language as u8 & 0x0F);
    }

    pub fn read_resolution_width(&self) -> u16 {
        let offset = get_offset(PLATFORM, Offset::ResolutionWidth);
        let low = self.data[offset] as u16;
        let high = self.data[offset + 1] as u16;
        (high << 8) | low
    }
    pub fn write_resolution_width(&mut self, width: u16) {
        let offset = get_offset(PLATFORM, Offset::ResolutionWidth);
        self.data[offset] = (width & 0x00FF) as u8;
        self.data[offset + 1] = (width >> 8) as u8;
    }

    pub fn read_resolution_height(&self) -> u16 {
        let offset = get_offset(PLATFORM, Offset::ResolutionHeight);
        let low = self.data[offset] as u16;
        let high = self.data[offset + 1] as u16;
        (high << 8) | low
    }
    pub fn write_resolution_height(&mut self, height: u16) {
        let offset = get_offset(PLATFORM, Offset::ResolutionHeight);
        self.data[offset] = (height & 0x00FF) as u8;
        self.data[offset + 1] = (height >> 8) as u8;
    }

    pub fn read_fullscreen(&self) -> bool {
        let offset = get_offset(PLATFORM, Offset::Fullscreen);
        (self.data[offset] & 0x0F) != 0
    }
    pub fn write_fullscreen(&mut self, enabled: bool) {
        let offset = get_offset(PLATFORM, Offset::Fullscreen);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | if enabled { 1 } else { 0 };
    }
    
    pub fn read_vsync(&self) -> bool {
        let offset = get_offset(PLATFORM, Offset::VSync);
        (self.data[offset] & 0x0F) != 0
    }
    pub fn write_vsync(&mut self, enabled: bool) {
        let offset = get_offset(PLATFORM, Offset::VSync);
        let high = self.data[offset] & 0xF0;
        self.data[offset] = high | if enabled { 1 } else { 0 };
    }
}

pub fn check_valid_chapter_for_game(game_index: usize, chapter: u8) -> bool {
    match game_index {
        0 => (1..=5).contains(&chapter),
        1 => (1..=4).contains(&chapter),
        2 => (1..=5).contains(&chapter),
        _ => false,
    }
}

fn get_game_chapter_offsets(game_index: usize) -> Offset {
    match game_index {
        0 => Offset::GameOneChapter,
        1 => Offset::GameTwoChapter,
        2 => Offset::GameThreeChapter,
        _ => panic!("Invalid game index"),
    }
}

pub fn check_valid_music_volume(value: u8) -> bool {
    (1..=5).contains(&value)
}

pub fn convert_u8_to_text_skip(value: u8) -> TextSkip {
    match value {
        0 => TextSkip::Off,
        1 => TextSkip::SingleBoxSkip,
        2 => TextSkip::FullAutoSkip,
        _ => panic!("Invalid text skip value"),
    }
}

pub fn convert_u8_to_text_box_transparency(value: u8) -> TextBoxTransparency {
    match value {
        0 => TextBoxTransparency::Off,
        1 => TextBoxTransparency::Low,
        2 => TextBoxTransparency::High,
        _ => panic!("Invalid text box transparency value"),
    }
}

pub fn convert_u8_to_language(value: u8) -> Language {
    match value {
        0 => Language::Japanese,
        1 => Language::English,
        2 => Language::French,
        3 => Language::German,
        4 => Language::Korean,
        5 => Language::ChineseSimplified,
        6 => Language::ChineseTraditional,
        _ => panic!("Invalid language setting"),
    }
}