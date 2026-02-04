use crate::types::*;

pub type TextDraw_Create_t =
    unsafe extern "C" fn(x: f32, y: f32, text: *const i8, id: *mut i32) -> TextDrawPtr;
pub type TextDraw_Destroy_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> bool;
pub type TextDraw_FromID_t = unsafe extern "C" fn(textdrawid: i32) -> TextDrawPtr;
pub type TextDraw_GetID_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_IsValid_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> bool;
pub type TextDraw_IsVisibleForPlayer_t =
    unsafe extern "C" fn(player: PlayerPtr, textdraw: TextDrawPtr) -> bool;
pub type TextDraw_SetLetterSize_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, size_x: f32, size_y: f32) -> bool;
pub type TextDraw_SetTextSize_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, size_x: f32, size_y: f32) -> bool;
pub type TextDraw_SetAlignment_t = unsafe extern "C" fn(textdraw: TextDrawPtr, alignment: i32) -> bool;
pub type TextDraw_SetColor_t = unsafe extern "C" fn(textdraw: TextDrawPtr, color: u32) -> bool;
pub type TextDraw_SetUseBox_t = unsafe extern "C" fn(textdraw: TextDrawPtr, use_: bool) -> bool;
pub type TextDraw_SetBoxColor_t = unsafe extern "C" fn(textdraw: TextDrawPtr, color: u32) -> bool;
pub type TextDraw_SetShadow_t = unsafe extern "C" fn(textdraw: TextDrawPtr, size: i32) -> bool;
pub type TextDraw_SetOutline_t = unsafe extern "C" fn(textdraw: TextDrawPtr, size: i32) -> bool;
pub type TextDraw_SetBackgroundColor_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, color: u32) -> bool;
pub type TextDraw_SetFont_t = unsafe extern "C" fn(textdraw: TextDrawPtr, font: i32) -> bool;
pub type TextDraw_SetProportional_t = unsafe extern "C" fn(textdraw: TextDrawPtr, set: bool) -> bool;
pub type TextDraw_SetSelectable_t = unsafe extern "C" fn(textdraw: TextDrawPtr, set: bool) -> bool;
pub type TextDraw_ShowForPlayer_t =
    unsafe extern "C" fn(player: PlayerPtr, textdraw: TextDrawPtr) -> bool;
pub type TextDraw_HideForPlayer_t =
    unsafe extern "C" fn(player: PlayerPtr, textdraw: TextDrawPtr) -> bool;
pub type TextDraw_ShowForAll_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> bool;
pub type TextDraw_HideForAll_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> bool;
pub type TextDraw_SetString_t = unsafe extern "C" fn(textdraw: TextDrawPtr, text: *const i8) -> bool;
pub type TextDraw_SetPreviewModel_t = unsafe extern "C" fn(textdraw: TextDrawPtr, model: i32) -> bool;
pub type TextDraw_SetPreviewRot_t = unsafe extern "C" fn(
    textdraw: TextDrawPtr,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    zoom: f32,
) -> bool;
pub type TextDraw_SetPreviewVehCol_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, color1: i32, color2: i32) -> bool;
pub type TextDraw_SetPos_t = unsafe extern "C" fn(textdraw: TextDrawPtr, x: f32, y: f32) -> bool;
pub type TextDraw_GetString_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, text: *mut CAPIStringView) -> bool;
pub type TextDraw_GetLetterSize_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, size_x: *mut f32, size_y: *mut f32) -> bool;
pub type TextDraw_GetTextSize_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, size_x: *mut f32, size_y: *mut f32) -> bool;
pub type TextDraw_GetPos_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, x: *mut f32, y: *mut f32) -> bool;
pub type TextDraw_GetColor_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_GetBoxColor_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_GetBackgroundColor_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_GetShadow_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_GetOutline_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_GetFont_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_IsBox_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> bool;
pub type TextDraw_IsProportional_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> bool;
pub type TextDraw_IsSelectable_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> bool;
pub type TextDraw_GetAlignment_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_GetPreviewModel_t = unsafe extern "C" fn(textdraw: TextDrawPtr) -> i32;
pub type TextDraw_GetPreviewRot_t = unsafe extern "C" fn(
    textdraw: TextDrawPtr,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    zoom: *mut f32,
) -> bool;
pub type TextDraw_GetPreviewVehColor_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, color1: *mut i32, color2: *mut i32) -> bool;
pub type TextDraw_SetStringForPlayer_t =
    unsafe extern "C" fn(textdraw: TextDrawPtr, player: PlayerPtr, text: *const i8) -> bool;

#[repr(C)]
pub struct TextDrawAPI {
    pub create: Option<TextDraw_Create_t>,
    pub destroy: Option<TextDraw_Destroy_t>,
    pub from_id: Option<TextDraw_FromID_t>,
    pub get_id: Option<TextDraw_GetID_t>,
    pub is_valid: Option<TextDraw_IsValid_t>,
    pub is_visible_for_player: Option<TextDraw_IsVisibleForPlayer_t>,
    pub set_letter_size: Option<TextDraw_SetLetterSize_t>,
    pub set_text_size: Option<TextDraw_SetTextSize_t>,
    pub set_alignment: Option<TextDraw_SetAlignment_t>,
    pub set_color: Option<TextDraw_SetColor_t>,
    pub set_use_box: Option<TextDraw_SetUseBox_t>,
    pub set_box_color: Option<TextDraw_SetBoxColor_t>,
    pub set_shadow: Option<TextDraw_SetShadow_t>,
    pub set_outline: Option<TextDraw_SetOutline_t>,
    pub set_background_color: Option<TextDraw_SetBackgroundColor_t>,
    pub set_font: Option<TextDraw_SetFont_t>,
    pub set_proportional: Option<TextDraw_SetProportional_t>,
    pub set_selectable: Option<TextDraw_SetSelectable_t>,
    pub show_for_player: Option<TextDraw_ShowForPlayer_t>,
    pub hide_for_player: Option<TextDraw_HideForPlayer_t>,
    pub show_for_all: Option<TextDraw_ShowForAll_t>,
    pub hide_for_all: Option<TextDraw_HideForAll_t>,
    pub set_string: Option<TextDraw_SetString_t>,
    pub set_preview_model: Option<TextDraw_SetPreviewModel_t>,
    pub set_preview_rot: Option<TextDraw_SetPreviewRot_t>,
    pub set_preview_veh_col: Option<TextDraw_SetPreviewVehCol_t>,
    pub set_pos: Option<TextDraw_SetPos_t>,
    pub get_string: Option<TextDraw_GetString_t>,
    pub get_letter_size: Option<TextDraw_GetLetterSize_t>,
    pub get_text_size: Option<TextDraw_GetTextSize_t>,
    pub get_pos: Option<TextDraw_GetPos_t>,
    pub get_color: Option<TextDraw_GetColor_t>,
    pub get_box_color: Option<TextDraw_GetBoxColor_t>,
    pub get_background_color: Option<TextDraw_GetBackgroundColor_t>,
    pub get_shadow: Option<TextDraw_GetShadow_t>,
    pub get_outline: Option<TextDraw_GetOutline_t>,
    pub get_font: Option<TextDraw_GetFont_t>,
    pub is_box: Option<TextDraw_IsBox_t>,
    pub is_proportional: Option<TextDraw_IsProportional_t>,
    pub is_selectable: Option<TextDraw_IsSelectable_t>,
    pub get_alignment: Option<TextDraw_GetAlignment_t>,
    pub get_preview_model: Option<TextDraw_GetPreviewModel_t>,
    pub get_preview_rot: Option<TextDraw_GetPreviewRot_t>,
    pub get_preview_veh_color: Option<TextDraw_GetPreviewVehColor_t>,
    pub set_string_for_player: Option<TextDraw_SetStringForPlayer_t>,
}

impl Default for TextDrawAPI {
    fn default() -> Self {
        Self {
            create: None,
            destroy: None,
            from_id: None,
            get_id: None,
            is_valid: None,
            is_visible_for_player: None,
            set_letter_size: None,
            set_text_size: None,
            set_alignment: None,
            set_color: None,
            set_use_box: None,
            set_box_color: None,
            set_shadow: None,
            set_outline: None,
            set_background_color: None,
            set_font: None,
            set_proportional: None,
            set_selectable: None,
            show_for_player: None,
            hide_for_player: None,
            show_for_all: None,
            hide_for_all: None,
            set_string: None,
            set_preview_model: None,
            set_preview_rot: None,
            set_preview_veh_col: None,
            set_pos: None,
            get_string: None,
            get_letter_size: None,
            get_text_size: None,
            get_pos: None,
            get_color: None,
            get_box_color: None,
            get_background_color: None,
            get_shadow: None,
            get_outline: None,
            get_font: None,
            is_box: None,
            is_proportional: None,
            is_selectable: None,
            get_alignment: None,
            get_preview_model: None,
            get_preview_rot: None,
            get_preview_veh_color: None,
            set_string_for_player: None,
        }
    }
}
