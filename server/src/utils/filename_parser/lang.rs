pub type LANG = &'static str;

pub const LANG_ZH: LANG = "zh";
pub const LANG_JP: LANG = "jp";
pub const LANG_EN: LANG = "en";
pub const LANG_ZH_CN: LANG = "zh-CN";
pub const LANG_ZH_TW: LANG = "zh-TW";
pub const LANG_UNKNOWN: LANG = "";

pub(super) fn from(lang: lingua::Language) -> LANG {
    match lang {
        lingua::Language::Chinese => LANG_ZH,
        lingua::Language::English => LANG_EN,
        lingua::Language::Japanese => LANG_JP,
    }
}
