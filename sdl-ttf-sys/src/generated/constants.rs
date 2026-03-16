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

pub const TTF_PROP_FONT_CREATE_FILENAME_STRING: Property = Property { name: "TTF_PROP_FONT_CREATE_FILENAME_STRING", value: "SDL_ttf.font.create.filename", ty: PropertyType::String, doc: "" };
pub const TTF_PROP_FONT_CREATE_IOSTREAM_POINTER: Property = Property { name: "TTF_PROP_FONT_CREATE_IOSTREAM_POINTER", value: "SDL_ttf.font.create.iostream", ty: PropertyType::Pointer, doc: "" };
pub const TTF_PROP_FONT_CREATE_IOSTREAM_OFFSET_NUMBER: Property = Property { name: "TTF_PROP_FONT_CREATE_IOSTREAM_OFFSET_NUMBER", value: "SDL_ttf.font.create.iostream.offset", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_FONT_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN: Property = Property { name: "TTF_PROP_FONT_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN", value: "SDL_ttf.font.create.iostream.autoclose", ty: PropertyType::Boolean, doc: "" };
pub const TTF_PROP_FONT_CREATE_SIZE_FLOAT: Property = Property { name: "TTF_PROP_FONT_CREATE_SIZE_FLOAT", value: "SDL_ttf.font.create.size", ty: PropertyType::Float, doc: "" };
pub const TTF_PROP_FONT_CREATE_FACE_NUMBER: Property = Property { name: "TTF_PROP_FONT_CREATE_FACE_NUMBER", value: "SDL_ttf.font.create.face", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_FONT_CREATE_HORIZONTAL_DPI_NUMBER: Property = Property { name: "TTF_PROP_FONT_CREATE_HORIZONTAL_DPI_NUMBER", value: "SDL_ttf.font.create.hdpi", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_FONT_CREATE_VERTICAL_DPI_NUMBER: Property = Property { name: "TTF_PROP_FONT_CREATE_VERTICAL_DPI_NUMBER", value: "SDL_ttf.font.create.vdpi", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_FONT_CREATE_EXISTING_FONT_POINTER: Property = Property { name: "TTF_PROP_FONT_CREATE_EXISTING_FONT_POINTER", value: "SDL_ttf.font.create.existing_font", ty: PropertyType::Pointer, doc: "" };
pub const TTF_PROP_FONT_OUTLINE_LINE_CAP_NUMBER: Property = Property { name: "TTF_PROP_FONT_OUTLINE_LINE_CAP_NUMBER", value: "SDL_ttf.font.outline.line_cap", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_FONT_OUTLINE_LINE_JOIN_NUMBER: Property = Property { name: "TTF_PROP_FONT_OUTLINE_LINE_JOIN_NUMBER", value: "SDL_ttf.font.outline.line_join", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_FONT_OUTLINE_MITER_LIMIT_NUMBER: Property = Property { name: "TTF_PROP_FONT_OUTLINE_MITER_LIMIT_NUMBER", value: "SDL_ttf.font.outline.miter_limit", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_RENDERER_TEXT_ENGINE_RENDERER_POINTER: Property = Property { name: "TTF_PROP_RENDERER_TEXT_ENGINE_RENDERER_POINTER", value: "SDL_ttf.renderer_text_engine.create.renderer", ty: PropertyType::Pointer, doc: "" };
pub const TTF_PROP_RENDERER_TEXT_ENGINE_ATLAS_TEXTURE_SIZE_NUMBER: Property = Property { name: "TTF_PROP_RENDERER_TEXT_ENGINE_ATLAS_TEXTURE_SIZE_NUMBER", value: "SDL_ttf.renderer_text_engine.create.atlas_texture_size", ty: PropertyType::Number, doc: "" };
pub const TTF_PROP_GPU_TEXT_ENGINE_DEVICE_POINTER: Property = Property { name: "TTF_PROP_GPU_TEXT_ENGINE_DEVICE_POINTER", value: "SDL_ttf.gpu_text_engine.create.device", ty: PropertyType::Pointer, doc: "" };
pub const TTF_PROP_GPU_TEXT_ENGINE_ATLAS_TEXTURE_SIZE_NUMBER: Property = Property { name: "TTF_PROP_GPU_TEXT_ENGINE_ATLAS_TEXTURE_SIZE_NUMBER", value: "SDL_ttf.gpu_text_engine.create.atlas_texture_size", ty: PropertyType::Number, doc: "" };
