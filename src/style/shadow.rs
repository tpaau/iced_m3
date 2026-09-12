use iced::{Color, Shadow, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Elevation {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
}

impl Default for Elevation {
    fn default() -> Self {
        Self::Level0
    }
}

impl Elevation {
    pub const fn level(self) -> u8 {
        self as u8
    }

    /// Material 3 surface elevation values in dp.
    pub const fn dp(self) -> f32 {
        match self {
            Self::Level0 => 0.0,
            Self::Level1 => 1.0,
            Self::Level2 => 3.0,
            Self::Level3 => 6.0,
            Self::Level4 => 8.0,
            Self::Level5 => 12.0,
        }
    }

    /// Shadow offset for iced's single-shadow model.
    const fn offset(self) -> Vector {
        match self {
            Self::Level0 => Vector::new(0.0, 0.0),
            Self::Level1 => Vector::new(0.0, 1.0),
            Self::Level2 => Vector::new(0.0, 2.0),
            Self::Level3 => Vector::new(0.0, 4.0),
            Self::Level4 => Vector::new(0.0, 6.0),
            Self::Level5 => Vector::new(0.0, 8.0),
        }
    }

    /// Shadow blur values approximating M3's restrained shadows.
    const fn blur_radius(self) -> f32 {
        match self {
            Self::Level0 => 0.0,
            Self::Level1 => 3.0,
            Self::Level2 => 6.0,
            Self::Level3 => 8.0,
            Self::Level4 => 10.0,
            Self::Level5 => 12.0,
        }
    }

    /// Shadow alpha for the single-shadow iced representation.
    const fn alpha(self) -> f32 {
        match self {
            Self::Level0 => 0.0,
            Self::Level1 => 0.12,
            Self::Level2 => 0.14,
            Self::Level3 => 0.16,
            Self::Level4 => 0.18,
            Self::Level5 => 0.20,
        }
    }
}

pub fn shadow(color: Color, elevation: Elevation) -> Shadow {
    Shadow {
        color: color.scale_alpha(elevation.alpha()),
        offset: elevation.offset(),
        blur_radius: elevation.blur_radius(),
    }
}
