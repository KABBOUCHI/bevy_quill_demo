mod button;

use bevy::color::Srgba;
use bevy::prelude::App;
use bevy::prelude::Plugin;
use bevy::prelude::Resource;

pub use button::*;

#[derive(Default, Clone, PartialEq)]
pub enum ThemeMode {
    #[default]
    DARK,
    LIGHT,
}

#[derive(Default, Clone, PartialEq)]
pub struct ColorScales {
    pub primary: [Srgba; 11],
    pub gray: [Srgba; 11],

    pub green: [Srgba; 11],
    pub indigo: [Srgba; 11],
    pub blue: [Srgba; 11],

    pub slate: [Srgba; 11],
    pub cool: [Srgba; 11],
    pub zinc: [Srgba; 11],
    pub neutral: [Srgba; 11],
    pub stone: [Srgba; 11],

    pub white: Srgba,
    pub black: Srgba,
}

fn into_colors(colors: [&str; 11]) -> [Srgba; 11] {
    [
        Srgba::hex(colors[0]).unwrap(),
        Srgba::hex(colors[1]).unwrap(),
        Srgba::hex(colors[2]).unwrap(),
        Srgba::hex(colors[3]).unwrap(),
        Srgba::hex(colors[4]).unwrap(),
        Srgba::hex(colors[5]).unwrap(),
        Srgba::hex(colors[6]).unwrap(),
        Srgba::hex(colors[7]).unwrap(),
        Srgba::hex(colors[8]).unwrap(),
        Srgba::hex(colors[9]).unwrap(),
        Srgba::hex(colors[10]).unwrap(),
    ]
}

#[derive(Resource, Clone, PartialEq)]
pub struct QuillUiTheme {
    pub mode: ThemeMode,
    pub colors: ColorScales,
}

impl Default for QuillUiTheme {
    fn default() -> Self {
        let green = into_colors([
            "#f0fdf4", "#dcfce7", "#bbf7d0", "#86efac", "#4ade80", "#22c55e", "#16a34a", "#15803d",
            "#166534", "#14532d", "#052e16",
        ]);

        let zinc = into_colors([
            "#fafafa", "#f5f5f5", "#e5e5e5", "#d4d4d4", "#a3a3a3", "#737373", "#525252", "#404040",
            "#262626", "#171717", "#0a0a0a",
        ]);

        Self {
            mode: ThemeMode::DARK,
            colors: ColorScales {
                primary: green,
                gray: zinc,

                green,
                indigo: into_colors([
                    "#eef2ff", "#e0e7ff", "#c7d2fe", "#a5b4fc", "#818cf8", "#6366f1", "#4f46e5",
                    "#4338ca", "#3730a3", "#312e81", "#1e1b4b",
                ]),
                blue: into_colors([
                    "#eff6ff", "#dbeafe", "#bfdbfe", "#93c5fd", "#60a5fa", "#3b82f6", "#2563eb",
                    "#1d4ed8", "#1e40af", "#1e3a8a", "#172554",
                ]),

                zinc,
                slate: into_colors([
                    "#f8fafc", "#f1f5f9", "#e2e8f0", "#cbd5e1", "#94a3b8", "#64748b", "#475569",
                    "#334155", "#1e293b", "#0f172a", "#020617",
                ]),

                neutral: into_colors([
                    "#fafafa", "#f5f5f5", "#e5e5e5", "#d4d4d4", "#a3a3a3", "#737373", "#525252",
                    "#404040", "#262626", "#171717", "#0a0a0a",
                ]),

                stone: into_colors([
                    "#fafaf9", "#f5f5f4", "#e7e5e4", "#d6d3d1", "#a8a29e", "#78716c", "#57534e",
                    "#44403c", "#292524", "#1c1917", "#0c0a09",
                ]),

                cool: into_colors([
                    "#f9fafb", "#f3f4f6", "#e5e7eb", "#d1d5db", "#9ca3af", "#6b7280", "#4b5563",
                    "#374151", "#1f2937", "#111827", "#030712",
                ]),

                white: Srgba::WHITE,
                black: Srgba::BLACK,
            },
        }
    }
}

pub struct QuillUiPlugin;

impl Plugin for QuillUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuillUiTheme>();
    }
}
