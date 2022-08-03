use into_value::{impl_into_value, IntoValue};

macro_rules! impl_api_arg {
    ($t:ident) => {
        impl From<$t> for rmpv::Value {
            fn from(t: $t) -> Self {
                t.0
            }
        }

        impl_into_value!($t);

        impl IntoValue for &$t {
            fn into_value(self) -> rmpv::Value {
                self.0.clone().into()
            }
        }

        impl From<rmpv::Value> for $t {
            fn from(v: rmpv::Value) -> $t {
                $t(v)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ShowTabline {
    #[default]
    Never,
    MoreThanOne,
    Always,
}

#[derive(Debug)]
pub enum OptionSet {
    Guifont(String),
    Linespace(i64),
    ShowTabline(ShowTabline),
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for OptionSet {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let data = rmpv::Value::deserialize(d)?;

        let bad_value = || serde::de::Error::missing_field("value");

        let name = data[0]
            .as_str()
            .ok_or_else(|| serde::de::Error::custom("bad name"))?;

        match name {
            "linespace" => Ok(Self::Linespace(data[1].as_i64().ok_or_else(bad_value)?)),
            "guifont" => Ok(Self::Guifont(
                data[1].as_str().ok_or_else(bad_value)?.to_string(),
            )),
            "showtabline" => Ok(Self::ShowTabline(
                data[1]
                    .as_i64()
                    .ok_or_else(bad_value)
                    .and_then(|v| match v {
                        0 => Ok(ShowTabline::Never),
                        1 => Ok(ShowTabline::MoreThanOne),
                        2 => Ok(ShowTabline::Always),
                        _ => Err(serde::de::Error::custom(format!(
                            "unexpected showtabline value: {:?}",
                            v,
                        ))),
                    })?,
            )),
            _ => Ok(Self::Unknown(name.to_string())),
        }
    }
}

#[derive(Debug, Default, into_value_proc::IntoValue)]
pub struct UiOptions {
    pub rgb: bool,
    pub r#override: bool,
    pub ext_cmdline: bool,
    pub ext_hlstate: bool,
    pub ext_linegrid: bool,
    pub ext_messages: bool,
    pub ext_multigrid: bool,
    pub ext_popupmenu: bool,
    pub ext_tabline: bool,
    pub ext_termcolors: bool,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct HlAttr {
    pub foreground: Option<i64>,
    pub background: Option<i64>,
    pub special: Option<i64>,
    pub reverse: Option<bool>,
    pub italic: Option<bool>,
    pub bold: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<bool>,
    pub underlineline: Option<bool>,
    pub undercurl: Option<bool>,
    pub underdot: Option<bool>,
    pub underdash: Option<bool>,
    pub blend: Option<i64>,
}

#[derive(Debug, Default)]
pub struct GridLineData {
    pub text: String,
    pub hl_id: Option<i64>,
    pub repeat: Option<i64>,
}

impl<'de> serde::Deserialize<'de> for GridLineData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let data = rmpv::Value::deserialize(d)?;

        let text = data[0]
            .as_str()
            .ok_or_else(|| serde::de::Error::custom(format!("bad text field: {:?}", data[0])))?;
        let hl_id = data[1].as_i64();
        let repeat = data[2].as_i64();

        Ok(Self {
            text: text.to_string(),
            hl_id,
            repeat,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CursorShape {
    Block,
    Horizontal,
    Vertical,
}

impl Default for CursorShape {
    fn default() -> Self {
        Self::Block
    }
}

impl<'de> serde::Deserialize<'de> for CursorShape {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let data = rmpv::Value::deserialize(d)?;

        match data.as_str() {
            Some("block") => Ok(Self::Block),
            Some("horizontal") => Ok(Self::Horizontal),
            Some("vertical") => Ok(Self::Vertical),
            Some(v) => Err(serde::de::Error::custom(format!(
                "unknown cursor shape: {}",
                v
            ))),
            None => Err(serde::de::Error::custom("missing value for cursor shape")),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct ModeInfo {
    pub cursor_shape: Option<CursorShape>,
    pub cell_percentage: Option<u64>,
    pub blinkwait: Option<u64>,
    pub blinkon: Option<u64>,
    pub blinkoff: Option<u64>,
    pub attr_id: Option<u64>,
    pub attr_id_lm: Option<u64>,
    pub short_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct CmdlineContent {
    pub hl_id: i64,
    pub text: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Window(rmpv::Value);

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Buffer(rmpv::Value);

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct Tabpage(rmpv::Value);

#[derive(Debug, serde::Deserialize)]
pub struct TablineTab {
    pub name: String,
    pub tab: Tabpage,
}

#[derive(Debug, serde::Deserialize)]
pub struct TablineBuffer {
    pub name: String,
    pub buffer: Buffer,
}

#[derive(Default, Debug, serde::Deserialize)]
pub struct PopupmenuItem {
    pub word: String,
    pub kind: String,
    pub menu: String,
    pub info: String,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Dictionary(rmpv::Value);

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct LuaRef(rmpv::Value);

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Object(rmpv::Value);

impl_api_arg!(Dictionary);
impl_api_arg!(LuaRef);
impl_api_arg!(Object);
impl_api_arg!(Window);
impl_api_arg!(Buffer);
impl_api_arg!(Tabpage);
