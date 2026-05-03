//! Eager copy of [`crate::GlobalSettings`] accessors (axes, units, time span, frame rate) for
//! [`crate::OwnedDocument`] without holding a full [`crate::Document`].

use crate::GlobalSettings;
use crate::FrameRate;

#[derive(Debug, Clone, PartialEq)]
pub struct OwnedGlobalSettings {
    pub up_axis: i32,
    pub up_axis_sign: i32,
    pub front_axis: i32,
    pub front_axis_sign: i32,
    pub coord_axis: i32,
    pub coord_axis_sign: i32,
    pub original_up_axis: i32,
    pub original_up_axis_sign: i32,
    pub unit_scale_factor: f32,
    pub original_unit_scale_factor: f32,
    pub ambient_color: [f32; 3],
    pub default_camera: String,
    pub time_span_start: u64,
    pub time_span_stop: u64,
    pub custom_frame_rate: f32,
    pub frame_rate: FrameRate,
}

impl Default for OwnedGlobalSettings {
    fn default() -> Self {
        Self {
            up_axis: 1,
            up_axis_sign: 1,
            front_axis: 2,
            front_axis_sign: 1,
            coord_axis: 0,
            coord_axis_sign: 1,
            original_up_axis: -1,
            original_up_axis_sign: 1,
            unit_scale_factor: 1.0,
            original_unit_scale_factor: 1.0,
            ambient_color: [0.0, 0.0, 0.0],
            default_camera: "".to_string(),
            time_span_start: 0,
            time_span_stop: 0,
            custom_frame_rate: -1.0,
            frame_rate: FrameRate::Default,
        }
    }
}

impl<'a> From<GlobalSettings<'a>> for OwnedGlobalSettings {
    fn from(global_settings: GlobalSettings<'a>) -> Self {
        Self {
            up_axis: global_settings.up_axis(),
            up_axis_sign: global_settings.up_axis_sign(),
            front_axis: global_settings.front_axis(),
            front_axis_sign: global_settings.front_axis_sign(),
            coord_axis: global_settings.coord_axis(),
            coord_axis_sign: global_settings.coord_axis_sign(),
            original_up_axis: global_settings.original_up_axis(),
            original_up_axis_sign: global_settings.original_up_axis_sign(),
            unit_scale_factor: global_settings.unit_scale_factor(),
            original_unit_scale_factor: global_settings.original_unit_scale_factor(),
            ambient_color: global_settings.ambient_color(),
            default_camera: global_settings.default_camera(),
            time_span_start: global_settings.time_span_start(),
            time_span_stop: global_settings.time_span_stop(),
            custom_frame_rate: global_settings.custom_frame_rate(),
            frame_rate: global_settings.frame_rate(),
        }
    }
}
