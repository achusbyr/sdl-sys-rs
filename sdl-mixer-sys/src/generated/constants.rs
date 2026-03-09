//! Generated constants

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    Pointer,
    String,
    Number,
    Float,
    Boolean,
}

#[derive(Debug, Clone, Copy)]
pub struct Hint {
    pub name: &'static str,
    pub value: &'static str,
    pub doc: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Property {
    pub name: &'static str,
    pub value: &'static str,
    pub ty: PropertyType,
    pub doc: &'static str,
}

pub const MIX_PROP_MIXER_DEVICE_NUMBER: Property = Property {
    name: "MIX_PROP_MIXER_DEVICE_NUMBER",
    value: "SDL_mixer.mixer.device",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_AUDIO_LOAD_IOSTREAM_POINTER: Property = Property {
    name: "MIX_PROP_AUDIO_LOAD_IOSTREAM_POINTER",
    value: "SDL_mixer.audio.load.iostream",
    ty: PropertyType::Pointer,
    doc: "",
};
pub const MIX_PROP_AUDIO_LOAD_CLOSEIO_BOOLEAN: Property = Property {
    name: "MIX_PROP_AUDIO_LOAD_CLOSEIO_BOOLEAN",
    value: "SDL_mixer.audio.load.closeio",
    ty: PropertyType::Boolean,
    doc: "",
};
pub const MIX_PROP_AUDIO_LOAD_PREDECODE_BOOLEAN: Property = Property {
    name: "MIX_PROP_AUDIO_LOAD_PREDECODE_BOOLEAN",
    value: "SDL_mixer.audio.load.predecode",
    ty: PropertyType::Boolean,
    doc: "",
};
pub const MIX_PROP_AUDIO_LOAD_PREFERRED_MIXER_POINTER: Property = Property {
    name: "MIX_PROP_AUDIO_LOAD_PREFERRED_MIXER_POINTER",
    value: "SDL_mixer.audio.load.preferred_mixer",
    ty: PropertyType::Pointer,
    doc: "",
};
pub const MIX_PROP_AUDIO_LOAD_SKIP_METADATA_TAGS_BOOLEAN: Property = Property {
    name: "MIX_PROP_AUDIO_LOAD_SKIP_METADATA_TAGS_BOOLEAN",
    value: "SDL_mixer.audio.load.skip_metadata_tags",
    ty: PropertyType::Boolean,
    doc: "",
};
pub const MIX_PROP_AUDIO_DECODER_STRING: Property = Property {
    name: "MIX_PROP_AUDIO_DECODER_STRING",
    value: "SDL_mixer.audio.decoder",
    ty: PropertyType::String,
    doc: "",
};
pub const MIX_PROP_METADATA_TITLE_STRING: Property = Property {
    name: "MIX_PROP_METADATA_TITLE_STRING",
    value: "SDL_mixer.metadata.title",
    ty: PropertyType::String,
    doc: "",
};
pub const MIX_PROP_METADATA_ARTIST_STRING: Property = Property {
    name: "MIX_PROP_METADATA_ARTIST_STRING",
    value: "SDL_mixer.metadata.artist",
    ty: PropertyType::String,
    doc: "",
};
pub const MIX_PROP_METADATA_ALBUM_STRING: Property = Property {
    name: "MIX_PROP_METADATA_ALBUM_STRING",
    value: "SDL_mixer.metadata.album",
    ty: PropertyType::String,
    doc: "",
};
pub const MIX_PROP_METADATA_COPYRIGHT_STRING: Property = Property {
    name: "MIX_PROP_METADATA_COPYRIGHT_STRING",
    value: "SDL_mixer.metadata.copyright",
    ty: PropertyType::String,
    doc: "",
};
pub const MIX_PROP_METADATA_TRACK_NUMBER: Property = Property {
    name: "MIX_PROP_METADATA_TRACK_NUMBER",
    value: "SDL_mixer.metadata.track",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_METADATA_TOTAL_TRACKS_NUMBER: Property = Property {
    name: "MIX_PROP_METADATA_TOTAL_TRACKS_NUMBER",
    value: "SDL_mixer.metadata.total_tracks",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_METADATA_YEAR_NUMBER: Property = Property {
    name: "MIX_PROP_METADATA_YEAR_NUMBER",
    value: "SDL_mixer.metadata.year",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_METADATA_DURATION_FRAMES_NUMBER: Property = Property {
    name: "MIX_PROP_METADATA_DURATION_FRAMES_NUMBER",
    value: "SDL_mixer.metadata.duration_frames",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_METADATA_DURATION_INFINITE_BOOLEAN: Property = Property {
    name: "MIX_PROP_METADATA_DURATION_INFINITE_BOOLEAN",
    value: "SDL_mixer.metadata.duration_infinite",
    ty: PropertyType::Boolean,
    doc: "",
};
pub const MIX_PROP_PLAY_LOOPS_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_LOOPS_NUMBER",
    value: "SDL_mixer.play.loops",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_MAX_FRAME_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_MAX_FRAME_NUMBER",
    value: "SDL_mixer.play.max_frame",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_MAX_MILLISECONDS_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_MAX_MILLISECONDS_NUMBER",
    value: "SDL_mixer.play.max_milliseconds",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_START_FRAME_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_START_FRAME_NUMBER",
    value: "SDL_mixer.play.start_frame",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_START_MILLISECOND_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_START_MILLISECOND_NUMBER",
    value: "SDL_mixer.play.start_millisecond",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_LOOP_START_FRAME_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_LOOP_START_FRAME_NUMBER",
    value: "SDL_mixer.play.loop_start_frame",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_LOOP_START_MILLISECOND_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_LOOP_START_MILLISECOND_NUMBER",
    value: "SDL_mixer.play.loop_start_millisecond",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_FADE_IN_FRAMES_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_FADE_IN_FRAMES_NUMBER",
    value: "SDL_mixer.play.fade_in_frames",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_FADE_IN_MILLISECONDS_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_FADE_IN_MILLISECONDS_NUMBER",
    value: "SDL_mixer.play.fade_in_milliseconds",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_FADE_IN_START_GAIN_FLOAT: Property = Property {
    name: "MIX_PROP_PLAY_FADE_IN_START_GAIN_FLOAT",
    value: "SDL_mixer.play.fade_in_start_gain",
    ty: PropertyType::Float,
    doc: "",
};
pub const MIX_PROP_PLAY_APPEND_SILENCE_FRAMES_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_APPEND_SILENCE_FRAMES_NUMBER",
    value: "SDL_mixer.play.append_silence_frames",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_APPEND_SILENCE_MILLISECONDS_NUMBER: Property = Property {
    name: "MIX_PROP_PLAY_APPEND_SILENCE_MILLISECONDS_NUMBER",
    value: "SDL_mixer.play.append_silence_milliseconds",
    ty: PropertyType::Number,
    doc: "",
};
pub const MIX_PROP_PLAY_HALT_WHEN_EXHAUSTED_BOOLEAN: Property = Property {
    name: "MIX_PROP_PLAY_HALT_WHEN_EXHAUSTED_BOOLEAN",
    value: "SDL_mixer.play.halt_when_exhausted",
    ty: PropertyType::Boolean,
    doc: "",
};
