use crate::document::{Document, Property};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    #[default]
    Default,
    Fps120,
    Fps100,
    Fps60,
    Fps50,
    Fps48,
    Fps30,
    Fps30Drop,
    NtsDropFrame,
    NtsFullFrame,
    Pal,
    Cinema,
    Fps1000,
    CinemaNd,
    Custom,
    Max,
}

#[derive(Debug)]
pub struct GlobalSettings<'a> {
    document: &'a Document,
    global_settings: &'a HashMap<String, Property>,
}

impl<'a> GlobalSettings<'a> {
    pub fn new(document: &'a Document, global_settings: &'a HashMap<String, Property>) -> Self {
        Self {
            document,
            global_settings,
        }
    }
    
    pub fn document(&self) -> &'a Document {
        self.document
    }

    pub fn global_settings(&self) -> &'a HashMap<String, Property> {
        self.global_settings
    }

    pub fn up_axis(&self) -> i32 {
        self.global_settings
            .get("UpAxis")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(1)
    }

    pub fn up_axis_sign(&self) -> i32 {
        self.global_settings
            .get("UpAxisSign")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(1)
    }

    pub fn front_axis(&self) -> i32 {
        self.global_settings
            .get("FrontAxis")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(2)
    }

    pub fn front_axis_sign(&self) -> i32 {
        self.global_settings
            .get("FrontAxisSign")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(1)
    }

    pub fn coord_axis(&self) -> i32 {
        self.global_settings
            .get("CoordAxis")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(0)
    }

    pub fn coord_axis_sign(&self) -> i32 {
        self.global_settings
            .get("CoordAxisSign")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(1)
    }

    pub fn original_up_axis(&self) -> i32 {
        self.global_settings
            .get("OriginalUpAxis")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(0)
    }

    pub fn original_up_axis_sign(&self) -> i32 {
        self.global_settings
            .get("OriginalUpAxisSign")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(1)
    }

    pub fn unit_scale_factor(&self) -> f32 {
        self.global_settings
            .get("UnitScaleFactor")
            .and_then(|prop| match prop {
                Property::Float(f) => Some(*f),
                _ => None,
            })
            .unwrap_or(1.0)
    }

    pub fn original_unit_scale_factor(&self) -> f32 {
        self.global_settings
            .get("OriginalUnitScaleFactor")
            .and_then(|prop| match prop {
                Property::Float(f) => Some(*f),
                _ => None,
            })
            .unwrap_or(1.0)
    }

    pub fn ambient_color(&self) -> [f32; 3] {
        self.global_settings
            .get("AmbientColor")
            .and_then(|prop| match prop {
                Property::Vec3(v) => Some(*v),
                _ => None,
            })
            .unwrap_or([0.0, 0.0, 0.0])
    }

    pub fn default_camera(&self) -> String {
        self.global_settings
            .get("DefaultCamera")
            .and_then(|prop| match prop {
                Property::String(s) => Some(s.to_string()),
                _ => None,
            })
            .unwrap_or("".to_string())
    }

    pub fn time_span_start(&self) -> u64 {
        self.global_settings
            .get("TimeSpanStart")
            .and_then(|prop| match prop {
                Property::ULongLong(u) => Some(*u),
                _ => None,
            })
            .unwrap_or(0)
    }

    pub fn time_span_stop(&self) -> u64 {
        self.global_settings
            .get("TimeSpanStop")
            .and_then(|prop| match prop {
                Property::ULongLong(u) => Some(*u),
                _ => None,
            })
            .unwrap_or(0)
    }

    pub fn custom_frame_rate(&self) -> f32 {
        self.global_settings
            .get("CustomFrameRate")
            .and_then(|prop| match prop {
                Property::Float(f) => Some(*f),
                _ => None,
            })
            .unwrap_or(-1.0)
    }

    pub fn frame_rate(&self) -> FrameRate {
        let i = self
            .global_settings
            .get("TimeMode")
            .and_then(|prop| match prop {
                Property::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(0);
        match i {
            0 => FrameRate::Default,
            1 => FrameRate::Fps120,
            2 => FrameRate::Fps100,
            3 => FrameRate::Fps60,
            4 => FrameRate::Fps50,
            5 => FrameRate::Fps48,
            6 => FrameRate::Fps30,
            7 => FrameRate::Fps30Drop,
            8 => FrameRate::NtsDropFrame,
            9 => FrameRate::NtsFullFrame,
            10 => FrameRate::Pal,
            11 => FrameRate::Cinema,
            12 => FrameRate::Fps1000,
            13 => FrameRate::CinemaNd,
            14 => FrameRate::Custom,
            15 => FrameRate::Max,
            _ => FrameRate::Default,
        }
    }
}
