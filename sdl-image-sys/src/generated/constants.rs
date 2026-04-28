//! Generated constants

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType { Pointer, String, Number, Float, Boolean }

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

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_FILENAME_STRING: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_FILENAME_STRING", value: "SDL_image.animation_encoder.create.filename", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_POINTER: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_POINTER", value: "SDL_image.animation_encoder.create.iostream", ty: PropertyType::Pointer, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN", value: "SDL_image.animation_encoder.create.iostream.autoclose", ty: PropertyType::Boolean, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_TYPE_STRING: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_TYPE_STRING", value: "SDL_image.animation_encoder.create.type", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_QUALITY_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_QUALITY_NUMBER", value: "SDL_image.animation_encoder.create.quality", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_NUMERATOR_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_NUMERATOR_NUMBER", value: "SDL_image.animation_encoder.create.timebase.numerator", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER", value: "SDL_image.animation_encoder.create.timebase.denominator", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_AVIF_MAX_THREADS_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_AVIF_MAX_THREADS_NUMBER", value: "SDL_image.animation_encoder.create.avif.max_threads", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_AVIF_KEYFRAME_INTERVAL_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_AVIF_KEYFRAME_INTERVAL_NUMBER", value: "SDL_image.animation_encoder.create.avif.keyframe_interval", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_ENCODER_CREATE_GIF_USE_LUT_BOOLEAN: Property = Property { name: "IMG_PROP_ANIMATION_ENCODER_CREATE_GIF_USE_LUT_BOOLEAN", value: "SDL_image.animation_encoder.create.gif.use_lut", ty: PropertyType::Boolean, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_FILENAME_STRING: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_FILENAME_STRING", value: "SDL_image.animation_decoder.create.filename", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_POINTER: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_POINTER", value: "SDL_image.animation_decoder.create.iostream", ty: PropertyType::Pointer, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN", value: "SDL_image.animation_decoder.create.iostream.autoclose", ty: PropertyType::Boolean, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_TYPE_STRING: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_TYPE_STRING", value: "SDL_image.animation_decoder.create.type", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_TIMEBASE_NUMERATOR_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_TIMEBASE_NUMERATOR_NUMBER", value: "SDL_image.animation_decoder.create.timebase.numerator", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER", value: "SDL_image.animation_decoder.create.timebase.denominator", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_MAX_THREADS_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_MAX_THREADS_NUMBER", value: "SDL_image.animation_decoder.create.avif.max_threads", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_ALLOW_INCREMENTAL_BOOLEAN: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_ALLOW_INCREMENTAL_BOOLEAN", value: "SDL_image.animation_decoder.create.avif.allow_incremental", ty: PropertyType::Boolean, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_ALLOW_PROGRESSIVE_BOOLEAN: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_ALLOW_PROGRESSIVE_BOOLEAN", value: "SDL_image.animation_decoder.create.avif.allow_progressive", ty: PropertyType::Boolean, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_GIF_TRANSPARENT_COLOR_INDEX_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_GIF_TRANSPARENT_COLOR_INDEX_NUMBER", value: "SDL_image.animation_encoder.create.gif.transparent_color_index", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_ANIMATION_DECODER_CREATE_GIF_NUM_COLORS_NUMBER: Property = Property { name: "IMG_PROP_ANIMATION_DECODER_CREATE_GIF_NUM_COLORS_NUMBER", value: "SDL_image.animation_encoder.create.gif.num_colors", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_METADATA_IGNORE_PROPS_BOOLEAN: Property = Property { name: "IMG_PROP_METADATA_IGNORE_PROPS_BOOLEAN", value: "SDL_image.metadata.ignore_props", ty: PropertyType::Boolean, doc: "" };
pub const IMG_PROP_METADATA_DESCRIPTION_STRING: Property = Property { name: "IMG_PROP_METADATA_DESCRIPTION_STRING", value: "SDL_image.metadata.description", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_METADATA_COPYRIGHT_STRING: Property = Property { name: "IMG_PROP_METADATA_COPYRIGHT_STRING", value: "SDL_image.metadata.copyright", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_METADATA_TITLE_STRING: Property = Property { name: "IMG_PROP_METADATA_TITLE_STRING", value: "SDL_image.metadata.title", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_METADATA_AUTHOR_STRING: Property = Property { name: "IMG_PROP_METADATA_AUTHOR_STRING", value: "SDL_image.metadata.author", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_METADATA_CREATION_TIME_STRING: Property = Property { name: "IMG_PROP_METADATA_CREATION_TIME_STRING", value: "SDL_image.metadata.creation_time", ty: PropertyType::String, doc: "" };
pub const IMG_PROP_METADATA_FRAME_COUNT_NUMBER: Property = Property { name: "IMG_PROP_METADATA_FRAME_COUNT_NUMBER", value: "SDL_image.metadata.frame_count", ty: PropertyType::Number, doc: "" };
pub const IMG_PROP_METADATA_LOOP_COUNT_NUMBER: Property = Property { name: "IMG_PROP_METADATA_LOOP_COUNT_NUMBER", value: "SDL_image.metadata.loop_count", ty: PropertyType::Number, doc: "" };
