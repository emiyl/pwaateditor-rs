pub enum Platform {
    Windows,
    // Switch
}

pub enum Offset {
    GameOneChapter,
    GameTwoChapter,
    GameThreeChapter,
    BGMusicVolume,
    SEVolume,
    TextSkip,
    ScreenShake,
    Vibration,
    TextBoxTransparency,
    Language,
    ResolutionWidth,
    ResolutionHeight,
    Fullscreen,
    VSync,
}

pub fn get_offset(platform: Platform, offset: Offset) -> usize {
    match platform {
        Platform::Windows => match offset {
            Offset::GameOneChapter => 0x12C4,
            Offset::GameTwoChapter => 0x12C5,
            Offset::GameThreeChapter => 0x12C6,
            Offset::BGMusicVolume => 0x12C8,
            Offset::SEVolume => 0x12CA,
            Offset::TextSkip => 0x12CC,
            Offset::ScreenShake => 0x12CE,
            Offset::Vibration => 0x12D0,
            Offset::TextBoxTransparency => 0x12D2,
            Offset::Language => 0x12D4,
            Offset::ResolutionWidth => 0x12D8,
            Offset::ResolutionHeight => 0x12DC,
            Offset::Fullscreen => 0x12D6,
            Offset::VSync => 0x12E0,
        },
        // Platform::Switch => {
        //     // Placeholder for Switch offsets
        //     unimplemented!("Switch platform offsets are not implemented yet");
        // }
    }
}