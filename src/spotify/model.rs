use std::fmt;
// album_type - ‘album’, ‘single’, ‘appears_on’, ‘compilation’
// #[derive(Debug)]
pub enum ALBUM_TYPE {
    ALBUM,
    SINGLE,
    APPEARS_ON,
    COMPILCATION,
}
impl ALBUM_TYPE {
    pub fn from_str(s: &str) -> Option<ALBUM_TYPE> {
        match s {
            "album" => Some(ALBUM_TYPE::ALBUM),
            "single" => Some(ALBUM_TYPE::SINGLE),
            "appears_on" => Some(ALBUM_TYPE::APPEARS_ON),
            "compilation" => Some(ALBUM_TYPE::COMPILCATION),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            &ALBUM_TYPE::ALBUM => "album",
            &ALBUM_TYPE::SINGLE => "single",
            &ALBUM_TYPE::APPEARS_ON => "appears_on",
            &ALBUM_TYPE::COMPILCATION => "compilation",
        }
    }
}
impl fmt::Debug for ALBUM_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ALBUM_TYPE::ALBUM => write!(f, "album"),
            ALBUM_TYPE::SINGLE => write!(f, "single"),
            ALBUM_TYPE::APPEARS_ON => write!(f, "appears_on"),
            ALBUM_TYPE::COMPILCATION => write!(f, "compilation"),
        }
    }
}
