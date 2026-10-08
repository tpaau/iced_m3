use iced_widget::canvas::Path;

use crate::{
    animation::Interpolable,
    widget::shapes::{
        Command,
        morph::{MorphPlan, MorphPlanError},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Point> for iced::Point {
    fn from(value: Point) -> Self {
        iced::Point::new(value.x, value.y)
    }
}

impl Interpolable for Point {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x.interpolate(other.x, t),
            y: self.y.interpolate(other.y, t),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cubic {
    pub anchor_0: Point,
    pub control_0: Point,
    pub control_1: Point,
    pub anchor_1: Point,
}

impl Interpolable for Cubic {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            anchor_0: self.anchor_0.interpolate(other.anchor_0, t),
            control_0: self.control_0.interpolate(other.control_0, t),
            control_1: self.control_1.interpolate(other.control_1, t),
            anchor_1: self.anchor_1.interpolate(other.anchor_1, t),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureKind {
    Edge,
    ConvexCorner,
    ConcaveCorner,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Feature {
    pub kind: FeatureKind,
    pub cubics: Vec<Cubic>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ShapeData {
    pub features: Vec<Feature>,
}

impl ShapeData {
    fn build_path(commands: &[Command]) -> Path {
        Path::new(|builder| {
            for command in commands {
                match *command {
                    Command::MoveTo(point) => {
                        builder.move_to(point.into());
                    }

                    Command::LineTo(point) => {
                        builder.line_to(point.into());
                    }

                    Command::QuadTo { control, to } => {
                        builder.quadratic_curve_to(control.into(), to.into());
                    }

                    Command::CubicTo {
                        control_1,
                        control_2,
                        to,
                    } => {
                        builder.bezier_curve_to(control_1.into(), control_2.into(), to.into());
                    }

                    Command::Close => {
                        builder.close();
                    }
                }
            }
        })
    }

    // TODO: Can be sped up
    pub fn as_commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();
        let mut first = true;

        for feature in &self.features {
            for cubic in &feature.cubics {
                if first {
                    commands.push(Command::MoveTo(cubic.anchor_0));
                    first = false;
                }

                commands.push(Command::CubicTo {
                    control_1: cubic.control_0,
                    control_2: cubic.control_1,
                    to: cubic.anchor_1,
                });
            }
        }

        commands.push(Command::Close);
        commands
    }

    pub fn path(&self) -> Path {
        Self::build_path(&self.as_commands())
    }

    pub fn morph(&self, to: &Self) -> Result<MorphPlan, MorphPlanError> {
        MorphPlan::new(self, to)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FeatureStored<'a> {
    pub kind: FeatureKind,
    pub cubics: &'a [Cubic],
}

impl From<FeatureStored<'_>> for Feature {
    fn from(value: FeatureStored<'_>) -> Self {
        Self {
            kind: value.kind,
            cubics: value.cubics.into_iter().copied().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ShapeDataStored<'a> {
    pub features: &'a [FeatureStored<'a>],
}

impl From<ShapeDataStored<'_>> for ShapeData {
    fn from(value: ShapeDataStored<'_>) -> Self {
        Self {
            features: value
                .features
                .into_iter()
                .copied()
                .map(|f| f.into())
                .collect(),
        }
    }
}

static ARCH_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.14644665f32, 0.1464466f32),
        control_0: Point::new(0.23692884f32, 0.05596439f32),
        control_1: Point::new(0.36192882f32, -2.1073424E-8f32),
        anchor_1: Point::new(0.5f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.5f32, 0.0f32),
        control_0: Point::new(0.7761423f32, 0.0f32),
        control_1: Point::new(1.0f32, 0.22385772f32),
        anchor_1: Point::new(0.99999994f32, 0.50000006f32),
    },
    Cubic {
        anchor_0: Point::new(0.99999994f32, 0.50000006f32),
        control_0: Point::new(0.99999994f32, 0.61952627f32),
        control_1: Point::new(1.0f32, 0.7390524f32),
        anchor_1: Point::new(1.0f32, 0.8585787f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.8585787f32),
        control_0: Point::new(0.99999994f32, 0.93668354f32),
        control_1: Point::new(0.93668354f32, 1.0f32),
        anchor_1: Point::new(0.8585787f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.8585787f32, 1.0f32),
        control_0: Point::new(0.61952615f32, 0.99999994f32),
        control_1: Point::new(0.38047376f32, 1.0f32),
        anchor_1: Point::new(0.14142135f32, 0.99999994f32),
    },
    Cubic {
        anchor_0: Point::new(0.14142135f32, 0.99999994f32),
        control_0: Point::new(0.063316494f32, 0.99999994f32),
        control_1: Point::new(0.0f32, 0.93668336f32),
        anchor_1: Point::new(0.0f32, 0.8585785f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.8585785f32),
        control_0: Point::new(0.0f32, 0.7390524f32),
        control_1: Point::new(4.2146848E-8f32, 0.61952615f32),
        anchor_1: Point::new(0.0f32, 0.49999997f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.49999997f32),
        control_0: Point::new(2.1073424E-8f32, 0.3619288f32),
        control_1: Point::new(0.055964436f32, 0.23692879f32),
        anchor_1: Point::new(0.14644665f32, 0.1464466f32),
    },
];
static ARCH_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: ARCH_CUBICS,
}];
pub const ARCH: ShapeDataStored = ShapeDataStored {
    features: ARCH_FEATURES,
};

static ARROW_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.49938738f32, 0.8368627f32),
        control_0: Point::new(0.47894177f32, 0.8367552f32),
        control_1: Point::new(0.45848423f32, 0.838917f32),
        anchor_1: Point::new(0.43839493f32, 0.84335023f32),
    },
    Cubic {
        anchor_0: Point::new(0.43839493f32, 0.84335023f32),
        control_0: Point::new(0.38492194f32, 0.85515004f32),
        control_1: Point::new(0.33144897f32, 0.86695f32),
        anchor_1: Point::new(0.27797604f32, 0.87874985f32),
    },
    Cubic {
        anchor_0: Point::new(0.27797604f32, 0.87874985f32),
        control_0: Point::new(0.12195567f32, 0.9131789f32),
        control_1: Point::new(0.0f32, 0.7449043f32),
        anchor_1: Point::new(0.08128172f32, 0.6073506f32),
    },
    Cubic {
        anchor_0: Point::new(0.08128172f32, 0.6073506f32),
        control_0: Point::new(0.11181903f32, 0.5556721f32),
        control_1: Point::new(0.14235632f32, 0.50399363f32),
        anchor_1: Point::new(0.17289363f32, 0.45231515f32),
    },
    Cubic {
        anchor_0: Point::new(0.17289363f32, 0.45231515f32),
        control_0: Point::new(0.31810787f32, 0.20656797f32),
        control_1: Point::new(0.390715f32, 0.0836944f32),
        anchor_1: Point::new(0.4988227f32, 0.08352743f32),
    },
    Cubic {
        anchor_0: Point::new(0.4988227f32, 0.08352743f32),
        control_0: Point::new(0.60693026f32, 0.08336049f32),
        control_1: Point::new(0.67991644f32, 0.20600939f32),
        anchor_1: Point::new(0.82588875f32, 0.45130718f32),
    },
    Cubic {
        anchor_0: Point::new(0.82588875f32, 0.45130718f32),
        control_0: Point::new(0.8559578f32, 0.5018366f32),
        control_1: Point::new(0.88602704f32, 0.552366f32),
        anchor_1: Point::new(0.9160962f32, 0.6028954f32),
    },
    Cubic {
        anchor_0: Point::new(0.9160962f32, 0.6028954f32),
        control_0: Point::new(1.0f32, 0.7438909f32),
        control_1: Point::new(0.87382054f32, 0.9166395f32),
        anchor_1: Point::new(0.7139839f32, 0.8796016f32),
    },
    Cubic {
        anchor_0: Point::new(0.7139839f32, 0.8796016f32),
        control_0: Point::new(0.66275865f32, 0.86773145f32),
        control_1: Point::new(0.61153334f32, 0.8558613f32),
        anchor_1: Point::new(0.56030816f32, 0.84399116f32),
    },
    Cubic {
        anchor_0: Point::new(0.56030816f32, 0.84399116f32),
        control_0: Point::new(0.54026663f32, 0.839347f32),
        control_1: Point::new(0.51983297f32, 0.8369702f32),
        anchor_1: Point::new(0.49938738f32, 0.8368627f32),
    },
];
static ARROW_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: ARROW_CUBICS,
}];
pub const ARROW: ShapeDataStored = ShapeDataStored {
    features: ARROW_FEATURES,
};

static BOOM_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.4541391f32, 0.2874297f32),
        control_0: Point::new(0.45683354f32, 0.28685427f32),
        control_1: Point::new(0.45920122f32, 0.2847487f32),
        anchor_1: Point::new(0.45960853f32, 0.28146183f32),
    },
    Cubic {
        anchor_0: Point::new(0.45960853f32, 0.28146183f32),
        control_0: Point::new(0.4707847f32, 0.19127269f32),
        control_1: Point::new(0.4819609f32, 0.10108354f32),
        anchor_1: Point::new(0.49313703f32, 0.010894388f32),
    },
    Cubic {
        anchor_0: Point::new(0.49313703f32, 0.010894388f32),
        control_0: Point::new(0.49414754f32, 0.0027398954f32),
        control_1: Point::new(0.5059636f32, 0.0027286115f32),
        anchor_1: Point::new(0.5069897f32, 0.010881159f32),
    },
    Cubic {
        anchor_0: Point::new(0.5069897f32, 0.010881159f32),
        control_0: Point::new(0.5183478f32, 0.1011252f32),
        control_1: Point::new(0.5297059f32, 0.19136924f32),
        anchor_1: Point::new(0.541064f32, 0.28161326f32),
    },
    Cubic {
        anchor_0: Point::new(0.541064f32, 0.28161326f32),
        control_0: Point::new(0.54189116f32, 0.28818542f32),
        control_1: Point::new(0.5505508f32, 0.2900174f32),
        anchor_1: Point::new(0.5539687f32, 0.28434336f32),
    },
    Cubic {
        anchor_0: Point::new(0.5539687f32, 0.28434336f32),
        control_0: Point::new(0.6008619f32, 0.20649722f32),
        control_1: Point::new(0.6477551f32, 0.1286511f32),
        anchor_1: Point::new(0.69464827f32, 0.050805f32),
    },
    Cubic {
        anchor_0: Point::new(0.69464827f32, 0.050805f32),
        control_0: Point::new(0.6988881f32, 0.043766446f32),
        control_1: Point::new(0.70968735f32, 0.048562206f32),
        anchor_1: Point::new(0.7073087f32, 0.05642731f32),
    },
    Cubic {
        anchor_0: Point::new(0.7073087f32, 0.05642731f32),
        control_0: Point::new(0.6809793f32, 0.1434891f32),
        control_1: Point::new(0.6546499f32, 0.23055086f32),
        anchor_1: Point::new(0.6283205f32, 0.31761262f32),
    },
    Cubic {
        anchor_0: Point::new(0.6283205f32, 0.31761262f32),
        control_0: Point::new(0.62640303f32, 0.323953f32),
        control_1: Point::new(0.6335688f32, 0.3291488f32),
        anchor_1: Point::new(0.6389991f32, 0.3253555f32),
    },
    Cubic {
        anchor_0: Point::new(0.6389991f32, 0.3253555f32),
        control_0: Point::new(0.7135009f32, 0.2733127f32),
        control_1: Point::new(0.7880029f32, 0.2212699f32),
        anchor_1: Point::new(0.86250484f32, 0.16922714f32),
    },
    Cubic {
        anchor_0: Point::new(0.86250484f32, 0.16922714f32),
        control_0: Point::new(0.869241f32, 0.1645216f32),
        control_1: Point::new(0.87715596f32, 0.1732952f32),
        anchor_1: Point::new(0.8717839f32, 0.17951286f32),
    },
    Cubic {
        anchor_0: Point::new(0.8717839f32, 0.17951286f32),
        control_0: Point::new(0.8123195f32, 0.2483386f32),
        control_1: Point::new(0.7528552f32, 0.31716433f32),
        anchor_1: Point::new(0.69339085f32, 0.38599005f32),
    },
    Cubic {
        anchor_0: Point::new(0.69339085f32, 0.38599005f32),
        control_0: Point::new(0.68906033f32, 0.39100245f32),
        control_1: Point::new(0.6934933f32, 0.39866358f32),
        anchor_1: Point::new(0.699997f32, 0.3974069f32),
    },
    Cubic {
        anchor_0: Point::new(0.699997f32, 0.3974069f32),
        control_0: Point::new(0.7892256f32, 0.38016608f32),
        control_1: Point::new(0.87845415f32, 0.3629253f32),
        anchor_1: Point::new(0.9676828f32, 0.3456845f32),
    },
    Cubic {
        anchor_0: Point::new(0.9676828f32, 0.3456845f32),
        control_0: Point::new(0.9757504f32, 0.34412563f32),
        control_1: Point::new(0.97941256f32, 0.35535997f32),
        anchor_1: Point::new(0.9719761f32, 0.35885513f32),
    },
    Cubic {
        anchor_0: Point::new(0.9719761f32, 0.35885513f32),
        control_0: Point::new(0.88965875f32, 0.3975442f32),
        control_1: Point::new(0.80734146f32, 0.43623337f32),
        anchor_1: Point::new(0.7250241f32, 0.47492248f32),
    },
    Cubic {
        anchor_0: Point::new(0.7250241f32, 0.47492248f32),
        control_0: Point::new(0.71902925f32, 0.4777401f32),
        control_1: Point::new(0.7199629f32, 0.4865419f32),
        anchor_1: Point::new(0.7264154f32, 0.48803926f32),
    },
    Cubic {
        anchor_0: Point::new(0.7264154f32, 0.48803926f32),
        control_0: Point::new(0.81494224f32, 0.5085815f32),
        control_1: Point::new(0.903469f32, 0.5291238f32),
        anchor_1: Point::new(0.9919958f32, 0.54966617f32),
    },
    Cubic {
        anchor_0: Point::new(0.9919958f32, 0.54966617f32),
        control_0: Point::new(1.0f32, 0.55152345f32),
        control_1: Point::new(0.9987762f32, 0.5632761f32),
        anchor_1: Point::new(0.990561f32, 0.5634444f32),
    },
    Cubic {
        anchor_0: Point::new(0.990561f32, 0.5634444f32),
        control_0: Point::new(0.8996241f32, 0.5653072f32),
        control_1: Point::new(0.80868727f32, 0.56716996f32),
        anchor_1: Point::new(0.71775043f32, 0.56903267f32),
    },
    Cubic {
        anchor_0: Point::new(0.71775043f32, 0.56903267f32),
        control_0: Point::new(0.71112776f32, 0.5691683f32),
        control_1: Point::new(0.7084006f32, 0.5775891f32),
        anchor_1: Point::new(0.7136864f32, 0.58158135f32),
    },
    Cubic {
        anchor_0: Point::new(0.7136864f32, 0.58158135f32),
        control_0: Point::new(0.7862044f32, 0.6363548f32),
        control_1: Point::new(0.8587224f32, 0.6911282f32),
        anchor_1: Point::new(0.9312404f32, 0.74590164f32),
    },
    Cubic {
        anchor_0: Point::new(0.9312404f32, 0.74590164f32),
        control_0: Point::new(0.93779707f32, 0.7508541f32),
        control_1: Point::new(0.9318988f32, 0.7610927f32),
        anchor_1: Point::new(0.9243254f32, 0.75790507f32),
    },
    Cubic {
        anchor_0: Point::new(0.9243254f32, 0.75790507f32),
        control_0: Point::new(0.8404928f32, 0.7226194f32),
        control_1: Point::new(0.75666016f32, 0.68733376f32),
        anchor_1: Point::new(0.6728275f32, 0.6520482f32),
    },
    Cubic {
        anchor_0: Point::new(0.6728275f32, 0.6520482f32),
        control_0: Point::new(0.66672224f32, 0.6494785f32),
        control_1: Point::new(0.6608059f32, 0.6560619f32),
        anchor_1: Point::new(0.6640108f32, 0.661859f32),
    },
    Cubic {
        anchor_0: Point::new(0.6640108f32, 0.661859f32),
        control_0: Point::new(0.7079809f32, 0.7413926f32),
        control_1: Point::new(0.7519511f32, 0.8209263f32),
        anchor_1: Point::new(0.79592115f32, 0.90046f32),
    },
    Cubic {
        anchor_0: Point::new(0.79592115f32, 0.90046f32),
        control_0: Point::new(0.7998968f32, 0.9076511f32),
        control_1: Point::new(0.7903439f32, 0.9146057f32),
        anchor_1: Point::new(0.7847218f32, 0.9086132f32),
    },
    Cubic {
        anchor_0: Point::new(0.7847218f32, 0.9086132f32),
        control_0: Point::new(0.7224888f32, 0.8422804f32),
        control_1: Point::new(0.6602559f32, 0.7759476f32),
        anchor_1: Point::new(0.59802294f32, 0.7096148f32),
    },
    Cubic {
        anchor_0: Point::new(0.59802294f32, 0.7096148f32),
        control_0: Point::new(0.5934907f32, 0.70478404f32),
        control_1: Point::new(0.5854082f32, 0.7083918f32),
        anchor_1: Point::new(0.5859781f32, 0.7149912f32),
    },
    Cubic {
        anchor_0: Point::new(0.5859781f32, 0.7149912f32),
        control_0: Point::new(0.59379745f32, 0.8055332f32),
        control_1: Point::new(0.6016169f32, 0.8960752f32),
        anchor_1: Point::new(0.60943633f32, 0.9866172f32),
    },
    Cubic {
        anchor_0: Point::new(0.60943633f32, 0.9866172f32),
        control_0: Point::new(0.61014336f32, 0.9948036f32),
        control_1: Point::new(0.59858775f32, 0.9972714f32),
        anchor_1: Point::new(0.5958891f32, 0.9895103f32),
    },
    Cubic {
        anchor_0: Point::new(0.5958891f32, 0.9895103f32),
        control_0: Point::new(0.5660164f32, 0.90359974f32),
        control_1: Point::new(0.5361437f32, 0.81768924f32),
        anchor_1: Point::new(0.506271f32, 0.73177874f32),
    },
    Cubic {
        anchor_0: Point::new(0.506271f32, 0.73177874f32),
        control_0: Point::new(0.5040955f32, 0.7255222f32),
        control_1: Point::new(0.4952443f32, 0.7255306f32),
        anchor_1: Point::new(0.49308074f32, 0.7317913f32),
    },
    Cubic {
        anchor_0: Point::new(0.49308074f32, 0.7317913f32),
        control_0: Point::new(0.46339735f32, 0.8176859f32),
        control_1: Point::new(0.433714f32, 0.9035806f32),
        anchor_1: Point::new(0.40403068f32, 0.98947513f32),
    },
    Cubic {
        anchor_0: Point::new(0.40403068f32, 0.98947513f32),
        control_0: Point::new(0.40134686f32, 0.9972413f32),
        control_1: Point::new(0.3897866f32, 0.99479574f32),
        anchor_1: Point::new(0.3904779f32, 0.98660797f32),
    },
    Cubic {
        anchor_0: Point::new(0.3904779f32, 0.98660797f32),
        control_0: Point::new(0.39813083f32, 0.8959745f32),
        control_1: Point::new(0.40578377f32, 0.80534106f32),
        anchor_1: Point::new(0.41343668f32, 0.7147077f32),
    },
    Cubic {
        anchor_0: Point::new(0.41343668f32, 0.7147077f32),
        control_0: Point::new(0.41399407f32, 0.7081071f32),
        control_1: Point::new(0.4059046f32, 0.70451474f32),
        anchor_1: Point::new(0.4013816f32, 0.70935416f32),
    },
    Cubic {
        anchor_0: Point::new(0.4013816f32, 0.70935416f32),
        control_0: Point::new(0.339328f32, 0.7757495f32),
        control_1: Point::new(0.2772744f32, 0.8421449f32),
        anchor_1: Point::new(0.21522087f32, 0.9085402f32),
    },
    Cubic {
        anchor_0: Point::new(0.21522087f32, 0.9085402f32),
        control_0: Point::new(0.20961027f32, 0.9145434f32),
        control_1: Point::new(0.20004416f32, 0.9076072f32),
        anchor_1: Point::new(0.20400599f32, 0.9004085f32),
    },
    Cubic {
        anchor_0: Point::new(0.20400599f32, 0.9004085f32),
        control_0: Point::new(0.24786124f32, 0.8207234f32),
        control_1: Point::new(0.2917165f32, 0.7410384f32),
        anchor_1: Point::new(0.33557174f32, 0.6613534f32),
    },
    Cubic {
        anchor_0: Point::new(0.33557174f32, 0.6613534f32),
        control_0: Point::new(0.33876556f32, 0.6555502f32),
        control_1: Point::new(0.33283666f32, 0.64897805f32),
        anchor_1: Point::new(0.32673633f32, 0.6515595f32),
    },
    Cubic {
        anchor_0: Point::new(0.32673633f32, 0.6515595f32),
        control_0: Point::new(0.24304214f32, 0.6869752f32),
        control_1: Point::new(0.15934794f32, 0.72239095f32),
        anchor_1: Point::new(0.07565376f32, 0.75780666f32),
    },
    Cubic {
        anchor_0: Point::new(0.07565376f32, 0.75780666f32),
        control_0: Point::new(0.06808649f32, 0.76100874f32),
        control_1: Point::new(0.062168647f32, 0.75078124f32),
        anchor_1: Point::new(0.068715945f32, 0.7458164f32),
    },
    Cubic {
        anchor_0: Point::new(0.068715945f32, 0.7458164f32),
        control_0: Point::new(0.14119056f32, 0.690858f32),
        control_1: Point::new(0.21366514f32, 0.6358996f32),
        anchor_1: Point::new(0.2861397f32, 0.58094126f32),
    },
    Cubic {
        anchor_0: Point::new(0.2861397f32, 0.58094126f32),
        control_0: Point::new(0.2914178f32, 0.5769388f32),
        control_1: Point::new(0.2886746f32, 0.5685234f32),
        anchor_1: Point::new(0.2820517f32, 0.5684004f32),
    },
    Cubic {
        anchor_0: Point::new(0.2820517f32, 0.5684004f32),
        control_0: Point::new(0.1911884f32, 0.56671274f32),
        control_1: Point::new(0.100325085f32, 0.5650251f32),
        anchor_1: Point::new(0.009461787f32, 0.56333745f32),
    },
    Cubic {
        anchor_0: Point::new(0.009461787f32, 0.56333745f32),
        control_0: Point::new(0.0012463524f32, 0.5631849f32),
        control_1: Point::new(0.0f32, 0.5514347f32),
        anchor_1: Point::new(0.008000619f32, 0.54956204f32),
    },
    Cubic {
        anchor_0: Point::new(0.008000619f32, 0.54956204f32),
        control_0: Point::new(0.096563034f32, 0.52883303f32),
        control_1: Point::new(0.18512546f32, 0.5081042f32),
        anchor_1: Point::new(0.27368787f32, 0.48737526f32),
    },
    Cubic {
        anchor_0: Point::new(0.27368787f32, 0.48737526f32),
        control_0: Point::new(0.28013754f32, 0.48586565f32),
        control_1: Point::new(0.28105438f32, 0.47706205f32),
        anchor_1: Point::new(0.27505413f32, 0.4742559f32),
    },
    Cubic {
        anchor_0: Point::new(0.27505413f32, 0.4742559f32),
        control_0: Point::new(0.19273275f32, 0.4357567f32),
        control_1: Point::new(0.11041142f32, 0.39725757f32),
        anchor_1: Point::new(0.02809007f32, 0.3587584f32),
    },
    Cubic {
        anchor_0: Point::new(0.02809007f32, 0.3587584f32),
        control_0: Point::new(0.020646961f32, 0.35527748f32),
        control_1: Point::new(0.024287617f32, 0.3440362f32),
        anchor_1: Point::new(0.03235822f32, 0.34557956f32),
    },
    Cubic {
        anchor_0: Point::new(0.03235822f32, 0.34557956f32),
        control_0: Point::new(0.121695235f32, 0.36266434f32),
        control_1: Point::new(0.21103224f32, 0.37974912f32),
        anchor_1: Point::new(0.30036926f32, 0.39683387f32),
    },
    Cubic {
        anchor_0: Point::new(0.30036926f32, 0.39683387f32),
        control_0: Point::new(0.30687535f32, 0.3980781f32),
        control_1: Point::new(0.31129366f32, 0.39040846f32),
        anchor_1: Point::new(0.3069535f32, 0.3854044f32),
    },
    Cubic {
        anchor_0: Point::new(0.3069535f32, 0.3854044f32),
        control_0: Point::new(0.24740824f32, 0.31675056f32),
        control_1: Point::new(0.18786298f32, 0.24809676f32),
        anchor_1: Point::new(0.12831773f32, 0.17944293f32),
    },
    Cubic {
        anchor_0: Point::new(0.12831773f32, 0.17944293f32),
        control_0: Point::new(0.12293388f32, 0.17323555f32),
        control_1: Point::new(0.13083206f32, 0.16444688f32),
        anchor_1: Point::new(0.13757718f32, 0.16913947f32),
    },
    Cubic {
        anchor_0: Point::new(0.13757718f32, 0.16913947f32),
        control_0: Point::new(0.21224159f32, 0.22108382f32),
        control_1: Point::new(0.28690597f32, 0.27302817f32),
        anchor_1: Point::new(0.3615704f32, 0.32497248f32),
    },
    Cubic {
        anchor_0: Point::new(0.3615704f32, 0.32497248f32),
        control_0: Point::new(0.36700794f32, 0.32875544f32),
        control_1: Point::new(0.37416378f32, 0.32354593f32),
        anchor_1: Point::new(0.3722342f32, 0.3172092f32),
    },
    Cubic {
        anchor_0: Point::new(0.3722342f32, 0.3172092f32),
        control_0: Point::new(0.34576094f32, 0.23027158f32),
        control_1: Point::new(0.31928766f32, 0.14333394f32),
        anchor_1: Point::new(0.2928144f32, 0.0563963f32),
    },
    Cubic {
        anchor_0: Point::new(0.2928144f32, 0.0563963f32),
        control_0: Point::new(0.29042083f32, 0.048535805f32),
        control_1: Point::new(0.3012108f32, 0.043719426f32),
        anchor_1: Point::new(0.30546412f32, 0.050749805f32),
    },
    Cubic {
        anchor_0: Point::new(0.30546412f32, 0.050749805f32),
        control_0: Point::new(0.35254577f32, 0.12857206f32),
        control_1: Point::new(0.39962742f32, 0.20639434f32),
        anchor_1: Point::new(0.44670907f32, 0.2842166f32),
    },
    Cubic {
        anchor_0: Point::new(0.44670907f32, 0.2842166f32),
        control_0: Point::new(0.44842345f32, 0.28705037f32),
        control_1: Point::new(0.45144466f32, 0.2880051f32),
        anchor_1: Point::new(0.4541391f32, 0.2874297f32),
    },
];
static BOOM_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: BOOM_CUBICS,
}];
pub const BOOM: ShapeDataStored = ShapeDataStored {
    features: BOOM_FEATURES,
};

static BUN_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.79599994f32, 0.50000006f32),
        control_0: Point::new(0.7996457f32, 0.5011514f32),
        control_1: Point::new(0.8032915f32, 0.5023027f32),
        anchor_1: Point::new(0.8069373f32, 0.50345397f32),
    },
    Cubic {
        anchor_0: Point::new(0.8069373f32, 0.50345397f32),
        control_0: Point::new(0.8373716f32, 0.5130648f32),
        control_1: Point::new(0.8657169f32, 0.5283384f32),
        anchor_1: Point::new(0.8904817f32, 0.54847085f32),
    },
    Cubic {
        anchor_0: Point::new(0.8904817f32, 0.54847085f32),
        control_0: Point::new(0.9541673f32, 0.60024405f32),
        control_1: Point::new(0.98883545f32, 0.6796542f32),
        anchor_1: Point::new(0.98350847f32, 0.7615562f32),
    },
    Cubic {
        anchor_0: Point::new(0.98350847f32, 0.7615562f32),
        control_0: Point::new(0.9834971f32, 0.76173073f32),
        control_1: Point::new(0.98348576f32, 0.76190525f32),
        anchor_1: Point::new(0.98347443f32, 0.7620798f32),
    },
    Cubic {
        anchor_0: Point::new(0.98347443f32, 0.7620798f32),
        control_0: Point::new(0.9747703f32, 0.89590603f32),
        control_1: Point::new(0.86368597f32, 1.0f32),
        anchor_1: Point::new(0.72957695f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.72957695f32, 1.0f32),
        control_0: Point::new(0.57652557f32, 1.0f32),
        control_1: Point::new(0.42347425f32, 1.0f32),
        anchor_1: Point::new(0.2704229f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.2704229f32, 1.0f32),
        control_0: Point::new(0.13631387f32, 1.0f32),
        control_1: Point::new(0.025229609f32, 0.8959061f32),
        anchor_1: Point::new(0.016525464f32, 0.76207983f32),
    },
    Cubic {
        anchor_0: Point::new(0.016525464f32, 0.76207983f32),
        control_0: Point::new(0.016514113f32, 0.76190525f32),
        control_1: Point::new(0.016502762f32, 0.76173073f32),
        anchor_1: Point::new(0.01649143f32, 0.76155627f32),
    },
    Cubic {
        anchor_0: Point::new(0.01649143f32, 0.76155627f32),
        control_0: Point::new(0.011164457f32, 0.6796542f32),
        control_1: Point::new(0.045832597f32, 0.600244f32),
        anchor_1: Point::new(0.1095182f32, 0.5484708f32),
    },
    Cubic {
        anchor_0: Point::new(0.1095182f32, 0.5484708f32),
        control_0: Point::new(0.13428295f32, 0.5283383f32),
        control_1: Point::new(0.16262831f32, 0.51306474f32),
        anchor_1: Point::new(0.19306253f32, 0.50345397f32),
    },
    Cubic {
        anchor_0: Point::new(0.19306253f32, 0.50345397f32),
        control_0: Point::new(0.19670834f32, 0.50230265f32),
        control_1: Point::new(0.20035414f32, 0.5011513f32),
        anchor_1: Point::new(0.20399995f32, 0.5f32),
    },
    Cubic {
        anchor_0: Point::new(0.20399995f32, 0.5f32),
        control_0: Point::new(0.20035423f32, 0.4988487f32),
        control_1: Point::new(0.19670855f32, 0.49769744f32),
        anchor_1: Point::new(0.19306283f32, 0.49654618f32),
    },
    Cubic {
        anchor_0: Point::new(0.19306283f32, 0.49654618f32),
        control_0: Point::new(0.16262832f32, 0.48693532f32),
        control_1: Point::new(0.13428299f32, 0.47166175f32),
        anchor_1: Point::new(0.10951801f32, 0.4515291f32),
    },
    Cubic {
        anchor_0: Point::new(0.10951801f32, 0.4515291f32),
        control_0: Point::new(0.045832522f32, 0.39975592f32),
        control_1: Point::new(0.011164458f32, 0.32034585f32),
        anchor_1: Point::new(0.016491449f32, 0.23844393f32),
    },
    Cubic {
        anchor_0: Point::new(0.016491449f32, 0.23844393f32),
        control_0: Point::new(0.016502801f32, 0.23826936f32),
        control_1: Point::new(0.016514156f32, 0.2380948f32),
        anchor_1: Point::new(0.016525524f32, 0.23792024f32),
    },
    Cubic {
        anchor_0: Point::new(0.016525524f32, 0.23792024f32),
        control_0: Point::new(0.025229687f32, 0.104094f32),
        control_1: Point::new(0.136314f32, 0.0f32),
        anchor_1: Point::new(0.27042302f32, 4.01349E-8f32),
    },
    Cubic {
        anchor_0: Point::new(0.27042302f32, 4.01349E-8f32),
        control_0: Point::new(0.4234744f32, 8.399343E-8f32),
        control_1: Point::new(0.5765258f32, 1.2785196E-7f32),
        anchor_1: Point::new(0.7295772f32, 1.8042849E-7f32),
    },
    Cubic {
        anchor_0: Point::new(0.7295772f32, 1.8042849E-7f32),
        control_0: Point::new(0.86368614f32, 2.1184536E-7f32),
        control_1: Point::new(0.9747704f32, 0.10409427f32),
        anchor_1: Point::new(0.9834745f32, 0.23792048f32),
    },
    Cubic {
        anchor_0: Point::new(0.9834745f32, 0.23792048f32),
        control_0: Point::new(0.9834858f32, 0.23809503f32),
        control_1: Point::new(0.9834972f32, 0.2382696f32),
        anchor_1: Point::new(0.9835086f32, 0.23844416f32),
    },
    Cubic {
        anchor_0: Point::new(0.9835086f32, 0.23844416f32),
        control_0: Point::new(0.9888355f32, 0.320346f32),
        control_1: Point::new(0.9541674f32, 0.39975607f32),
        anchor_1: Point::new(0.8904819f32, 0.45152918f32),
    },
    Cubic {
        anchor_0: Point::new(0.8904819f32, 0.45152918f32),
        control_0: Point::new(0.86571676f32, 0.47166198f32),
        control_1: Point::new(0.83737177f32, 0.48693532f32),
        anchor_1: Point::new(0.80693704f32, 0.49654624f32),
    },
    Cubic {
        anchor_0: Point::new(0.80693704f32, 0.49654624f32),
        control_0: Point::new(0.8032913f32, 0.4976975f32),
        control_1: Point::new(0.79964566f32, 0.49884877f32),
        anchor_1: Point::new(0.79599994f32, 0.50000006f32),
    },
];
static BUN_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: BUN_CUBICS,
}];
pub const BUN: ShapeDataStored = ShapeDataStored {
    features: BUN_FEATURES,
};

static BURST_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.50000834f32, 5.145101E-4f32),
        control_0: Point::new(0.5020512f32, 5.1178806E-4f32),
        control_1: Point::new(0.50409544f32, 0.0015313908f32),
        anchor_1: Point::new(0.5052416f32, 0.003574511f32),
    },
    Cubic {
        anchor_0: Point::new(0.5052416f32, 0.003574511f32),
        control_0: Point::new(0.53311175f32, 0.053256284f32),
        control_1: Point::new(0.5609821f32, 0.102938056f32),
        anchor_1: Point::new(0.58885235f32, 0.15261984f32),
    },
    Cubic {
        anchor_0: Point::new(0.58885235f32, 0.15261984f32),
        control_0: Point::new(0.5905026f32, 0.15556169f32),
        control_1: Point::new(0.5942552f32, 0.15656185f32),
        anchor_1: Point::new(0.59715074f32, 0.15483154f32),
    },
    Cubic {
        anchor_0: Point::new(0.59715074f32, 0.15483154f32),
        control_0: Point::new(0.6459112f32, 0.12569374f32),
        control_1: Point::new(0.6946717f32, 0.096555926f32),
        anchor_1: Point::new(0.7434322f32, 0.06741811f32),
    },
    Cubic {
        anchor_0: Point::new(0.7434322f32, 0.06741811f32),
        control_0: Point::new(0.7474541f32, 0.06501477f32),
        control_1: Point::new(0.7525542f32, 0.067950234f32),
        anchor_1: Point::new(0.7524962f32, 0.07263514f32),
    },
    Cubic {
        anchor_0: Point::new(0.7524962f32, 0.07263514f32),
        control_0: Point::new(0.75179166f32, 0.12959595f32),
        control_1: Point::new(0.7510871f32, 0.18655676f32),
        anchor_1: Point::new(0.7503826f32, 0.24351756f32),
    },
    Cubic {
        anchor_0: Point::new(0.7503826f32, 0.24351756f32),
        control_0: Point::new(0.7503409f32, 0.24689046f32),
        control_1: Point::new(0.7530906f32, 0.24963288f32),
        anchor_1: Point::new(0.7564634f32, 0.24958217f32),
    },
    Cubic {
        anchor_0: Point::new(0.7564634f32, 0.24958217f32),
        control_0: Point::new(0.81326f32, 0.24872832f32),
        control_1: Point::new(0.87005675f32, 0.2478745f32),
        anchor_1: Point::new(0.92685354f32, 0.24702063f32),
    },
    Cubic {
        anchor_0: Point::new(0.92685354f32, 0.24702063f32),
        control_0: Point::new(0.9315382f32, 0.24695021f32),
        control_1: Point::new(0.93448734f32, 0.2520424f32),
        anchor_1: Point::new(0.9320947f32, 0.2560707f32),
    },
    Cubic {
        anchor_0: Point::new(0.9320947f32, 0.2560707f32),
        control_0: Point::new(0.90300405f32, 0.30504793f32),
        control_1: Point::new(0.8739136f32, 0.3540252f32),
        anchor_1: Point::new(0.84482306f32, 0.40300244f32),
    },
    Cubic {
        anchor_0: Point::new(0.84482306f32, 0.40300244f32),
        control_0: Point::new(0.8431005f32, 0.40590256f32),
        control_1: Point::new(0.8441106f32, 0.40965247f32),
        anchor_1: Point::new(0.84705687f32, 0.41129497f32),
    },
    Cubic {
        anchor_0: Point::new(0.84705687f32, 0.41129497f32),
        control_0: Point::new(0.8966711f32, 0.43895385f32),
        control_1: Point::new(0.94628555f32, 0.46661276f32),
        anchor_1: Point::new(0.99589986f32, 0.49427167f32),
    },
    Cubic {
        anchor_0: Point::new(0.99589986f32, 0.49427167f32),
        control_0: Point::new(0.99999213f32, 0.49655303f32),
        control_1: Point::new(1.0f32, 0.50243753f32),
        anchor_1: Point::new(0.9959138f32, 0.5047298f32),
    },
    Cubic {
        anchor_0: Point::new(0.9959138f32, 0.5047298f32),
        control_0: Point::new(0.94623196f32, 0.53260005f32),
        control_1: Point::new(0.89655024f32, 0.5604704f32),
        anchor_1: Point::new(0.8468684f32, 0.58834064f32),
    },
    Cubic {
        anchor_0: Point::new(0.8468684f32, 0.58834064f32),
        control_0: Point::new(0.84392655f32, 0.5899909f32),
        control_1: Point::new(0.8429264f32, 0.59374344f32),
        anchor_1: Point::new(0.8446567f32, 0.59663904f32),
    },
    Cubic {
        anchor_0: Point::new(0.8446567f32, 0.59663904f32),
        control_0: Point::new(0.8737945f32, 0.64539945f32),
        control_1: Point::new(0.9029324f32, 0.69416f32),
        anchor_1: Point::new(0.93207014f32, 0.7429205f32),
    },
    Cubic {
        anchor_0: Point::new(0.93207014f32, 0.7429205f32),
        control_0: Point::new(0.9344735f32, 0.7469424f32),
        control_1: Point::new(0.931538f32, 0.7520425f32),
        anchor_1: Point::new(0.9268531f32, 0.7519845f32),
    },
    Cubic {
        anchor_0: Point::new(0.9268531f32, 0.7519845f32),
        control_0: Point::new(0.86989224f32, 0.7512799f32),
        control_1: Point::new(0.8129315f32, 0.7505754f32),
        anchor_1: Point::new(0.7559707f32, 0.74987084f32),
    },
    Cubic {
        anchor_0: Point::new(0.7559707f32, 0.74987084f32),
        control_0: Point::new(0.7525978f32, 0.7498291f32),
        control_1: Point::new(0.7498554f32, 0.75257885f32),
        anchor_1: Point::new(0.74990606f32, 0.75595164f32),
    },
    Cubic {
        anchor_0: Point::new(0.74990606f32, 0.75595164f32),
        control_0: Point::new(0.75075984f32, 0.8127483f32),
        control_1: Point::new(0.7516138f32, 0.86954504f32),
        anchor_1: Point::new(0.75246763f32, 0.9263418f32),
    },
    Cubic {
        anchor_0: Point::new(0.75246763f32, 0.9263418f32),
        control_0: Point::new(0.752538f32, 0.9310265f32),
        control_1: Point::new(0.7474459f32, 0.9339756f32),
        anchor_1: Point::new(0.74341756f32, 0.9315829f32),
    },
    Cubic {
        anchor_0: Point::new(0.74341756f32, 0.9315829f32),
        control_0: Point::new(0.6944403f32, 0.90249234f32),
        control_1: Point::new(0.64546305f32, 0.8734019f32),
        anchor_1: Point::new(0.59648585f32, 0.8443113f32),
    },
    Cubic {
        anchor_0: Point::new(0.59648585f32, 0.8443113f32),
        control_0: Point::new(0.59358567f32, 0.8425888f32),
        control_1: Point::new(0.5898359f32, 0.8435989f32),
        anchor_1: Point::new(0.58819336f32, 0.8465451f32),
    },
    Cubic {
        anchor_0: Point::new(0.58819336f32, 0.8465451f32),
        control_0: Point::new(0.5605344f32, 0.8961594f32),
        control_1: Point::new(0.5328755f32, 0.9457738f32),
        anchor_1: Point::new(0.50521654f32, 0.99538815f32),
    },
    Cubic {
        anchor_0: Point::new(0.50521654f32, 0.99538815f32),
        control_0: Point::new(0.5029352f32, 0.9994804f32),
        control_1: Point::new(0.4970507f32, 0.9994883f32),
        anchor_1: Point::new(0.49475843f32, 0.99540204f32),
    },
    Cubic {
        anchor_0: Point::new(0.49475843f32, 0.99540204f32),
        control_0: Point::new(0.46688813f32, 0.94572026f32),
        control_1: Point::new(0.4390179f32, 0.89603853f32),
        anchor_1: Point::new(0.41114756f32, 0.8463567f32),
    },
    Cubic {
        anchor_0: Point::new(0.41114756f32, 0.8463567f32),
        control_0: Point::new(0.40949726f32, 0.84341484f32),
        control_1: Point::new(0.4057447f32, 0.8424147f32),
        anchor_1: Point::new(0.40284914f32, 0.84414494f32),
    },
    Cubic {
        anchor_0: Point::new(0.40284914f32, 0.84414494f32),
        control_0: Point::new(0.3540887f32, 0.87328273f32),
        control_1: Point::new(0.30532825f32, 0.9024205f32),
        anchor_1: Point::new(0.25656784f32, 0.93155843f32),
    },
    Cubic {
        anchor_0: Point::new(0.25656784f32, 0.93155843f32),
        control_0: Point::new(0.25254592f32, 0.9339618f32),
        control_1: Point::new(0.2474459f32, 0.93102634f32),
        anchor_1: Point::new(0.24750383f32, 0.92634135f32),
    },
    Cubic {
        anchor_0: Point::new(0.24750383f32, 0.92634135f32),
        control_0: Point::new(0.24820836f32, 0.86938053f32),
        control_1: Point::new(0.24891289f32, 0.8124198f32),
        anchor_1: Point::new(0.24961738f32, 0.75545895f32),
    },
    Cubic {
        anchor_0: Point::new(0.24961738f32, 0.75545895f32),
        control_0: Point::new(0.24965912f32, 0.7520861f32),
        control_1: Point::new(0.24690935f32, 0.74934363f32),
        anchor_1: Point::new(0.2435366f32, 0.74939436f32),
    },
    Cubic {
        anchor_0: Point::new(0.2435366f32, 0.74939436f32),
        control_0: Point::new(0.18673988f32, 0.75024813f32),
        control_1: Point::new(0.12994315f32, 0.751102f32),
        anchor_1: Point::new(0.07314644f32, 0.7519558f32),
    },
    Cubic {
        anchor_0: Point::new(0.07314644f32, 0.7519558f32),
        control_0: Point::new(0.06846169f32, 0.7520262f32),
        control_1: Point::new(0.065512665f32, 0.74693406f32),
        anchor_1: Point::new(0.06790531f32, 0.74290574f32),
    },
    Cubic {
        anchor_0: Point::new(0.06790531f32, 0.74290574f32),
        control_0: Point::new(0.09699585f32, 0.6939285f32),
        control_1: Point::new(0.1260864f32, 0.64495134f32),
        anchor_1: Point::new(0.15517694f32, 0.595974f32),
    },
    Cubic {
        anchor_0: Point::new(0.15517694f32, 0.595974f32),
        control_0: Point::new(0.15689951f32, 0.5930739f32),
        control_1: Point::new(0.15588939f32, 0.589324f32),
        anchor_1: Point::new(0.15294313f32, 0.5876816f32),
    },
    Cubic {
        anchor_0: Point::new(0.15294313f32, 0.5876816f32),
        control_0: Point::new(0.1033288f32, 0.5600227f32),
        control_1: Point::new(0.05371449f32, 0.5323638f32),
        anchor_1: Point::new(0.0041001793f32, 0.5047049f32),
    },
    Cubic {
        anchor_0: Point::new(0.0041001793f32, 0.5047049f32),
        control_0: Point::new(7.842781E-6f32, 0.5024235f32),
        control_1: Point::new(0.0f32, 0.49653903f32),
        anchor_1: Point::new(0.0040862393f32, 0.49424672f32),
    },
    Cubic {
        anchor_0: Point::new(0.0040862393f32, 0.49424672f32),
        control_0: Point::new(0.053768016f32, 0.46637642f32),
        control_1: Point::new(0.103449784f32, 0.4385062f32),
        anchor_1: Point::new(0.15313156f32, 0.4106359f32),
    },
    Cubic {
        anchor_0: Point::new(0.15313156f32, 0.4106359f32),
        control_0: Point::new(0.15607344f32, 0.40898558f32),
        control_1: Point::new(0.15707357f32, 0.405233f32),
        anchor_1: Point::new(0.15534328f32, 0.40233746f32),
    },
    Cubic {
        anchor_0: Point::new(0.15534328f32, 0.40233746f32),
        control_0: Point::new(0.12620546f32, 0.353577f32),
        control_1: Point::new(0.09706764f32, 0.3048165f32),
        anchor_1: Point::new(0.06792982f32, 0.2560561f32),
    },
    Cubic {
        anchor_0: Point::new(0.06792982f32, 0.2560561f32),
        control_0: Point::new(0.06552648f32, 0.25203422f32),
        control_1: Point::new(0.06846193f32, 0.2469342f32),
        anchor_1: Point::new(0.073146835f32, 0.24699214f32),
    },
    Cubic {
        anchor_0: Point::new(0.073146835f32, 0.24699214f32),
        control_0: Point::new(0.13010766f32, 0.24769665f32),
        control_1: Point::new(0.18706849f32, 0.24840118f32),
        anchor_1: Point::new(0.2440293f32, 0.24910568f32),
    },
    Cubic {
        anchor_0: Point::new(0.2440293f32, 0.24910568f32),
        control_0: Point::new(0.24740216f32, 0.2491474f32),
        control_1: Point::new(0.25014463f32, 0.24639764f32),
        anchor_1: Point::new(0.2500939f32, 0.2430249f32),
    },
    Cubic {
        anchor_0: Point::new(0.2500939f32, 0.2430249f32),
        control_0: Point::new(0.24924009f32, 0.18622814f32),
        control_1: Point::new(0.24838628f32, 0.12943141f32),
        anchor_1: Point::new(0.24753244f32, 0.07263466f32),
    },
    Cubic {
        anchor_0: Point::new(0.24753244f32, 0.07263466f32),
        control_0: Point::new(0.24746202f32, 0.06794991f32),
        control_1: Point::new(0.2525542f32, 0.06500088f32),
        anchor_1: Point::new(0.2565825f32, 0.06739352f32),
    },
    Cubic {
        anchor_0: Point::new(0.2565825f32, 0.06739352f32),
        control_0: Point::new(0.30555975f32, 0.09648407f32),
        control_1: Point::new(0.35453698f32, 0.12557462f32),
        anchor_1: Point::new(0.40351424f32, 0.15466519f32),
    },
    Cubic {
        anchor_0: Point::new(0.40351424f32, 0.15466519f32),
        control_0: Point::new(0.4064144f32, 0.15638776f32),
        control_1: Point::new(0.4101643f32, 0.15537763f32),
        anchor_1: Point::new(0.41180676f32, 0.15243137f32),
    },
    Cubic {
        anchor_0: Point::new(0.41180676f32, 0.15243137f32),
        control_0: Point::new(0.4394656f32, 0.10281706f32),
        control_1: Point::new(0.46712446f32, 0.05320275f32),
        anchor_1: Point::new(0.49478334f32, 0.0035884525f32),
    },
    Cubic {
        anchor_0: Point::new(0.49478334f32, 0.0035884525f32),
        control_0: Point::new(0.49592406f32, 0.0015422789f32),
        control_1: Point::new(0.4979655f32, 5.172321E-4f32),
        anchor_1: Point::new(0.50000834f32, 5.145101E-4f32),
    },
];
static BURST_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: BURST_CUBICS,
}];
pub const BURST: ShapeDataStored = ShapeDataStored {
    features: BURST_FEATURES,
};

static CIRCLE_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(1.0f32, 0.5f32),
        control_0: Point::new(1.0f32, 0.5517268f32),
        control_1: Point::new(0.9919087f32, 0.6034536f32),
        anchor_1: Point::new(0.97572625f32, 0.6532584f32),
    },
    Cubic {
        anchor_0: Point::new(0.97572625f32, 0.6532584f32),
        control_0: Point::new(0.9433613f32, 0.75286746f32),
        control_1: Point::new(0.8802929f32, 0.8396737f32),
        anchor_1: Point::new(0.7955603f32, 0.9012355f32),
    },
    Cubic {
        anchor_0: Point::new(0.7955603f32, 0.9012355f32),
        control_0: Point::new(0.71082765f32, 0.96279734f32),
        control_1: Point::new(0.6087809f32, 0.9959544f32),
        anchor_1: Point::new(0.50404555f32, 0.9959544f32),
    },
    Cubic {
        anchor_0: Point::new(0.50404555f32, 0.9959544f32),
        control_0: Point::new(0.3993104f32, 0.9959544f32),
        control_1: Point::new(0.29726353f32, 0.9627972f32),
        anchor_1: Point::new(0.21253096f32, 0.9012355f32),
    },
    Cubic {
        anchor_0: Point::new(0.21253096f32, 0.9012355f32),
        control_0: Point::new(0.12779829f32, 0.83967364f32),
        control_1: Point::new(0.06472998f32, 0.7528675f32),
        anchor_1: Point::new(0.032364976f32, 0.65325826f32),
    },
    Cubic {
        anchor_0: Point::new(0.032364976f32, 0.65325826f32),
        control_0: Point::new(0.0f32, 0.5536492f32),
        control_1: Point::new(0.0f32, 0.44635072f32),
        anchor_1: Point::new(0.032365f32, 0.3467416f32),
    },
    Cubic {
        anchor_0: Point::new(0.032365f32, 0.3467416f32),
        control_0: Point::new(0.06473f32, 0.24713236f32),
        control_1: Point::new(0.12779829f32, 0.16032636f32),
        anchor_1: Point::new(0.21253096f32, 0.09876451f32),
    },
    Cubic {
        anchor_0: Point::new(0.21253096f32, 0.09876451f32),
        control_0: Point::new(0.29726362f32, 0.037202634f32),
        control_1: Point::new(0.3993102f32, 0.0040456844f32),
        anchor_1: Point::new(0.5040456f32, 0.004045655f32),
    },
    Cubic {
        anchor_0: Point::new(0.5040456f32, 0.004045655f32),
        control_0: Point::new(0.6087809f32, 0.004045655f32),
        control_1: Point::new(0.7108278f32, 0.03720272f32),
        anchor_1: Point::new(0.7955605f32, 0.09876463f32),
    },
    Cubic {
        anchor_0: Point::new(0.7955605f32, 0.09876463f32),
        control_0: Point::new(0.8802931f32, 0.16032647f32),
        control_1: Point::new(0.9433614f32, 0.24713263f32),
        anchor_1: Point::new(0.9757263f32, 0.34674183f32),
    },
    Cubic {
        anchor_0: Point::new(0.9757263f32, 0.34674183f32),
        control_0: Point::new(0.9919088f32, 0.39654645f32),
        control_1: Point::new(1.0f32, 0.44827324f32),
        anchor_1: Point::new(1.0f32, 0.5f32),
    },
];
static CIRCLE_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: CIRCLE_CUBICS,
}];
pub const CIRCLE: ShapeDataStored = ShapeDataStored {
    features: CIRCLE_FEATURES,
};

static CLAM_SHELL_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.18728554f32, 0.8156485f32),
        control_0: Point::new(0.16357858f32, 0.8017548f32),
        control_1: Point::new(0.14350897f32, 0.7816546f32),
        anchor_1: Point::new(0.12960371f32, 0.75682896f32),
    },
    Cubic {
        anchor_0: Point::new(0.12960371f32, 0.75682896f32),
        control_0: Point::new(0.09425148f32, 0.6937132f32),
        control_1: Point::new(0.05889928f32, 0.6305975f32),
        anchor_1: Point::new(0.02354707f32, 0.5674817f32),
    },
    Cubic {
        anchor_0: Point::new(0.02354707f32, 0.5674817f32),
        control_0: Point::new(5.7118308E-5f32, 0.52554417f32),
        control_1: Point::new(0.0f32, 0.47442397f32),
        anchor_1: Point::new(0.023396147f32, 0.432434f32),
    },
    Cubic {
        anchor_0: Point::new(0.023396147f32, 0.432434f32),
        control_0: Point::new(0.058490634f32, 0.3694486f32),
        control_1: Point::new(0.09358512f32, 0.30646324f32),
        anchor_1: Point::new(0.1286796f32, 0.24347785f32),
    },
    Cubic {
        anchor_0: Point::new(0.1286796f32, 0.24347785f32),
        control_0: Point::new(0.15644029f32, 0.19365472f32),
        control_1: Point::new(0.20900366f32, 0.16277023f32),
        anchor_1: Point::new(0.26603875f32, 0.16277024f32),
    },
    Cubic {
        anchor_0: Point::new(0.26603875f32, 0.16277024f32),
        control_0: Point::new(0.421762f32, 0.16277024f32),
        control_1: Point::new(0.57748526f32, 0.16277026f32),
        anchor_1: Point::new(0.73320854f32, 0.16277026f32),
    },
    Cubic {
        anchor_0: Point::new(0.73320854f32, 0.16277026f32),
        control_0: Point::new(0.79011786f32, 0.16277026f32),
        control_1: Point::new(0.84258574f32, 0.1935198f32),
        anchor_1: Point::new(0.87039626f32, 0.24317108f32),
    },
    Cubic {
        anchor_0: Point::new(0.87039626f32, 0.24317108f32),
        control_0: Point::new(0.9057484f32, 0.3062868f32),
        control_1: Point::new(0.9411007f32, 0.36940262f32),
        anchor_1: Point::new(0.97645295f32, 0.43251833f32),
    },
    Cubic {
        anchor_0: Point::new(0.97645295f32, 0.43251833f32),
        control_0: Point::new(0.99994284f32, 0.47445592f32),
        control_1: Point::new(1.0f32, 0.5255761f32),
        anchor_1: Point::new(0.9766038f32, 0.56756604f32),
    },
    Cubic {
        anchor_0: Point::new(0.9766038f32, 0.56756604f32),
        control_0: Point::new(0.9415093f32, 0.63055146f32),
        control_1: Point::new(0.90641487f32, 0.6935369f32),
        anchor_1: Point::new(0.8713203f32, 0.75652224f32),
    },
    Cubic {
        anchor_0: Point::new(0.8713203f32, 0.75652224f32),
        control_0: Point::new(0.8435596f32, 0.80634534f32),
        control_1: Point::new(0.79099625f32, 0.8372298f32),
        anchor_1: Point::new(0.73396116f32, 0.8372298f32),
    },
    Cubic {
        anchor_0: Point::new(0.73396116f32, 0.8372298f32),
        control_0: Point::new(0.57823795f32, 0.8372297f32),
        control_1: Point::new(0.42251465f32, 0.8372297f32),
        anchor_1: Point::new(0.26679143f32, 0.8372297f32),
    },
    Cubic {
        anchor_0: Point::new(0.26679143f32, 0.8372297f32),
        control_0: Point::new(0.2383368f32, 0.8372297f32),
        control_1: Point::new(0.21099249f32, 0.82954234f32),
        anchor_1: Point::new(0.18728554f32, 0.8156485f32),
    },
];
static CLAM_SHELL_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: CLAM_SHELL_CUBICS,
}];
pub const CLAM_SHELL: ShapeDataStored = ShapeDataStored {
    features: CLAM_SHELL_FEATURES,
};

static CLOVER_4_LEAF_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.09812955f32),
        control_0: Point::new(0.5048268f32, 0.09441828f32),
        control_1: Point::new(0.5096535f32, 0.09070702f32),
        anchor_1: Point::new(0.51448035f32, 0.08699575f32),
    },
    Cubic {
        anchor_0: Point::new(0.51448035f32, 0.08699575f32),
        control_0: Point::new(0.6276252f32, 2.1085691E-8f32),
        control_1: Point::new(0.7877427f32, 0.010415749f32),
        anchor_1: Point::new(0.8886635f32, 0.11133653f32),
    },
    Cubic {
        anchor_0: Point::new(0.8886635f32, 0.11133653f32),
        control_0: Point::new(0.9895842f32, 0.21225737f32),
        control_1: Point::new(1.0f32, 0.37237483f32),
        anchor_1: Point::new(0.9130042f32, 0.4855197f32),
    },
    Cubic {
        anchor_0: Point::new(0.9130042f32, 0.4855197f32),
        control_0: Point::new(0.90929294f32, 0.49034646f32),
        control_1: Point::new(0.9055817f32, 0.49517325f32),
        anchor_1: Point::new(0.9018704f32, 0.5f32),
    },
    Cubic {
        anchor_0: Point::new(0.9018704f32, 0.5f32),
        control_0: Point::new(0.90558165f32, 0.5048268f32),
        control_1: Point::new(0.90929294f32, 0.5096535f32),
        anchor_1: Point::new(0.91300416f32, 0.51448035f32),
    },
    Cubic {
        anchor_0: Point::new(0.91300416f32, 0.51448035f32),
        control_0: Point::new(0.9999999f32, 0.62762517f32),
        control_1: Point::new(0.9895842f32, 0.7877427f32),
        anchor_1: Point::new(0.8886635f32, 0.8886635f32),
    },
    Cubic {
        anchor_0: Point::new(0.8886635f32, 0.8886635f32),
        control_0: Point::new(0.7877427f32, 0.9895843f32),
        control_1: Point::new(0.6276252f32, 1.0f32),
        anchor_1: Point::new(0.5144803f32, 0.9130043f32),
    },
    Cubic {
        anchor_0: Point::new(0.5144803f32, 0.9130043f32),
        control_0: Point::new(0.5096535f32, 0.90929294f32),
        control_1: Point::new(0.5048268f32, 0.9055818f32),
        anchor_1: Point::new(0.5f32, 0.9018704f32),
    },
    Cubic {
        anchor_0: Point::new(0.5f32, 0.9018704f32),
        control_0: Point::new(0.4951732f32, 0.9055817f32),
        control_1: Point::new(0.4903464f32, 0.909293f32),
        anchor_1: Point::new(0.4855196f32, 0.9130042f32),
    },
    Cubic {
        anchor_0: Point::new(0.4855196f32, 0.9130042f32),
        control_0: Point::new(0.3723747f32, 1.0f32),
        control_1: Point::new(0.2122573f32, 0.9895842f32),
        anchor_1: Point::new(0.1113365f32, 0.8886635f32),
    },
    Cubic {
        anchor_0: Point::new(0.1113365f32, 0.8886635f32),
        control_0: Point::new(0.010415741f32, 0.7877427f32),
        control_1: Point::new(2.1085691E-8f32, 0.6276252f32),
        anchor_1: Point::new(0.08699573f32, 0.5144804f32),
    },
    Cubic {
        anchor_0: Point::new(0.08699573f32, 0.5144804f32),
        control_0: Point::new(0.090707004f32, 0.50965357f32),
        control_1: Point::new(0.09441828f32, 0.5048268f32),
        anchor_1: Point::new(0.09812956f32, 0.5f32),
    },
    Cubic {
        anchor_0: Point::new(0.09812956f32, 0.5f32),
        control_0: Point::new(0.09441827f32, 0.49517313f32),
        control_1: Point::new(0.090707f32, 0.49034634f32),
        anchor_1: Point::new(0.08699573f32, 0.48551953f32),
    },
    Cubic {
        anchor_0: Point::new(0.08699573f32, 0.48551953f32),
        control_0: Point::new(0.0f32, 0.37237465f32),
        control_1: Point::new(0.010415727f32, 0.21225728f32),
        anchor_1: Point::new(0.111336514f32, 0.111336485f32),
    },
    Cubic {
        anchor_0: Point::new(0.111336514f32, 0.111336485f32),
        control_0: Point::new(0.2122573f32, 0.010415706f32),
        control_1: Point::new(0.3723747f32, 0.0f32),
        anchor_1: Point::new(0.48551953f32, 0.08699571f32),
    },
    Cubic {
        anchor_0: Point::new(0.48551953f32, 0.08699571f32),
        control_0: Point::new(0.49034634f32, 0.09070699f32),
        control_1: Point::new(0.4951732f32, 0.09441827f32),
        anchor_1: Point::new(0.5f32, 0.09812955f32),
    },
];
static CLOVER_4_LEAF_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: CLOVER_4_LEAF_CUBICS,
}];
pub const CLOVER_4_LEAF: ShapeDataStored = ShapeDataStored {
    features: CLOVER_4_LEAF_FEATURES,
};

static CLOVER_8_LEAF_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.49999994f32, 0.07129116f32),
        control_0: Point::new(0.50726324f32, 0.067434296f32),
        control_1: Point::new(0.5145265f32, 0.06357744f32),
        anchor_1: Point::new(0.5217898f32, 0.059720606f32),
    },
    Cubic {
        anchor_0: Point::new(0.5217898f32, 0.059720606f32),
        control_0: Point::new(0.6321642f32, 0.0011109513f32),
        control_1: Point::new(0.76829517f32, 0.061188515f32),
        anchor_1: Point::new(0.79938567f32, 0.18222968f32),
    },
    Cubic {
        anchor_0: Point::new(0.79938567f32, 0.18222968f32),
        control_0: Point::new(0.800638f32, 0.18710548f32),
        control_1: Point::new(0.80189043f32, 0.19198129f32),
        anchor_1: Point::new(0.80314285f32, 0.19685706f32),
    },
    Cubic {
        anchor_0: Point::new(0.80314285f32, 0.19685706f32),
        control_0: Point::new(0.81100595f32, 0.19926576f32),
        control_1: Point::new(0.8188691f32, 0.20167448f32),
        anchor_1: Point::new(0.8267322f32, 0.20408317f32),
    },
    Cubic {
        anchor_0: Point::new(0.8267322f32, 0.20408317f32),
        control_0: Point::new(0.946222f32, 0.24068633f32),
        control_1: Point::new(0.9999999f32, 0.37942675f32),
        anchor_1: Point::new(0.9363951f32, 0.48700005f32),
    },
    Cubic {
        anchor_0: Point::new(0.9363951f32, 0.48700005f32),
        control_0: Point::new(0.93383294f32, 0.49133334f32),
        control_1: Point::new(0.93127084f32, 0.49566665f32),
        anchor_1: Point::new(0.9287087f32, 0.49999994f32),
    },
    Cubic {
        anchor_0: Point::new(0.9287087f32, 0.49999994f32),
        control_0: Point::new(0.9325655f32, 0.5072632f32),
        control_1: Point::new(0.9364224f32, 0.5145265f32),
        anchor_1: Point::new(0.94027925f32, 0.5217897f32),
    },
    Cubic {
        anchor_0: Point::new(0.94027925f32, 0.5217897f32),
        control_0: Point::new(0.9988889f32, 0.6321641f32),
        control_1: Point::new(0.93881136f32, 0.7682951f32),
        anchor_1: Point::new(0.81777024f32, 0.79938567f32),
    },
    Cubic {
        anchor_0: Point::new(0.81777024f32, 0.79938567f32),
        control_0: Point::new(0.8128944f32, 0.800638f32),
        control_1: Point::new(0.8080186f32, 0.80189043f32),
        anchor_1: Point::new(0.80314285f32, 0.80314285f32),
    },
    Cubic {
        anchor_0: Point::new(0.80314285f32, 0.80314285f32),
        control_0: Point::new(0.80073404f32, 0.81100595f32),
        control_1: Point::new(0.79832536f32, 0.8188691f32),
        anchor_1: Point::new(0.7959168f32, 0.8267322f32),
    },
    Cubic {
        anchor_0: Point::new(0.7959168f32, 0.8267322f32),
        control_0: Point::new(0.75931364f32, 0.94622207f32),
        control_1: Point::new(0.6205732f32, 1.0f32),
        anchor_1: Point::new(0.51299983f32, 0.93639517f32),
    },
    Cubic {
        anchor_0: Point::new(0.51299983f32, 0.93639517f32),
        control_0: Point::new(0.5086665f32, 0.93383306f32),
        control_1: Point::new(0.5043332f32, 0.9312709f32),
        anchor_1: Point::new(0.49999988f32, 0.9287088f32),
    },
    Cubic {
        anchor_0: Point::new(0.49999988f32, 0.9287088f32),
        control_0: Point::new(0.4927366f32, 0.9325656f32),
        control_1: Point::new(0.4854733f32, 0.9364225f32),
        anchor_1: Point::new(0.47821003f32, 0.94027925f32),
    },
    Cubic {
        anchor_0: Point::new(0.47821003f32, 0.94027925f32),
        control_0: Point::new(0.36783567f32, 0.99888897f32),
        control_1: Point::new(0.23170474f32, 0.93881136f32),
        anchor_1: Point::new(0.20061418f32, 0.81777024f32),
    },
    Cubic {
        anchor_0: Point::new(0.20061418f32, 0.81777024f32),
        control_0: Point::new(0.19936179f32, 0.8128944f32),
        control_1: Point::new(0.19810939f32, 0.8080186f32),
        anchor_1: Point::new(0.19685698f32, 0.8031428f32),
    },
    Cubic {
        anchor_0: Point::new(0.19685698f32, 0.8031428f32),
        control_0: Point::new(0.18899386f32, 0.80073404f32),
        control_1: Point::new(0.18113074f32, 0.79832536f32),
        anchor_1: Point::new(0.17326762f32, 0.7959167f32),
    },
    Cubic {
        anchor_0: Point::new(0.17326762f32, 0.7959167f32),
        control_0: Point::new(0.053777914f32, 0.75931346f32),
        control_1: Point::new(5.5071197E-8f32, 0.62057304f32),
        anchor_1: Point::new(0.06360478f32, 0.5129997f32),
    },
    Cubic {
        anchor_0: Point::new(0.06360478f32, 0.5129997f32),
        control_0: Point::new(0.06616689f32, 0.50866646f32),
        control_1: Point::new(0.06872901f32, 0.5043332f32),
        anchor_1: Point::new(0.071291134f32, 0.49999994f32),
    },
    Cubic {
        anchor_0: Point::new(0.071291134f32, 0.49999994f32),
        control_0: Point::new(0.06743426f32, 0.4927366f32),
        control_1: Point::new(0.06357737f32, 0.4854733f32),
        anchor_1: Point::new(0.059720494f32, 0.47820997f32),
    },
    Cubic {
        anchor_0: Point::new(0.059720494f32, 0.47820997f32),
        control_0: Point::new(0.0011108894f32, 0.36783567f32),
        control_1: Point::new(0.061188404f32, 0.23170477f32),
        anchor_1: Point::new(0.18222949f32, 0.20061418f32),
    },
    Cubic {
        anchor_0: Point::new(0.18222949f32, 0.20061418f32),
        control_0: Point::new(0.18710536f32, 0.19936179f32),
        control_1: Point::new(0.19198126f32, 0.19810936f32),
        anchor_1: Point::new(0.19685712f32, 0.19685696f32),
    },
    Cubic {
        anchor_0: Point::new(0.19685712f32, 0.19685696f32),
        control_0: Point::new(0.19926582f32, 0.18899386f32),
        control_1: Point::new(0.20167452f32, 0.18113074f32),
        anchor_1: Point::new(0.2040832f32, 0.17326765f32),
    },
    Cubic {
        anchor_0: Point::new(0.2040832f32, 0.17326765f32),
        control_0: Point::new(0.24068642f32, 0.053777844f32),
        control_1: Point::new(0.37942687f32, 0.0f32),
        anchor_1: Point::new(0.48700023f32, 0.06360485f32),
    },
    Cubic {
        anchor_0: Point::new(0.48700023f32, 0.06360485f32),
        control_0: Point::new(0.49133345f32, 0.06616695f32),
        control_1: Point::new(0.4956667f32, 0.06872906f32),
        anchor_1: Point::new(0.49999994f32, 0.07129116f32),
    },
];
static CLOVER_8_LEAF_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: CLOVER_8_LEAF_CUBICS,
}];
pub const CLOVER_8_LEAF: ShapeDataStored = ShapeDataStored {
    features: CLOVER_8_LEAF_FEATURES,
};

static COOKIE_12_SIDED_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.005154038f32),
        control_0: Point::new(0.5256129f32, 0.005154042f32),
        control_1: Point::new(0.5512257f32, 0.015461998f32),
        anchor_1: Point::new(0.5700089f32, 0.036077958f32),
    },
    Cubic {
        anchor_0: Point::new(0.5700089f32, 0.036077958f32),
        control_0: Point::new(0.5949293f32, 0.063430086f32),
        control_1: Point::new(0.6334968f32, 0.07376422f32),
        anchor_1: Point::new(0.66875464f32, 0.062536836f32),
    },
    Cubic {
        anchor_0: Point::new(0.66875464f32, 0.062536836f32),
        control_0: Point::new(0.72190386f32, 0.04561209f32),
        control_1: Point::new(0.7780961f32, 0.07805472f32),
        anchor_1: Point::new(0.7900135f32, 0.13254565f32),
    },
    Cubic {
        anchor_0: Point::new(0.7900135f32, 0.13254565f32),
        control_0: Point::new(0.7979192f32, 0.16869347f32),
        control_1: Point::new(0.8261526f32, 0.1969269f32),
        anchor_1: Point::new(0.86230046f32, 0.20483258f32),
    },
    Cubic {
        anchor_0: Point::new(0.86230046f32, 0.20483258f32),
        control_0: Point::new(0.91679144f32, 0.21674997f32),
        control_1: Point::new(0.94923407f32, 0.27294225f32),
        anchor_1: Point::new(0.93230927f32, 0.32609144f32),
    },
    Cubic {
        anchor_0: Point::new(0.93230927f32, 0.32609144f32),
        control_0: Point::new(0.92108184f32, 0.36134923f32),
        control_1: Point::new(0.931416f32, 0.39991683f32),
        anchor_1: Point::new(0.9587682f32, 0.42483726f32),
    },
    Cubic {
        anchor_0: Point::new(0.9587682f32, 0.42483726f32),
        control_0: Point::new(1.0f32, 0.46240348f32),
        control_1: Point::new(1.0f32, 0.5272887f32),
        anchor_1: Point::new(0.9587681f32, 0.5648549f32),
    },
    Cubic {
        anchor_0: Point::new(0.9587681f32, 0.5648549f32),
        control_0: Point::new(0.931416f32, 0.5897754f32),
        control_1: Point::new(0.9210818f32, 0.6283429f32),
        anchor_1: Point::new(0.93230927f32, 0.66360074f32),
    },
    Cubic {
        anchor_0: Point::new(0.93230927f32, 0.66360074f32),
        control_0: Point::new(0.94923395f32, 0.71674997f32),
        control_1: Point::new(0.9167914f32, 0.7729422f32),
        anchor_1: Point::new(0.8623004f32, 0.78485954f32),
    },
    Cubic {
        anchor_0: Point::new(0.8623004f32, 0.78485954f32),
        control_0: Point::new(0.82615256f32, 0.7927652f32),
        control_1: Point::new(0.79791915f32, 0.8209986f32),
        anchor_1: Point::new(0.7900135f32, 0.85714656f32),
    },
    Cubic {
        anchor_0: Point::new(0.7900135f32, 0.85714656f32),
        control_0: Point::new(0.7780961f32, 0.9116374f32),
        control_1: Point::new(0.7219039f32, 0.94408005f32),
        anchor_1: Point::new(0.6687546f32, 0.92715526f32),
    },
    Cubic {
        anchor_0: Point::new(0.6687546f32, 0.92715526f32),
        control_0: Point::new(0.6334968f32, 0.9159278f32),
        control_1: Point::new(0.5949292f32, 0.926262f32),
        anchor_1: Point::new(0.5700088f32, 0.9536142f32),
    },
    Cubic {
        anchor_0: Point::new(0.5700088f32, 0.9536142f32),
        control_0: Point::new(0.53244257f32, 0.994846f32),
        control_1: Point::new(0.46755734f32, 0.994846f32),
        anchor_1: Point::new(0.42999116f32, 0.9536142f32),
    },
    Cubic {
        anchor_0: Point::new(0.42999116f32, 0.9536142f32),
        control_0: Point::new(0.40507072f32, 0.9262621f32),
        control_1: Point::new(0.36650312f32, 0.91592795f32),
        anchor_1: Point::new(0.33124524f32, 0.9271553f32),
    },
    Cubic {
        anchor_0: Point::new(0.33124524f32, 0.9271553f32),
        control_0: Point::new(0.27809605f32, 0.94408005f32),
        control_1: Point::new(0.2219038f32, 0.9116374f32),
        anchor_1: Point::new(0.20998645f32, 0.8571465f32),
    },
    Cubic {
        anchor_0: Point::new(0.20998645f32, 0.8571465f32),
        control_0: Point::new(0.2020808f32, 0.8209986f32),
        control_1: Point::new(0.17384736f32, 0.79276514f32),
        anchor_1: Point::new(0.13769951f32, 0.7848595f32),
    },
    Cubic {
        anchor_0: Point::new(0.13769951f32, 0.7848595f32),
        control_0: Point::new(0.08320858f32, 0.7729422f32),
        control_1: Point::new(0.05076599f32, 0.7167499f32),
        anchor_1: Point::new(0.06769073f32, 0.6636007f32),
    },
    Cubic {
        anchor_0: Point::new(0.06769073f32, 0.6636007f32),
        control_0: Point::new(0.07891815f32, 0.62834287f32),
        control_1: Point::new(0.06858399f32, 0.5897753f32),
        anchor_1: Point::new(0.041231856f32, 0.56485486f32),
    },
    Cubic {
        anchor_0: Point::new(0.041231856f32, 0.56485486f32),
        control_0: Point::new(3.1924742E-8f32, 0.5272887f32),
        control_1: Point::new(0.0f32, 0.46240345f32),
        anchor_1: Point::new(0.04123189f32, 0.42483726f32),
    },
    Cubic {
        anchor_0: Point::new(0.04123189f32, 0.42483726f32),
        control_0: Point::new(0.06858402f32, 0.39991683f32),
        control_1: Point::new(0.07891815f32, 0.36134923f32),
        anchor_1: Point::new(0.0676907f32, 0.3260914f32),
    },
    Cubic {
        anchor_0: Point::new(0.0676907f32, 0.3260914f32),
        control_0: Point::new(0.050765958f32, 0.27294225f32),
        control_1: Point::new(0.08320855f32, 0.21675f32),
        anchor_1: Point::new(0.13769974f32, 0.20483252f32),
    },
    Cubic {
        anchor_0: Point::new(0.13769974f32, 0.20483252f32),
        control_0: Point::new(0.17384759f32, 0.19692685f32),
        control_1: Point::new(0.202081f32, 0.16869342f32),
        anchor_1: Point::new(0.20998664f32, 0.13254553f32),
    },
    Cubic {
        anchor_0: Point::new(0.20998664f32, 0.13254553f32),
        control_0: Point::new(0.22190402f32, 0.07805462f32),
        control_1: Point::new(0.27809626f32, 0.04561206f32),
        anchor_1: Point::new(0.33124542f32, 0.06253674f32),
    },
    Cubic {
        anchor_0: Point::new(0.33124542f32, 0.06253674f32),
        control_0: Point::new(0.36650324f32, 0.07376416f32),
        control_1: Point::new(0.40507084f32, 0.063430026f32),
        anchor_1: Point::new(0.42999125f32, 0.036077894f32),
    },
    Cubic {
        anchor_0: Point::new(0.42999125f32, 0.036077894f32),
        control_0: Point::new(0.44877434f32, 0.015461982f32),
        control_1: Point::new(0.47438717f32, 0.005154034f32),
        anchor_1: Point::new(0.5f32, 0.005154038f32),
    },
];
static COOKIE_12_SIDED_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: COOKIE_12_SIDED_CUBICS,
}];
pub const COOKIE_12_SIDED: ShapeDataStored = ShapeDataStored {
    features: COOKIE_12_SIDED_FEATURES,
};

static COOKIE_4_SIDED_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.8713652f32, 0.8707754f32),
        control_0: Point::new(0.8104821f32, 0.9317583f32),
        control_1: Point::new(0.71593565f32, 0.95913297f32),
        anchor_1: Point::new(0.62197703f32, 0.91859186f32),
    },
    Cubic {
        anchor_0: Point::new(0.62197703f32, 0.91859186f32),
        control_0: Point::new(0.60837704f32, 0.9127237f32),
        control_1: Point::new(0.5947771f32, 0.90685564f32),
        anchor_1: Point::new(0.5811772f32, 0.90098757f32),
    },
    Cubic {
        anchor_0: Point::new(0.5811772f32, 0.90098757f32),
        control_0: Point::new(0.5294812f32, 0.87868184f32),
        control_1: Point::new(0.47086555f32, 0.8787299f32),
        anchor_1: Point::new(0.41920617f32, 0.90112025f32),
    },
    Cubic {
        anchor_0: Point::new(0.41920617f32, 0.90112025f32),
        control_0: Point::new(0.4057553f32, 0.9069501f32),
        control_1: Point::new(0.39230454f32, 0.91278f32),
        anchor_1: Point::new(0.3788537f32, 0.91861f32),
    },
    Cubic {
        anchor_0: Point::new(0.3788537f32, 0.91861f32),
        control_0: Point::new(0.19106969f32, 1.0f32),
        control_1: Point::new(4.888185E-4f32, 0.80973125f32),
        anchor_1: Point::new(0.08157111f32, 0.62181413f32),
    },
    Cubic {
        anchor_0: Point::new(0.08157111f32, 0.62181413f32),
        control_0: Point::new(0.08743921f32, 0.60821414f32),
        control_1: Point::new(0.09330731f32, 0.5946142f32),
        anchor_1: Point::new(0.09917542f32, 0.5810142f32),
    },
    Cubic {
        anchor_0: Point::new(0.09917542f32, 0.5810142f32),
        control_0: Point::new(0.12148117f32, 0.52931815f32),
        control_1: Point::new(0.121433124f32, 0.4707026f32),
        anchor_1: Point::new(0.09904271f32, 0.41904324f32),
    },
    Cubic {
        anchor_0: Point::new(0.09904271f32, 0.41904324f32),
        control_0: Point::new(0.09321282f32, 0.4055924f32),
        control_1: Point::new(0.08738292f32, 0.39214164f32),
        anchor_1: Point::new(0.08155305f32, 0.3786908f32),
    },
    Cubic {
        anchor_0: Point::new(0.08155305f32, 0.3786908f32),
        control_0: Point::new(1.6295037E-4f32, 0.1909068f32),
        control_1: Point::new(0.1904318f32, 3.2592032E-4f32),
        anchor_1: Point::new(0.37834895f32, 0.08140821f32),
    },
    Cubic {
        anchor_0: Point::new(0.37834895f32, 0.08140821f32),
        control_0: Point::new(0.3919489f32, 0.08727631f32),
        control_1: Point::new(0.40554887f32, 0.09314441f32),
        anchor_1: Point::new(0.41914883f32, 0.09901253f32),
    },
    Cubic {
        anchor_0: Point::new(0.41914883f32, 0.09901253f32),
        control_0: Point::new(0.47084484f32, 0.12131827f32),
        control_1: Point::new(0.52946043f32, 0.121270224f32),
        anchor_1: Point::new(0.5811198f32, 0.09887981f32),
    },
    Cubic {
        anchor_0: Point::new(0.5811198f32, 0.09887981f32),
        control_0: Point::new(0.5945706f32, 0.09304991f32),
        control_1: Point::new(0.6080214f32, 0.08722002f32),
        anchor_1: Point::new(0.6214721f32, 0.08139013f32),
    },
    Cubic {
        anchor_0: Point::new(0.6214721f32, 0.08139013f32),
        control_0: Point::new(0.8092561f32, 0.0f32),
        control_1: Point::new(0.99983704f32, 0.19026883f32),
        anchor_1: Point::new(0.91875476f32, 0.378186f32),
    },
    Cubic {
        anchor_0: Point::new(0.91875476f32, 0.378186f32),
        control_0: Point::new(0.9128867f32, 0.39178592f32),
        control_1: Point::new(0.9070186f32, 0.4053859f32),
        anchor_1: Point::new(0.9011505f32, 0.41898587f32),
    },
    Cubic {
        anchor_0: Point::new(0.9011505f32, 0.41898587f32),
        control_0: Point::new(0.8788447f32, 0.47068194f32),
        control_1: Point::new(0.8788927f32, 0.5292974f32),
        anchor_1: Point::new(0.9012832f32, 0.5809568f32),
    },
    Cubic {
        anchor_0: Point::new(0.9012832f32, 0.5809568f32),
        control_0: Point::new(0.90711296f32, 0.5944076f32),
        control_1: Point::new(0.91294295f32, 0.6078584f32),
        anchor_1: Point::new(0.9187728f32, 0.6213092f32),
    },
    Cubic {
        anchor_0: Point::new(0.9187728f32, 0.6213092f32),
        control_0: Point::new(0.9594679f32, 0.7152012f32),
        control_1: Point::new(0.93224823f32, 0.8097924f32),
        anchor_1: Point::new(0.8713652f32, 0.8707754f32),
    },
];
static COOKIE_4_SIDED_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: COOKIE_4_SIDED_CUBICS,
}];
pub const COOKIE_4_SIDED: ShapeDataStored = ShapeDataStored {
    features: COOKIE_4_SIDED_FEATURES,
};

static COOKIE_6_SIDED_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.71652484f32, 0.8728998f32),
        control_0: Point::new(0.6996478f32, 0.8826239f32),
        control_1: Point::new(0.68395615f32, 0.8944055f32),
        anchor_1: Point::new(0.66982883f32, 0.90802586f32),
    },
    Cubic {
        anchor_0: Point::new(0.66982883f32, 0.90802586f32),
        control_0: Point::new(0.6693294f32, 0.90850735f32),
        control_1: Point::new(0.66883f32, 0.90898895f32),
        anchor_1: Point::new(0.6683305f32, 0.90947056f32),
    },
    Cubic {
        anchor_0: Point::new(0.6683305f32, 0.90947056f32),
        control_0: Point::new(0.57443243f32, 1.0f32),
        control_1: Point::new(0.42569405f32, 0.9998689f32),
        anchor_1: Point::new(0.33195567f32, 0.90917414f32),
    },
    Cubic {
        anchor_0: Point::new(0.33195567f32, 0.90917414f32),
        control_0: Point::new(0.3037492f32, 0.88188356f32),
        control_1: Point::new(0.26927224f32, 0.86193776f32),
        anchor_1: Point::new(0.23155367f32, 0.85108906f32),
    },
    Cubic {
        anchor_0: Point::new(0.23155367f32, 0.85108906f32),
        control_0: Point::new(0.23088692f32, 0.8508973f32),
        control_1: Point::new(0.23022017f32, 0.85070556f32),
        anchor_1: Point::new(0.22955342f32, 0.85051376f32),
    },
    Cubic {
        anchor_0: Point::new(0.22955342f32, 0.85051376f32),
        control_0: Point::new(0.10420353f32, 0.8144604f32),
        control_1: Point::new(0.029947806f32, 0.68558365f32),
        anchor_1: Point::new(0.061622597f32, 0.5590564f32),
    },
    Cubic {
        anchor_0: Point::new(0.061622597f32, 0.5590564f32),
        control_0: Point::new(0.07115373f32, 0.5209836f32),
        control_1: Point::new(0.07118882f32, 0.48115262f32),
        anchor_1: Point::new(0.061724786f32, 0.4430631f32),
    },
    Cubic {
        anchor_0: Point::new(0.061724786f32, 0.4430631f32),
        control_0: Point::new(0.06155749f32, 0.4423898f32),
        control_1: Point::new(0.0613902f32, 0.4417165f32),
        anchor_1: Point::new(0.0612229f32, 0.4410432f32),
    },
    Cubic {
        anchor_0: Point::new(0.0612229f32, 0.4410432f32),
        control_0: Point::new(0.029771078f32, 0.31446034f32),
        control_1: Point::new(0.10425376f32, 0.18571465f32),
        anchor_1: Point::new(0.22966698f32, 0.14988223f32),
    },
    Cubic {
        anchor_0: Point::new(0.22966698f32, 0.14988223f32),
        control_0: Point::new(0.26740456f32, 0.13910002f32),
        control_1: Point::new(0.30191663f32, 0.119215f32),
        anchor_1: Point::new(0.33017114f32, 0.091974124f32),
    },
    Cubic {
        anchor_0: Point::new(0.33017114f32, 0.091974124f32),
        control_0: Point::new(0.3306706f32, 0.09149257f32),
        control_1: Point::new(0.33117005f32, 0.091011025f32),
        anchor_1: Point::new(0.3316695f32, 0.09052947f32),
    },
    Cubic {
        anchor_0: Point::new(0.3316695f32, 0.09052947f32),
        control_0: Point::new(0.42556754f32, 0.0f32),
        control_1: Point::new(0.57430595f32, 1.3102757E-4f32),
        anchor_1: Point::new(0.6680444f32, 0.09082588f32),
    },
    Cubic {
        anchor_0: Point::new(0.6680444f32, 0.09082588f32),
        control_0: Point::new(0.69625074f32, 0.11811641f32),
        control_1: Point::new(0.7307279f32, 0.13806231f32),
        anchor_1: Point::new(0.7684463f32, 0.14891095f32),
    },
    Cubic {
        anchor_0: Point::new(0.7684463f32, 0.14891095f32),
        control_0: Point::new(0.7691131f32, 0.14910275f32),
        control_1: Point::new(0.7697799f32, 0.14929454f32),
        anchor_1: Point::new(0.7704467f32, 0.14948629f32),
    },
    Cubic {
        anchor_0: Point::new(0.7704467f32, 0.14948629f32),
        control_0: Point::new(0.89579654f32, 0.18553965f32),
        control_1: Point::new(0.9700522f32, 0.31441635f32),
        anchor_1: Point::new(0.9383774f32, 0.44094348f32),
    },
    Cubic {
        anchor_0: Point::new(0.9383774f32, 0.44094348f32),
        control_0: Point::new(0.92884624f32, 0.4790163f32),
        control_1: Point::new(0.92881125f32, 0.5188472f32),
        anchor_1: Point::new(0.9382752f32, 0.55693674f32),
    },
    Cubic {
        anchor_0: Point::new(0.9382752f32, 0.55693674f32),
        control_0: Point::new(0.93844247f32, 0.55761003f32),
        control_1: Point::new(0.9386098f32, 0.5582833f32),
        anchor_1: Point::new(0.93877715f32, 0.5589567f32),
    },
    Cubic {
        anchor_0: Point::new(0.93877715f32, 0.5589567f32),
        control_0: Point::new(0.9702289f32, 0.68553954f32),
        control_1: Point::new(0.8957463f32, 0.8142852f32),
        anchor_1: Point::new(0.7703331f32, 0.85011774f32),
    },
    Cubic {
        anchor_0: Point::new(0.7703331f32, 0.85011774f32),
        control_0: Point::new(0.75146437f32, 0.8555088f32),
        control_1: Point::new(0.73340195f32, 0.86317563f32),
        anchor_1: Point::new(0.71652484f32, 0.8728998f32),
    },
];
static COOKIE_6_SIDED_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: COOKIE_6_SIDED_CUBICS,
}];
pub const COOKIE_6_SIDED: ShapeDataStored = ShapeDataStored {
    features: COOKIE_6_SIDED_FEATURES,
};

static COOKIE_7_SIDED_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.021824071f32),
        control_0: Point::new(0.54879564f32, 0.021824071f32),
        control_1: Point::new(0.5975913f32, 0.040400095f32),
        anchor_1: Point::new(0.634874f32, 0.077552214f32),
    },
    Cubic {
        anchor_0: Point::new(0.634874f32, 0.077552214f32),
        control_0: Point::new(0.6653732f32, 0.107944645f32),
        control_1: Point::new(0.70524335f32, 0.12714514f32),
        anchor_1: Point::new(0.7480212f32, 0.13204098f32),
    },
    Cubic {
        anchor_0: Point::new(0.7480212f32, 0.13204098f32),
        control_0: Point::new(0.8526051f32, 0.14401054f32),
        control_1: Point::new(0.9278088f32, 0.23831302f32),
        anchor_1: Point::new(0.9162061f32, 0.34293833f32),
    },
    Cubic {
        anchor_0: Point::new(0.9162061f32, 0.34293833f32),
        control_0: Point::new(0.91146034f32, 0.38573298f32),
        control_1: Point::new(0.9213075f32, 0.4288761f32),
        anchor_1: Point::new(0.9441512f32, 0.4653736f32),
    },
    Cubic {
        anchor_0: Point::new(0.9441512f32, 0.4653736f32),
        control_0: Point::new(1.0f32, 0.5546035f32),
        control_1: Point::new(0.9731601f32, 0.67219675f32),
        anchor_1: Point::new(0.88412666f32, 0.72835827f32),
    },
    Cubic {
        anchor_0: Point::new(0.88412666f32, 0.72835827f32),
        control_0: Point::new(0.84770954f32, 0.75132984f32),
        control_1: Point::new(0.8201184f32, 0.78592795f32),
        anchor_1: Point::new(0.8058263f32, 0.8265438f32),
    },
    Cubic {
        anchor_0: Point::new(0.8058263f32, 0.8265438f32),
        control_0: Point::new(0.77088475f32, 0.92584205f32),
        control_1: Point::new(0.6622122f32, 0.97817594f32),
        anchor_1: Point::new(0.56279194f32, 0.9435829f32),
    },
    Cubic {
        anchor_0: Point::new(0.56279194f32, 0.9435829f32),
        control_0: Point::new(0.5221263f32, 0.92943335f32),
        control_1: Point::new(0.4778737f32, 0.9294334f32),
        anchor_1: Point::new(0.43720806f32, 0.9435829f32),
    },
    Cubic {
        anchor_0: Point::new(0.43720806f32, 0.9435829f32),
        control_0: Point::new(0.3377878f32, 0.97817594f32),
        control_1: Point::new(0.22911528f32, 0.92584205f32),
        anchor_1: Point::new(0.19417366f32, 0.8265437f32),
    },
    Cubic {
        anchor_0: Point::new(0.19417366f32, 0.8265437f32),
        control_0: Point::new(0.17988156f32, 0.78592783f32),
        control_1: Point::new(0.15229054f32, 0.75132984f32),
        anchor_1: Point::new(0.115873344f32, 0.72835827f32),
    },
    Cubic {
        anchor_0: Point::new(0.115873344f32, 0.72835827f32),
        control_0: Point::new(0.026839884f32, 0.67219675f32),
        control_1: Point::new(0.0f32, 0.5546036f32),
        anchor_1: Point::new(0.055848908f32, 0.46537355f32),
    },
    Cubic {
        anchor_0: Point::new(0.055848908f32, 0.46537355f32),
        control_0: Point::new(0.078692615f32, 0.42887604f32),
        control_1: Point::new(0.088539734f32, 0.38573292f32),
        anchor_1: Point::new(0.08379392f32, 0.34293813f32),
    },
    Cubic {
        anchor_0: Point::new(0.08379392f32, 0.34293813f32),
        control_0: Point::new(0.07219132f32, 0.23831287f32),
        control_1: Point::new(0.14739504f32, 0.14401048f32),
        anchor_1: Point::new(0.25197893f32, 0.13204098f32),
    },
    Cubic {
        anchor_0: Point::new(0.25197893f32, 0.13204098f32),
        control_0: Point::new(0.29475668f32, 0.12714511f32),
        control_1: Point::new(0.33462688f32, 0.10794468f32),
        anchor_1: Point::new(0.36512613f32, 0.07755215f32),
    },
    Cubic {
        anchor_0: Point::new(0.36512613f32, 0.07755215f32),
        control_0: Point::new(0.40240875f32, 0.040400095f32),
        control_1: Point::new(0.4512044f32, 0.02182407f32),
        anchor_1: Point::new(0.5f32, 0.021824071f32),
    },
];
static COOKIE_7_SIDED_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: COOKIE_7_SIDED_CUBICS,
}];
pub const COOKIE_7_SIDED: ShapeDataStored = ShapeDataStored {
    features: COOKIE_7_SIDED_FEATURES,
};

static COOKIE_9_SIDED_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.014150618f32),
        control_0: Point::new(0.53648806f32, 0.014150613f32),
        control_1: Point::new(0.5729761f32, 0.027259544f32),
        anchor_1: Point::new(0.60187346f32, 0.053477466f32),
    },
    Cubic {
        anchor_0: Point::new(0.60187346f32, 0.053477466f32),
        control_0: Point::new(0.6312687f32, 0.0801471f32),
        control_1: Point::new(0.66995025f32, 0.09422602f32),
        anchor_1: Point::new(0.70961124f32, 0.09269085f32),
    },
    Cubic {
        anchor_0: Point::new(0.70961124f32, 0.09269085f32),
        control_0: Point::new(0.7875895f32, 0.08967239f32),
        control_1: Point::new(0.8551222f32, 0.14633904f32),
        anchor_1: Point::new(0.86569047f32, 0.22365679f32),
    },
    Cubic {
        anchor_0: Point::new(0.86569047f32, 0.22365679f32),
        control_0: Point::new(0.87106556f32, 0.26298186f32),
        control_1: Point::new(0.8916476f32, 0.29863095f32),
        anchor_1: Point::new(0.9230165f32, 0.3229485f32),
    },
    Cubic {
        anchor_0: Point::new(0.9230165f32, 0.3229485f32),
        control_0: Point::new(0.98469156f32, 0.3707597f32),
        control_1: Point::new(1.0f32, 0.4575781f32),
        anchor_1: Point::new(0.9583968f32, 0.52360004f32),
    },
    Cubic {
        anchor_0: Point::new(0.9583968f32, 0.52360004f32),
        control_0: Point::new(0.9372367f32, 0.55717987f32),
        control_1: Point::new(0.93008864f32, 0.5977185f32),
        anchor_1: Point::new(0.93848765f32, 0.6365104f32),
    },
    Cubic {
        anchor_0: Point::new(0.93848765f32, 0.6365104f32),
        control_0: Point::new(0.955001f32, 0.7127798f32),
        control_1: Point::new(0.91092217f32, 0.78912663f32),
        anchor_1: Point::new(0.8366141f32, 0.8129603f32),
    },
    Cubic {
        anchor_0: Point::new(0.8366141f32, 0.8129603f32),
        control_0: Point::new(0.79881984f32, 0.8250826f32),
        control_1: Point::new(0.7672864f32, 0.85154223f32),
        anchor_1: Point::new(0.74878544f32, 0.88665736f32),
    },
    Cubic {
        anchor_0: Point::new(0.74878544f32, 0.88665736f32),
        control_0: Point::new(0.71241033f32, 0.9556977f32),
        control_1: Point::new(0.62956923f32, 0.9858494f32),
        anchor_1: Point::new(0.557326f32, 0.9563428f32),
    },
    Cubic {
        anchor_0: Point::new(0.557326f32, 0.9563428f32),
        control_0: Point::new(0.52058196f32, 0.9413353f32),
        control_1: Point::new(0.47941795f32, 0.9413353f32),
        anchor_1: Point::new(0.44267377f32, 0.9563428f32),
    },
    Cubic {
        anchor_0: Point::new(0.44267377f32, 0.9563428f32),
        control_0: Point::new(0.37043056f32, 0.9858494f32),
        control_1: Point::new(0.28758946f32, 0.9556977f32),
        anchor_1: Point::new(0.2512144f32, 0.88665724f32),
    },
    Cubic {
        anchor_0: Point::new(0.2512144f32, 0.88665724f32),
        control_0: Point::new(0.23271349f32, 0.8515422f32),
        control_1: Point::new(0.20117998f32, 0.8250824f32),
        anchor_1: Point::new(0.16338576f32, 0.8129602f32),
    },
    Cubic {
        anchor_0: Point::new(0.16338576f32, 0.8129602f32),
        control_0: Point::new(0.08907778f32, 0.7891266f32),
        control_1: Point::new(0.044998955f32, 0.7127798f32),
        anchor_1: Point::new(0.061512407f32, 0.63651025f32),
    },
    Cubic {
        anchor_0: Point::new(0.061512407f32, 0.63651025f32),
        control_0: Point::new(0.0699114f32, 0.59771836f32),
        control_1: Point::new(0.06276337f32, 0.5571798f32),
        anchor_1: Point::new(0.041603155f32, 0.52359974f32),
    },
    Cubic {
        anchor_0: Point::new(0.041603155f32, 0.52359974f32),
        control_0: Point::new(0.0f32, 0.4575778f32),
        control_1: Point::new(0.015308507f32, 0.37075946f32),
        anchor_1: Point::new(0.0769836f32, 0.3229483f32),
    },
    Cubic {
        anchor_0: Point::new(0.0769836f32, 0.3229483f32),
        control_0: Point::new(0.10835253f32, 0.29863077f32),
        control_1: Point::new(0.12893455f32, 0.26298165f32),
        anchor_1: Point::new(0.13430968f32, 0.22365662f32),
    },
    Cubic {
        anchor_0: Point::new(0.13430968f32, 0.22365662f32),
        control_0: Point::new(0.14487787f32, 0.14633888f32),
        control_1: Point::new(0.21241055f32, 0.08967229f32),
        anchor_1: Point::new(0.29038888f32, 0.09269078f32),
    },
    Cubic {
        anchor_0: Point::new(0.29038888f32, 0.09269078f32),
        control_0: Point::new(0.33004987f32, 0.09422605f32),
        control_1: Point::new(0.36873144f32, 0.0801471f32),
        anchor_1: Point::new(0.39812666f32, 0.053477436f32),
    },
    Cubic {
        anchor_0: Point::new(0.39812666f32, 0.053477436f32),
        control_0: Point::new(0.42702392f32, 0.02725956f32),
        control_1: Point::new(0.46351194f32, 0.014150621f32),
        anchor_1: Point::new(0.5f32, 0.014150618f32),
    },
];
static COOKIE_9_SIDED_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: COOKIE_9_SIDED_CUBICS,
}];
pub const COOKIE_9_SIDED: ShapeDataStored = ShapeDataStored {
    features: COOKIE_9_SIDED_FEATURES,
};

static DIAMOND_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.49999994f32, 1.0f32),
        control_0: Point::new(0.4727132f32, 1.0f32),
        control_1: Point::new(0.44542643f32, 0.99256146f32),
        anchor_1: Point::new(0.42130372f32, 0.97768426f32),
    },
    Cubic {
        anchor_0: Point::new(0.42130372f32, 0.97768426f32),
        control_0: Point::new(0.3980278f32, 0.9633294f32),
        control_1: Point::new(0.37172875f32, 0.929255f32),
        anchor_1: Point::new(0.31913075f32, 0.86110616f32),
    },
    Cubic {
        anchor_0: Point::new(0.31913075f32, 0.86110616f32),
        control_0: Point::new(0.25199988f32, 0.7741279f32),
        control_1: Point::new(0.184869f32, 0.68714964f32),
        anchor_1: Point::new(0.117738135f32, 0.60017145f32),
    },
    Cubic {
        anchor_0: Point::new(0.117738135f32, 0.60017145f32),
        control_0: Point::new(0.07388129f32, 0.54334825f32),
        control_1: Point::new(0.07388129f32, 0.46409053f32),
        anchor_1: Point::new(0.11773815f32, 0.4072673f32),
    },
    Cubic {
        anchor_0: Point::new(0.11773815f32, 0.4072673f32),
        control_0: Point::new(0.184869f32, 0.32028905f32),
        control_1: Point::new(0.25199988f32, 0.23331082f32),
        anchor_1: Point::new(0.31913075f32, 0.14633258f32),
    },
    Cubic {
        anchor_0: Point::new(0.31913075f32, 0.14633258f32),
        control_0: Point::new(0.37172875f32, 0.0781838f32),
        control_1: Point::new(0.3980278f32, 0.04410941f32),
        anchor_1: Point::new(0.42130375f32, 0.029754452f32),
    },
    Cubic {
        anchor_0: Point::new(0.42130375f32, 0.029754452f32),
        control_0: Point::new(0.46954924f32, 0.0f32),
        control_1: Point::new(0.5304509f32, 7.397207E-9f32),
        anchor_1: Point::new(0.57869637f32, 0.029754482f32),
    },
    Cubic {
        anchor_0: Point::new(0.57869637f32, 0.029754482f32),
        control_0: Point::new(0.60197216f32, 0.04410938f32),
        control_1: Point::new(0.62827116f32, 0.07818378f32),
        anchor_1: Point::new(0.6808692f32, 0.14633258f32),
    },
    Cubic {
        anchor_0: Point::new(0.6808692f32, 0.14633258f32),
        control_0: Point::new(0.74800014f32, 0.23331083f32),
        control_1: Point::new(0.815131f32, 0.3202891f32),
        anchor_1: Point::new(0.8822618f32, 0.40726733f32),
    },
    Cubic {
        anchor_0: Point::new(0.8822618f32, 0.40726733f32),
        control_0: Point::new(0.92611873f32, 0.4640906f32),
        control_1: Point::new(0.92611873f32, 0.5433483f32),
        anchor_1: Point::new(0.8822618f32, 0.6001715f32),
    },
    Cubic {
        anchor_0: Point::new(0.8822618f32, 0.6001715f32),
        control_0: Point::new(0.8151309f32, 0.6871497f32),
        control_1: Point::new(0.74800014f32, 0.77412796f32),
        anchor_1: Point::new(0.6808692f32, 0.86110616f32),
    },
    Cubic {
        anchor_0: Point::new(0.6808692f32, 0.86110616f32),
        control_0: Point::new(0.62827116f32, 0.92925483f32),
        control_1: Point::new(0.60197216f32, 0.96332926f32),
        anchor_1: Point::new(0.5786962f32, 0.97768426f32),
    },
    Cubic {
        anchor_0: Point::new(0.5786962f32, 0.97768426f32),
        control_0: Point::new(0.5545735f32, 0.99256146f32),
        control_1: Point::new(0.5272867f32, 1.0f32),
        anchor_1: Point::new(0.49999994f32, 1.0f32),
    },
];
static DIAMOND_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: DIAMOND_CUBICS,
}];
pub const DIAMOND: ShapeDataStored = ShapeDataStored {
    features: DIAMOND_FEATURES,
};

static FAN_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.95717525f32, 0.95531684f32),
        control_0: Point::new(0.938846f32, 0.97413886f32),
        control_1: Point::new(0.9155047f32, 0.98808014f32),
        anchor_1: Point::new(0.8891754f32, 0.99506223f32),
    },
    Cubic {
        anchor_0: Point::new(0.8891754f32, 0.99506223f32),
        control_0: Point::new(0.87055564f32, 1.0f32),
        control_1: Point::new(0.84303874f32, 1.0f32),
        anchor_1: Point::new(0.78800505f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.78800505f32, 1.0f32),
        control_0: Point::new(0.5757516f32, 1.0f32),
        control_1: Point::new(0.3634982f32, 1.0f32),
        anchor_1: Point::new(0.1512448f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.1512448f32, 1.0f32),
        control_0: Point::new(0.06796568f32, 1.0f32),
        control_1: Point::new(4.546277E-4f32, 0.93248886f32),
        anchor_1: Point::new(4.546277E-4f32, 0.8492097f32),
    },
    Cubic {
        anchor_0: Point::new(4.546277E-4f32, 0.8492097f32),
        control_0: Point::new(4.546277E-4f32, 0.61604124f32),
        control_1: Point::new(4.546277E-4f32, 0.3828729f32),
        anchor_1: Point::new(4.546277E-4f32, 0.14970455f32),
    },
    Cubic {
        anchor_0: Point::new(4.546277E-4f32, 0.14970455f32),
        control_0: Point::new(4.5463606E-4f32, 0.06671953f32),
        control_1: Point::new(0.06876169f32, 0.0f32),
        anchor_1: Point::new(0.15172377f32, 0.001951042f32),
    },
    Cubic {
        anchor_0: Point::new(0.15172377f32, 0.001951042f32),
        control_0: Point::new(0.17267422f32, 0.0024437425f32),
        control_1: Point::new(0.1936247f32, 0.0029364433f32),
        anchor_1: Point::new(0.21457517f32, 0.0034292482f32),
    },
    Cubic {
        anchor_0: Point::new(0.21457517f32, 0.0034292482f32),
        control_0: Point::new(0.641736f32, 0.013474982f32),
        control_1: Point::new(0.98599225f32, 0.35669622f32),
        anchor_1: Point::new(0.9973243f32, 0.78382474f32),
    },
    Cubic {
        anchor_0: Point::new(0.9973243f32, 0.78382474f32),
        control_0: Point::new(0.99733484f32, 0.7842235f32),
        control_1: Point::new(0.99734545f32, 0.7846224f32),
        anchor_1: Point::new(0.99735606f32, 0.78502107f32),
    },
    Cubic {
        anchor_0: Point::new(0.99735606f32, 0.78502107f32),
        control_0: Point::new(0.99881566f32, 0.8400355f32),
        control_1: Point::new(0.9995454f32, 0.8675427f32),
        anchor_1: Point::new(0.99510324f32, 0.88628685f32),
    },
    Cubic {
        anchor_0: Point::new(0.99510324f32, 0.88628685f32),
        control_0: Point::new(0.98882186f32, 0.912792f32),
        control_1: Point::new(0.9755046f32, 0.9364948f32),
        anchor_1: Point::new(0.95717525f32, 0.95531684f32),
    },
];
static FAN_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: FAN_CUBICS,
}];
pub const FAN: ShapeDataStored = ShapeDataStored {
    features: FAN_FEATURES,
};

static FLOWER_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.36974832f32, 0.18639387f32),
        control_0: Point::new(0.3785385f32, 0.16002327f32),
        control_1: Point::new(0.3873287f32, 0.13365272f32),
        anchor_1: Point::new(0.39611894f32, 0.10728214f32),
    },
    Cubic {
        anchor_0: Point::new(0.39611894f32, 0.10728214f32),
        control_0: Point::new(0.4089664f32, 0.06873975f32),
        control_1: Point::new(0.4331206f32, 0.034958526f32),
        anchor_1: Point::new(0.46543673f32, 0.010336585f32),
    },
    Cubic {
        anchor_0: Point::new(0.46543673f32, 0.010336585f32),
        control_0: Point::new(0.47420937f32, 0.0036526364f32),
        control_1: Point::new(0.48493078f32, 2.7938904E-5f32),
        anchor_1: Point::new(0.49595958f32, 1.7383149E-5f32),
    },
    Cubic {
        anchor_0: Point::new(0.49595958f32, 1.7383149E-5f32),
        control_0: Point::new(0.4983341f32, 1.5110636E-5f32),
        control_1: Point::new(0.50070864f32, 1.2838241E-5f32),
        anchor_1: Point::new(0.5030832f32, 1.0563046E-5f32),
    },
    Cubic {
        anchor_0: Point::new(0.5030832f32, 1.0563046E-5f32),
        control_0: Point::new(0.51411206f32, 0.0f32),
        control_1: Point::new(0.52484035f32, 0.003604166f32),
        anchor_1: Point::new(0.5336258f32, 0.010271285f32),
    },
    Cubic {
        anchor_0: Point::new(0.5336258f32, 0.010271285f32),
        control_0: Point::new(0.565989f32, 0.034831263f32),
        control_1: Point::new(0.59020793f32, 0.06856625f32),
        anchor_1: Point::new(0.60312915f32, 0.107083954f32),
    },
    Cubic {
        anchor_0: Point::new(0.60312915f32, 0.107083954f32),
        control_0: Point::new(0.61196977f32, 0.13343766f32),
        control_1: Point::new(0.6208105f32, 0.15979138f32),
        anchor_1: Point::new(0.6296512f32, 0.18614511f32),
    },
    Cubic {
        anchor_0: Point::new(0.6296512f32, 0.18614511f32),
        control_0: Point::new(0.6545136f32, 0.1737139f32),
        control_1: Point::new(0.67937607f32, 0.16128269f32),
        anchor_1: Point::new(0.7042385f32, 0.14885144f32),
    },
    Cubic {
        anchor_0: Point::new(0.7042385f32, 0.14885144f32),
        control_0: Point::new(0.7405766f32, 0.13068242f32),
        control_1: Point::new(0.78154325f32, 0.123875074f32),
        anchor_1: Point::new(0.8218045f32, 0.1293158f32),
    },
    Cubic {
        anchor_0: Point::new(0.8218045f32, 0.1293158f32),
        control_0: Point::new(0.8327339f32, 0.13079275f32),
        control_1: Point::new(0.8428782f32, 0.13581088f32),
        anchor_1: Point::new(0.85068417f32, 0.14360192f32),
    },
    Cubic {
        anchor_0: Point::new(0.85068417f32, 0.14360192f32),
        control_0: Point::new(0.85236484f32, 0.14527938f32),
        control_1: Point::new(0.85404545f32, 0.14695683f32),
        anchor_1: Point::new(0.8557262f32, 0.1486343f32),
    },
    Cubic {
        anchor_0: Point::new(0.8557262f32, 0.1486343f32),
        control_0: Point::new(0.8635321f32, 0.15642534f32),
        control_1: Point::new(0.8685697f32, 0.1665599f32),
        anchor_1: Point::new(0.87006754f32, 0.17748646f32),
    },
    Cubic {
        anchor_0: Point::new(0.87006754f32, 0.17748646f32),
        control_0: Point::new(0.8755853f32, 0.21773736f32),
        control_1: Point::new(0.8688564f32, 0.25871694f32),
        anchor_1: Point::new(0.8507569f32, 0.29508984f32),
    },
    Cubic {
        anchor_0: Point::new(0.8507569f32, 0.29508984f32),
        control_0: Point::new(0.8383733f32, 0.31997597f32),
        control_1: Point::new(0.8259898f32, 0.34486213f32),
        anchor_1: Point::new(0.8136062f32, 0.3697483f32),
    },
    Cubic {
        anchor_0: Point::new(0.8136062f32, 0.3697483f32),
        control_0: Point::new(0.8399767f32, 0.3785385f32),
        control_1: Point::new(0.8663473f32, 0.3873287f32),
        anchor_1: Point::new(0.8927179f32, 0.39611894f32),
    },
    Cubic {
        anchor_0: Point::new(0.8927179f32, 0.39611894f32),
        control_0: Point::new(0.93126017f32, 0.40896636f32),
        control_1: Point::new(0.9650416f32, 0.43312073f32),
        anchor_1: Point::new(0.9896635f32, 0.4654368f32),
    },
    Cubic {
        anchor_0: Point::new(0.9896635f32, 0.4654368f32),
        control_0: Point::new(0.9963474f32, 0.47420946f32),
        control_1: Point::new(0.99997216f32, 0.48493093f32),
        anchor_1: Point::new(0.99998266f32, 0.49595973f32),
    },
    Cubic {
        anchor_0: Point::new(0.99998266f32, 0.49595973f32),
        control_0: Point::new(0.9999849f32, 0.49833426f32),
        control_1: Point::new(0.99998724f32, 0.5007088f32),
        anchor_1: Point::new(0.9999895f32, 0.50308335f32),
    },
    Cubic {
        anchor_0: Point::new(0.9999895f32, 0.50308335f32),
        control_0: Point::new(1.0f32, 0.5141121f32),
        control_1: Point::new(0.9963958f32, 0.52484035f32),
        anchor_1: Point::new(0.9897288f32, 0.5336258f32),
    },
    Cubic {
        anchor_0: Point::new(0.9897288f32, 0.5336258f32),
        control_0: Point::new(0.96516883f32, 0.5659891f32),
        control_1: Point::new(0.9314338f32, 0.59020805f32),
        anchor_1: Point::new(0.892916f32, 0.6031292f32),
    },
    Cubic {
        anchor_0: Point::new(0.892916f32, 0.6031292f32),
        control_0: Point::new(0.86656225f32, 0.61196977f32),
        control_1: Point::new(0.84020853f32, 0.6208105f32),
        anchor_1: Point::new(0.81385493f32, 0.6296512f32),
    },
    Cubic {
        anchor_0: Point::new(0.81385493f32, 0.6296512f32),
        control_0: Point::new(0.82628614f32, 0.6545136f32),
        control_1: Point::new(0.83871734f32, 0.67937607f32),
        anchor_1: Point::new(0.8511486f32, 0.70423853f32),
    },
    Cubic {
        anchor_0: Point::new(0.8511486f32, 0.70423853f32),
        control_0: Point::new(0.8693176f32, 0.7405766f32),
        control_1: Point::new(0.876125f32, 0.78154325f32),
        anchor_1: Point::new(0.8706843f32, 0.8218045f32),
    },
    Cubic {
        anchor_0: Point::new(0.8706843f32, 0.8218045f32),
        control_0: Point::new(0.8692074f32, 0.8327339f32),
        control_1: Point::new(0.8641892f32, 0.84287816f32),
        anchor_1: Point::new(0.8563981f32, 0.85068417f32),
    },
    Cubic {
        anchor_0: Point::new(0.8563981f32, 0.85068417f32),
        control_0: Point::new(0.8547206f32, 0.85236484f32),
        control_1: Point::new(0.85304314f32, 0.85404557f32),
        anchor_1: Point::new(0.8513657f32, 0.85572624f32),
    },
    Cubic {
        anchor_0: Point::new(0.8513657f32, 0.85572624f32),
        control_0: Point::new(0.84357464f32, 0.8635322f32),
        control_1: Point::new(0.83344007f32, 0.8685697f32),
        anchor_1: Point::new(0.8225135f32, 0.87006754f32),
    },
    Cubic {
        anchor_0: Point::new(0.8225135f32, 0.87006754f32),
        control_0: Point::new(0.7822627f32, 0.8755853f32),
        control_1: Point::new(0.7412831f32, 0.8688563f32),
        anchor_1: Point::new(0.7049102f32, 0.85075694f32),
    },
    Cubic {
        anchor_0: Point::new(0.7049102f32, 0.85075694f32),
        control_0: Point::new(0.680024f32, 0.83837336f32),
        control_1: Point::new(0.65513796f32, 0.8259898f32),
        anchor_1: Point::new(0.63025177f32, 0.8136062f32),
    },
    Cubic {
        anchor_0: Point::new(0.63025177f32, 0.8136062f32),
        control_0: Point::new(0.6214615f32, 0.8399767f32),
        control_1: Point::new(0.6126713f32, 0.8663473f32),
        anchor_1: Point::new(0.6038812f32, 0.8927179f32),
    },
    Cubic {
        anchor_0: Point::new(0.6038812f32, 0.8927179f32),
        control_0: Point::new(0.5910337f32, 0.9312602f32),
        control_1: Point::new(0.5668794f32, 0.9650416f32),
        anchor_1: Point::new(0.53456324f32, 0.9896635f32),
    },
    Cubic {
        anchor_0: Point::new(0.53456324f32, 0.9896635f32),
        control_0: Point::new(0.52579063f32, 0.9963474f32),
        control_1: Point::new(0.5150691f32, 0.99997216f32),
        anchor_1: Point::new(0.50404036f32, 0.99998266f32),
    },
    Cubic {
        anchor_0: Point::new(0.50404036f32, 0.99998266f32),
        control_0: Point::new(0.5016658f32, 0.9999849f32),
        control_1: Point::new(0.4992912f32, 0.99998724f32),
        anchor_1: Point::new(0.49691668f32, 0.9999895f32),
    },
    Cubic {
        anchor_0: Point::new(0.49691668f32, 0.9999895f32),
        control_0: Point::new(0.48588794f32, 1.0f32),
        control_1: Point::new(0.47515965f32, 0.9963958f32),
        anchor_1: Point::new(0.46637428f32, 0.98972875f32),
    },
    Cubic {
        anchor_0: Point::new(0.46637428f32, 0.98972875f32),
        control_0: Point::new(0.43401107f32, 0.96516883f32),
        control_1: Point::new(0.40979218f32, 0.93143374f32),
        anchor_1: Point::new(0.39687088f32, 0.8929161f32),
    },
    Cubic {
        anchor_0: Point::new(0.39687088f32, 0.8929161f32),
        control_0: Point::new(0.38803017f32, 0.8665623f32),
        control_1: Point::new(0.3791895f32, 0.8402086f32),
        anchor_1: Point::new(0.37034884f32, 0.81385493f32),
    },
    Cubic {
        anchor_0: Point::new(0.37034884f32, 0.81385493f32),
        control_0: Point::new(0.34548637f32, 0.8262861f32),
        control_1: Point::new(0.32062396f32, 0.83871734f32),
        anchor_1: Point::new(0.29576156f32, 0.85114855f32),
    },
    Cubic {
        anchor_0: Point::new(0.29576156f32, 0.85114855f32),
        control_0: Point::new(0.25942346f32, 0.8693176f32),
        control_1: Point::new(0.21845685f32, 0.876125f32),
        anchor_1: Point::new(0.17819555f32, 0.8706843f32),
    },
    Cubic {
        anchor_0: Point::new(0.17819555f32, 0.8706843f32),
        control_0: Point::new(0.16726615f32, 0.8692074f32),
        control_1: Point::new(0.15712193f32, 0.8641892f32),
        anchor_1: Point::new(0.14931595f32, 0.85639817f32),
    },
    Cubic {
        anchor_0: Point::new(0.14931595f32, 0.85639817f32),
        control_0: Point::new(0.14763525f32, 0.85472065f32),
        control_1: Point::new(0.14595455f32, 0.8530432f32),
        anchor_1: Point::new(0.14427383f32, 0.8513657f32),
    },
    Cubic {
        anchor_0: Point::new(0.14427383f32, 0.8513657f32),
        control_0: Point::new(0.13646787f32, 0.8435747f32),
        control_1: Point::new(0.13143033f32, 0.83344007f32),
        anchor_1: Point::new(0.12993245f32, 0.8225135f32),
    },
    Cubic {
        anchor_0: Point::new(0.12993245f32, 0.8225135f32),
        control_0: Point::new(0.12441473f32, 0.78226274f32),
        control_1: Point::new(0.13114364f32, 0.7412832f32),
        anchor_1: Point::new(0.14924306f32, 0.7049104f32),
    },
    Cubic {
        anchor_0: Point::new(0.14924306f32, 0.7049104f32),
        control_0: Point::new(0.16162665f32, 0.68002415f32),
        control_1: Point::new(0.17401026f32, 0.65513796f32),
        anchor_1: Point::new(0.18639386f32, 0.63025177f32),
    },
    Cubic {
        anchor_0: Point::new(0.18639386f32, 0.63025177f32),
        control_0: Point::new(0.16002324f32, 0.6214615f32),
        control_1: Point::new(0.13365264f32, 0.6126713f32),
        anchor_1: Point::new(0.10728206f32, 0.6038811f32),
    },
    Cubic {
        anchor_0: Point::new(0.10728206f32, 0.6038811f32),
        control_0: Point::new(0.068739764f32, 0.59103364f32),
        control_1: Point::new(0.034958445f32, 0.56687933f32),
        anchor_1: Point::new(0.010336571f32, 0.5345632f32),
    },
    Cubic {
        anchor_0: Point::new(0.010336571f32, 0.5345632f32),
        control_0: Point::new(0.0036526388f32, 0.5257906f32),
        control_1: Point::new(2.7935172E-5f32, 0.5150692f32),
        anchor_1: Point::new(1.7372884E-5f32, 0.5040404f32),
    },
    Cubic {
        anchor_0: Point::new(1.7372884E-5f32, 0.5040404f32),
        control_0: Point::new(1.51003715E-5f32, 0.5016659f32),
        control_1: Point::new(1.2827976E-5f32, 0.49929127f32),
        anchor_1: Point::new(1.0556514E-5f32, 0.49691674f32),
    },
    Cubic {
        anchor_0: Point::new(1.0556514E-5f32, 0.49691674f32),
        control_0: Point::new(0.0f32, 0.48588797f32),
        control_1: Point::new(0.0036041632f32, 0.47515967f32),
        anchor_1: Point::new(0.0102712745f32, 0.46637425f32),
    },
    Cubic {
        anchor_0: Point::new(0.0102712745f32, 0.46637425f32),
        control_0: Point::new(0.034831252f32, 0.43401098f32),
        control_1: Point::new(0.06856626f32, 0.40979213f32),
        anchor_1: Point::new(0.10708399f32, 0.39687088f32),
    },
    Cubic {
        anchor_0: Point::new(0.10708399f32, 0.39687088f32),
        control_0: Point::new(0.13343768f32, 0.3880302f32),
        control_1: Point::new(0.15979138f32, 0.37918955f32),
        anchor_1: Point::new(0.1861451f32, 0.3703489f32),
    },
    Cubic {
        anchor_0: Point::new(0.1861451f32, 0.3703489f32),
        control_0: Point::new(0.17371388f32, 0.34548646f32),
        control_1: Point::new(0.1612827f32, 0.32062405f32),
        anchor_1: Point::new(0.14885145f32, 0.29576162f32),
    },
    Cubic {
        anchor_0: Point::new(0.14885145f32, 0.29576162f32),
        control_0: Point::new(0.13068242f32, 0.25942352f32),
        control_1: Point::new(0.12387508f32, 0.21845686f32),
        anchor_1: Point::new(0.12931578f32, 0.17819555f32),
    },
    Cubic {
        anchor_0: Point::new(0.12931578f32, 0.17819555f32),
        control_0: Point::new(0.13079274f32, 0.16726612f32),
        control_1: Point::new(0.13581087f32, 0.15712188f32),
        anchor_1: Point::new(0.14360192f32, 0.14931588f32),
    },
    Cubic {
        anchor_0: Point::new(0.14360192f32, 0.14931588f32),
        control_0: Point::new(0.1452794f32, 0.1476352f32),
        control_1: Point::new(0.14695683f32, 0.14595453f32),
        anchor_1: Point::new(0.1486343f32, 0.14427386f32),
    },
    Cubic {
        anchor_0: Point::new(0.1486343f32, 0.14427386f32),
        control_0: Point::new(0.15642537f32, 0.13646789f32),
        control_1: Point::new(0.16656f32, 0.13143034f32),
        anchor_1: Point::new(0.17748658f32, 0.1299325f32),
    },
    Cubic {
        anchor_0: Point::new(0.17748658f32, 0.1299325f32),
        control_0: Point::new(0.21773721f32, 0.12441481f32),
        control_1: Point::new(0.258717f32, 0.13114373f32),
        anchor_1: Point::new(0.29508966f32, 0.14924309f32),
    },
    Cubic {
        anchor_0: Point::new(0.29508966f32, 0.14924309f32),
        control_0: Point::new(0.31997588f32, 0.16162667f32),
        control_1: Point::new(0.3448621f32, 0.17401028f32),
        anchor_1: Point::new(0.36974832f32, 0.18639387f32),
    },
];
static FLOWER_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: FLOWER_CUBICS,
}];
pub const FLOWER: ShapeDataStored = ShapeDataStored {
    features: FLOWER_FEATURES,
};

static GEM_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.4994853f32, 0.99998444f32),
        control_0: Point::new(0.4913765f32, 0.99996895f32),
        control_1: Point::new(0.4832684f32, 0.9995445f32),
        anchor_1: Point::new(0.47519168f32, 0.9987113f32),
    },
    Cubic {
        anchor_0: Point::new(0.47519168f32, 0.9987113f32),
        control_0: Point::new(0.4356124f32, 0.99462795f32),
        control_1: Point::new(0.3976227f32, 0.97721606f32),
        anchor_1: Point::new(0.32164326f32, 0.94239205f32),
    },
    Cubic {
        anchor_0: Point::new(0.32164326f32, 0.94239205f32),
        control_0: Point::new(0.26002494f32, 0.9141503f32),
        control_1: Point::new(0.19840662f32, 0.8859086f32),
        anchor_1: Point::new(0.13678831f32, 0.8576669f32),
    },
    Cubic {
        anchor_0: Point::new(0.13678831f32, 0.8576669f32),
        control_0: Point::new(0.052739035f32, 0.8191443f32),
        control_1: Point::new(0.0041653397f32, 0.7298057f32),
        anchor_1: Point::new(0.01752848f32, 0.6383197f32),
    },
    Cubic {
        anchor_0: Point::new(0.01752848f32, 0.6383197f32),
        control_0: Point::new(0.031362873f32, 0.5436074f32),
        control_1: Point::new(0.045197275f32, 0.44889516f32),
        anchor_1: Point::new(0.059031673f32, 0.35418293f32),
    },
    Cubic {
        anchor_0: Point::new(0.059031673f32, 0.35418293f32),
        control_0: Point::new(0.06799148f32, 0.29284286f32),
        control_1: Point::new(0.101520166f32, 0.23778714f32),
        anchor_1: Point::new(0.15190747f32, 0.2016762f32),
    },
    Cubic {
        anchor_0: Point::new(0.15190747f32, 0.2016762f32),
        control_0: Point::new(0.22730288f32, 0.14764284f32),
        control_1: Point::new(0.30269828f32, 0.09360947f32),
        anchor_1: Point::new(0.3780937f32, 0.03957611f32),
    },
    Cubic {
        anchor_0: Point::new(0.3780937f32, 0.03957611f32),
        control_0: Point::new(0.41404003f32, 0.013814549f32),
        control_1: Point::new(0.4571729f32, 0.0f32),
        anchor_1: Point::new(0.50139725f32, 8.4575615E-5f32),
    },
    Cubic {
        anchor_0: Point::new(0.50139725f32, 8.4575615E-5f32),
        control_0: Point::new(0.5456216f32, 1.6916129E-4f32),
        control_1: Point::new(0.58870137f32, 0.01414858f32),
        anchor_1: Point::new(0.6245489f32, 0.040047407f32),
    },
    Cubic {
        anchor_0: Point::new(0.6245489f32, 0.040047407f32),
        control_0: Point::new(0.6997371f32, 0.094368696f32),
        control_1: Point::new(0.7749252f32, 0.14868999f32),
        anchor_1: Point::new(0.8501134f32, 0.20301129f32),
    },
    Cubic {
        anchor_0: Point::new(0.8501134f32, 0.20301129f32),
        control_0: Point::new(0.9003623f32, 0.23931468f32),
        control_1: Point::new(0.93368006f32, 0.29449806f32),
        anchor_1: Point::new(0.9424052f32, 0.35587198f32),
    },
    Cubic {
        anchor_0: Point::new(0.9424052f32, 0.35587198f32),
        control_0: Point::new(0.9558773f32, 0.45063642f32),
        control_1: Point::new(0.9693495f32, 0.54540086f32),
        anchor_1: Point::new(0.9828215f32, 0.64016527f32),
    },
    Cubic {
        anchor_0: Point::new(0.9828215f32, 0.64016527f32),
        control_0: Point::new(0.9958347f32, 0.73170173f32),
        control_1: Point::new(0.9469198f32, 0.82085395f32),
        anchor_1: Point::new(0.8627239f32, 0.85905486f32),
    },
    Cubic {
        anchor_0: Point::new(0.8627239f32, 0.85905486f32),
        control_0: Point::new(0.800998f32, 0.88706076f32),
        control_1: Point::new(0.73927206f32, 0.91506666f32),
        anchor_1: Point::new(0.67754626f32, 0.94307256f32),
    },
    Cubic {
        anchor_0: Point::new(0.67754626f32, 0.94307256f32),
        control_0: Point::new(0.6014343f32, 0.97760564f32),
        control_1: Point::new(0.5633782f32, 0.9948722f32),
        anchor_1: Point::new(0.5237836f32, 0.9988042f32),
    },
    Cubic {
        anchor_0: Point::new(0.5237836f32, 0.9988042f32),
        control_0: Point::new(0.5157038f32, 0.9996066f32),
        control_1: Point::new(0.50759417f32, 1.0f32),
        anchor_1: Point::new(0.4994853f32, 0.99998444f32),
    },
];
static GEM_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: GEM_CUBICS,
}];
pub const GEM: ShapeDataStored = ShapeDataStored {
    features: GEM_FEATURES,
};

static GHOSTISH_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.0f32),
        control_0: Point::new(0.763236f32, 0.0f32),
        control_1: Point::new(0.976631f32, 0.213395f32),
        anchor_1: Point::new(0.976631f32, 0.47663105f32),
    },
    Cubic {
        anchor_0: Point::new(0.976631f32, 0.47663105f32),
        control_0: Point::new(0.976631f32, 0.57110304f32),
        control_1: Point::new(0.976631f32, 0.665575f32),
        anchor_1: Point::new(0.976631f32, 0.7600471f32),
    },
    Cubic {
        anchor_0: Point::new(0.976631f32, 0.7600471f32),
        control_0: Point::new(0.976631f32, 0.9069215f32),
        control_1: Point::new(0.81912905f32, 0.99999976f32),
        anchor_1: Point::new(0.6904673f32, 0.92916036f32),
    },
    Cubic {
        anchor_0: Point::new(0.6904673f32, 0.92916036f32),
        control_0: Point::new(0.6685608f32, 0.9170989f32),
        control_1: Point::new(0.6466543f32, 0.90503746f32),
        anchor_1: Point::new(0.6247478f32, 0.8929759f32),
    },
    Cubic {
        anchor_0: Point::new(0.6247478f32, 0.8929759f32),
        control_0: Point::new(0.58981365f32, 0.8737416f32),
        control_1: Point::new(0.55058235f32, 0.8636553f32),
        anchor_1: Point::new(0.51070315f32, 0.8636554f32),
    },
    Cubic {
        anchor_0: Point::new(0.51070315f32, 0.8636554f32),
        control_0: Point::new(0.50356764f32, 0.8636554f32),
        control_1: Point::new(0.49643224f32, 0.8636554f32),
        anchor_1: Point::new(0.48929676f32, 0.8636554f32),
    },
    Cubic {
        anchor_0: Point::new(0.48929676f32, 0.8636554f32),
        control_0: Point::new(0.44941753f32, 0.8636554f32),
        control_1: Point::new(0.41018623f32, 0.8737416f32),
        anchor_1: Point::new(0.37525213f32, 0.892976f32),
    },
    Cubic {
        anchor_0: Point::new(0.37525213f32, 0.892976f32),
        control_0: Point::new(0.35334563f32, 0.9050374f32),
        control_1: Point::new(0.33143914f32, 0.91709894f32),
        anchor_1: Point::new(0.30953255f32, 0.92916036f32),
    },
    Cubic {
        anchor_0: Point::new(0.30953255f32, 0.92916036f32),
        control_0: Point::new(0.18087082f32, 1.0f32),
        control_1: Point::new(0.023368964f32, 0.90692157f32),
        anchor_1: Point::new(0.023368983f32, 0.76004714f32),
    },
    Cubic {
        anchor_0: Point::new(0.023368983f32, 0.76004714f32),
        control_0: Point::new(0.023368997f32, 0.665575f32),
        control_1: Point::new(0.023369016f32, 0.5711029f32),
        anchor_1: Point::new(0.023369035f32, 0.47663087f32),
    },
    Cubic {
        anchor_0: Point::new(0.023369035f32, 0.47663087f32),
        control_0: Point::new(0.023369083f32, 0.21339487f32),
        control_1: Point::new(0.23676398f32, 2.5438656E-8f32),
        anchor_1: Point::new(0.5f32, 0.0f32),
    },
];
static GHOSTISH_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: GHOSTISH_CUBICS,
}];
pub const GHOSTISH: ShapeDataStored = ShapeDataStored {
    features: GHOSTISH_FEATURES,
};

static HEART_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.28592262f32),
        control_0: Point::new(0.50162935f32, 0.28592262f32),
        control_1: Point::new(0.5032587f32, 0.28525198f32),
        anchor_1: Point::new(0.50443137f32, 0.28391066f32),
    },
    Cubic {
        anchor_0: Point::new(0.50443137f32, 0.28391066f32),
        control_0: Point::new(0.54294467f32, 0.23985782f32),
        control_1: Point::new(0.5814579f32, 0.19580503f32),
        anchor_1: Point::new(0.61997116f32, 0.1517522f32),
    },
    Cubic {
        anchor_0: Point::new(0.61997116f32, 0.1517522f32),
        control_0: Point::new(0.7046104f32, 0.054938894f32),
        control_1: Point::new(0.85627925f32, 0.058467712f32),
        anchor_1: Point::new(0.93632436f32, 0.15911266f32),
    },
    Cubic {
        anchor_0: Point::new(0.93632436f32, 0.15911266f32),
        control_0: Point::new(1.0f32, 0.2391755f32),
        control_1: Point::new(0.9984658f32, 0.35301173f32),
        anchor_1: Point::new(0.9326556f32, 0.43132946f32),
    },
    Cubic {
        anchor_0: Point::new(0.9326556f32, 0.43132946f32),
        control_0: Point::new(0.78896713f32, 0.6023264f32),
        control_1: Point::new(0.6452787f32, 0.7733233f32),
        anchor_1: Point::new(0.5015903f32, 0.9443202f32),
    },
    Cubic {
        anchor_0: Point::new(0.5015903f32, 0.9443202f32),
        control_0: Point::new(0.5011956f32, 0.9447899f32),
        control_1: Point::new(0.5006135f32, 0.9450611f32),
        anchor_1: Point::new(0.5f32, 0.9450611f32),
    },
    Cubic {
        anchor_0: Point::new(0.5f32, 0.9450611f32),
        control_0: Point::new(0.4993865f32, 0.9450611f32),
        control_1: Point::new(0.4988044f32, 0.9447899f32),
        anchor_1: Point::new(0.49840972f32, 0.9443202f32),
    },
    Cubic {
        anchor_0: Point::new(0.49840972f32, 0.9443202f32),
        control_0: Point::new(0.35472128f32, 0.77332324f32),
        control_1: Point::new(0.21103285f32, 0.6023264f32),
        anchor_1: Point::new(0.06734445f32, 0.43132952f32),
    },
    Cubic {
        anchor_0: Point::new(0.06734445f32, 0.43132952f32),
        control_0: Point::new(0.0015341921f32, 0.35301176f32),
        control_1: Point::new(0.0f32, 0.23917544f32),
        anchor_1: Point::new(0.063675694f32, 0.15911256f32),
    },
    Cubic {
        anchor_0: Point::new(0.063675694f32, 0.15911256f32),
        control_0: Point::new(0.14372082f32, 0.05846764f32),
        control_1: Point::new(0.2953897f32, 0.054938894f32),
        anchor_1: Point::new(0.38002893f32, 0.15175223f32),
    },
    Cubic {
        anchor_0: Point::new(0.38002893f32, 0.15175223f32),
        control_0: Point::new(0.41854212f32, 0.19580503f32),
        control_1: Point::new(0.45705533f32, 0.23985782f32),
        anchor_1: Point::new(0.49556857f32, 0.28391063f32),
    },
    Cubic {
        anchor_0: Point::new(0.49556857f32, 0.28391063f32),
        control_0: Point::new(0.49674124f32, 0.28525195f32),
        control_1: Point::new(0.4983706f32, 0.2859226f32),
        anchor_1: Point::new(0.5f32, 0.28592262f32),
    },
];
static HEART_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: HEART_CUBICS,
}];
pub const HEART: ShapeDataStored = ShapeDataStored {
    features: HEART_FEATURES,
};

static OVAL_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.9084564f32, 0.091543645f32),
        control_0: Point::new(0.942462f32, 0.12554933f32),
        control_1: Point::new(0.9661038f32, 0.16991898f32),
        anchor_1: Point::new(0.97740257f32, 0.22267362f32),
    },
    Cubic {
        anchor_0: Point::new(0.97740257f32, 0.22267362f32),
        control_0: Point::new(1.0f32, 0.32818276f32),
        control_1: Point::new(0.97137433f32, 0.45858875f32),
        anchor_1: Point::new(0.89782274f32, 0.5852039f32),
    },
    Cubic {
        anchor_0: Point::new(0.89782274f32, 0.5852039f32),
        control_0: Point::new(0.82427114f32, 0.7118188f32),
        control_1: Point::new(0.71181875f32, 0.8242712f32),
        anchor_1: Point::new(0.58520377f32, 0.89782274f32),
    },
    Cubic {
        anchor_0: Point::new(0.58520377f32, 0.89782274f32),
        control_0: Point::new(0.45858875f32, 0.9713742f32),
        control_1: Point::new(0.32818276f32, 1.0f32),
        anchor_1: Point::new(0.22267361f32, 0.97740257f32),
    },
    Cubic {
        anchor_0: Point::new(0.22267361f32, 0.97740257f32),
        control_0: Point::new(0.11716447f32, 0.95480514f32),
        control_1: Point::new(0.04519486f32, 0.8828355f32),
        anchor_1: Point::new(0.02259743f32, 0.77732635f32),
    },
    Cubic {
        anchor_0: Point::new(0.02259743f32, 0.77732635f32),
        control_0: Point::new(0.0f32, 0.6718172f32),
        control_1: Point::new(0.028625693f32, 0.5414112f32),
        anchor_1: Point::new(0.10217724f32, 0.41479623f32),
    },
    Cubic {
        anchor_0: Point::new(0.10217724f32, 0.41479623f32),
        control_0: Point::new(0.17572875f32, 0.28818125f32),
        control_1: Point::new(0.28818122f32, 0.17572889f32),
        anchor_1: Point::new(0.41479647f32, 0.10217724f32),
    },
    Cubic {
        anchor_0: Point::new(0.41479647f32, 0.10217724f32),
        control_0: Point::new(0.5414115f32, 0.028625727f32),
        control_1: Point::new(0.67181736f32, 3.4430297E-8f32),
        anchor_1: Point::new(0.77732646f32, 0.0225975f32),
    },
    Cubic {
        anchor_0: Point::new(0.77732646f32, 0.0225975f32),
        control_0: Point::new(0.83008105f32, 0.03389623f32),
        control_1: Point::new(0.87445074f32, 0.057537958f32),
        anchor_1: Point::new(0.9084564f32, 0.091543645f32),
    },
];
static OVAL_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: OVAL_CUBICS,
}];
pub const OVAL: ShapeDataStored = ShapeDataStored {
    features: OVAL_FEATURES,
};

static PENTAGON_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.4999999f32, 0.04283998f32),
        control_0: Point::new(0.53375417f32, 0.04283998f32),
        control_1: Point::new(0.56750846f32, 0.053037085f32),
        anchor_1: Point::new(0.5964094f32, 0.0734313f32),
    },
    Cubic {
        anchor_0: Point::new(0.5964094f32, 0.0734313f32),
        control_0: Point::new(0.7037436f32, 0.14917278f32),
        control_1: Point::new(0.8110778f32, 0.22491425f32),
        anchor_1: Point::new(0.9184119f32, 0.30065572f32),
    },
    Cubic {
        anchor_0: Point::new(0.9184119f32, 0.30065572f32),
        control_0: Point::new(0.97590053f32, 0.3412231f32),
        control_1: Point::new(1.0f32, 0.4146785f32),
        anchor_1: Point::new(0.97771674f32, 0.48141766f32),
    },
    Cubic {
        anchor_0: Point::new(0.97771674f32, 0.48141766f32),
        control_0: Point::new(0.9372636f32, 0.60257673f32),
        control_1: Point::new(0.89681053f32, 0.72373587f32),
        anchor_1: Point::new(0.85635746f32, 0.844895f32),
    },
    Cubic {
        anchor_0: Point::new(0.85635746f32, 0.844895f32),
        control_0: Point::new(0.8339704f32, 0.91194516f32),
        control_1: Point::new(0.7712053f32, 0.95716006f32),
        anchor_1: Point::new(0.7005165f32, 0.95716006f32),
    },
    Cubic {
        anchor_0: Point::new(0.7005165f32, 0.95716006f32),
        control_0: Point::new(0.56683874f32, 0.95716f32),
        control_1: Point::new(0.43316096f32, 0.95716f32),
        anchor_1: Point::new(0.29948324f32, 0.95716006f32),
    },
    Cubic {
        anchor_0: Point::new(0.29948324f32, 0.95716006f32),
        control_0: Point::new(0.22879441f32, 0.95716006f32),
        control_1: Point::new(0.16602935f32, 0.9119451f32),
        anchor_1: Point::new(0.14364237f32, 0.8448949f32),
    },
    Cubic {
        anchor_0: Point::new(0.14364237f32, 0.8448949f32),
        control_0: Point::new(0.103189275f32, 0.72373575f32),
        control_1: Point::new(0.0627362f32, 0.6025766f32),
        anchor_1: Point::new(0.022283113f32, 0.48141748f32),
    },
    Cubic {
        anchor_0: Point::new(0.022283113f32, 0.48141748f32),
        control_0: Point::new(0.0f32, 0.4146784f32),
        control_1: Point::new(0.024099478f32, 0.34122294f32),
        anchor_1: Point::new(0.08158801f32, 0.3006556f32),
    },
    Cubic {
        anchor_0: Point::new(0.08158801f32, 0.3006556f32),
        control_0: Point::new(0.18892215f32, 0.22491416f32),
        control_1: Point::new(0.2962563f32, 0.14917272f32),
        anchor_1: Point::new(0.4035904f32, 0.0734313f32),
    },
    Cubic {
        anchor_0: Point::new(0.4035904f32, 0.0734313f32),
        control_0: Point::new(0.4324913f32, 0.053037085f32),
        control_1: Point::new(0.46624562f32, 0.04283998f32),
        anchor_1: Point::new(0.4999999f32, 0.04283998f32),
    },
];
static PENTAGON_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: PENTAGON_CUBICS,
}];
pub const PENTAGON: ShapeDataStored = ShapeDataStored {
    features: PENTAGON_FEATURES,
};

static PILL_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.8731647f32, 0.12683535f32),
        control_0: Point::new(0.9403969f32, 0.19406751f32),
        control_1: Point::new(0.98515487f32, 0.2837738f32),
        anchor_1: Point::new(0.9954581f32, 0.38397372f32),
    },
    Cubic {
        anchor_0: Point::new(0.9954581f32, 0.38397372f32),
        control_0: Point::new(0.9969721f32, 0.39869705f32),
        control_1: Point::new(0.998486f32, 0.4134204f32),
        anchor_1: Point::new(1.0f32, 0.4281437f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.4281437f32),
        control_0: Point::new(0.9993606f32, 0.54387325f32),
        control_1: Point::new(0.95310354f32, 0.6546809f32),
        anchor_1: Point::new(0.8712691f32, 0.7365152f32),
    },
    Cubic {
        anchor_0: Point::new(0.8712691f32, 0.7365152f32),
        control_0: Point::new(0.8263512f32, 0.78143317f32),
        control_1: Point::new(0.7814332f32, 0.82635117f32),
        anchor_1: Point::new(0.73651534f32, 0.8712691f32),
    },
    Cubic {
        anchor_0: Point::new(0.73651534f32, 0.8712691f32),
        control_0: Point::new(0.65468085f32, 0.9531036f32),
        control_1: Point::new(0.5438733f32, 0.9993605f32),
        anchor_1: Point::new(0.4281436f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.4281436f32, 1.0f32),
        control_0: Point::new(0.41342032f32, 0.998486f32),
        control_1: Point::new(0.39869702f32, 0.9969721f32),
        anchor_1: Point::new(0.38397372f32, 0.9954581f32),
    },
    Cubic {
        anchor_0: Point::new(0.38397372f32, 0.9954581f32),
        control_0: Point::new(0.18357389f32, 0.9748515f32),
        control_1: Point::new(0.025148578f32, 0.81642616f32),
        anchor_1: Point::new(0.004541943f32, 0.61602634f32),
    },
    Cubic {
        anchor_0: Point::new(0.004541943f32, 0.61602634f32),
        control_0: Point::new(0.0030279716f32, 0.601303f32),
        control_1: Point::new(0.0015140006f32, 0.5865796f32),
        anchor_1: Point::new(0.0f32, 0.57185626f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.57185626f32),
        control_0: Point::new(6.394614E-4f32, 0.45612672f32),
        control_1: Point::new(0.046896663f32, 0.34531894f32),
        anchor_1: Point::new(0.12873106f32, 0.2634846f32),
    },
    Cubic {
        anchor_0: Point::new(0.12873106f32, 0.2634846f32),
        control_0: Point::new(0.17364901f32, 0.21856667f32),
        control_1: Point::new(0.21856697f32, 0.17364874f32),
        anchor_1: Point::new(0.2634849f32, 0.12873076f32),
    },
    Cubic {
        anchor_0: Point::new(0.2634849f32, 0.12873076f32),
        control_0: Point::new(0.34531933f32, 0.046896398f32),
        control_1: Point::new(0.4561269f32, 6.393849E-4f32),
        anchor_1: Point::new(0.57185644f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.57185644f32, 0.0f32),
        control_0: Point::new(0.58657974f32, 0.001513966f32),
        control_1: Point::new(0.60130304f32, 0.003027932f32),
        anchor_1: Point::new(0.61602634f32, 0.0045418795f32),
    },
    Cubic {
        anchor_0: Point::new(0.61602634f32, 0.0045418795f32),
        control_0: Point::new(0.7162262f32, 0.014845201f32),
        control_1: Point::new(0.8059326f32, 0.059603203f32),
        anchor_1: Point::new(0.8731647f32, 0.12683535f32),
    },
];
static PILL_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: PILL_CUBICS,
}];
pub const PILL: ShapeDataStored = ShapeDataStored {
    features: PILL_FEATURES,
};

static PIXEL_CIRCLE_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.49999997f32, 0.0f32),
        control_0: Point::new(0.56799996f32, 0.0f32),
        control_1: Point::new(0.636f32, 0.0f32),
        anchor_1: Point::new(0.704f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.704f32, 0.0f32),
        control_0: Point::new(0.704f32, 0.021666666f32),
        control_1: Point::new(0.704f32, 0.043333333f32),
        anchor_1: Point::new(0.704f32, 0.065f32),
    },
    Cubic {
        anchor_0: Point::new(0.704f32, 0.065f32),
        control_0: Point::new(0.7503333f32, 0.065f32),
        control_1: Point::new(0.7966667f32, 0.065f32),
        anchor_1: Point::new(0.84300005f32, 0.065f32),
    },
    Cubic {
        anchor_0: Point::new(0.84300005f32, 0.065f32),
        control_0: Point::new(0.843f32, 0.09266666f32),
        control_1: Point::new(0.84300005f32, 0.12033333f32),
        anchor_1: Point::new(0.84300005f32, 0.148f32),
    },
    Cubic {
        anchor_0: Point::new(0.84300005f32, 0.148f32),
        control_0: Point::new(0.8706666f32, 0.148f32),
        control_1: Point::new(0.8983333f32, 0.148f32),
        anchor_1: Point::new(0.926f32, 0.148f32),
    },
    Cubic {
        anchor_0: Point::new(0.926f32, 0.148f32),
        control_0: Point::new(0.926f32, 0.19733334f32),
        control_1: Point::new(0.926f32, 0.24666667f32),
        anchor_1: Point::new(0.926f32, 0.296f32),
    },
    Cubic {
        anchor_0: Point::new(0.926f32, 0.296f32),
        control_0: Point::new(0.95066667f32, 0.296f32),
        control_1: Point::new(0.97533333f32, 0.296f32),
        anchor_1: Point::new(1.0f32, 0.296f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.296f32),
        control_0: Point::new(1.0f32, 0.43199998f32),
        control_1: Point::new(1.0f32, 0.568f32),
        anchor_1: Point::new(1.0f32, 0.704f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.704f32),
        control_0: Point::new(0.97533333f32, 0.704f32),
        control_1: Point::new(0.95066667f32, 0.704f32),
        anchor_1: Point::new(0.926f32, 0.704f32),
    },
    Cubic {
        anchor_0: Point::new(0.926f32, 0.704f32),
        control_0: Point::new(0.926f32, 0.75333333f32),
        control_1: Point::new(0.926f32, 0.80266666f32),
        anchor_1: Point::new(0.926f32, 0.852f32),
    },
    Cubic {
        anchor_0: Point::new(0.926f32, 0.852f32),
        control_0: Point::new(0.8983333f32, 0.852f32),
        control_1: Point::new(0.8706667f32, 0.852f32),
        anchor_1: Point::new(0.84300005f32, 0.852f32),
    },
    Cubic {
        anchor_0: Point::new(0.84300005f32, 0.852f32),
        control_0: Point::new(0.843f32, 0.8796666f32),
        control_1: Point::new(0.84300005f32, 0.9073333f32),
        anchor_1: Point::new(0.84300005f32, 0.935f32),
    },
    Cubic {
        anchor_0: Point::new(0.84300005f32, 0.935f32),
        control_0: Point::new(0.7966667f32, 0.93499994f32),
        control_1: Point::new(0.7503334f32, 0.93499994f32),
        anchor_1: Point::new(0.7040001f32, 0.93499994f32),
    },
    Cubic {
        anchor_0: Point::new(0.7040001f32, 0.93499994f32),
        control_0: Point::new(0.70400006f32, 0.9566666f32),
        control_1: Point::new(0.70400006f32, 0.97833335f32),
        anchor_1: Point::new(0.704f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.704f32, 1.0f32),
        control_0: Point::new(0.6359999f32, 1.0f32),
        control_1: Point::new(0.56799996f32, 1.0f32),
        anchor_1: Point::new(0.49999997f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.49999997f32, 1.0f32),
        control_0: Point::new(0.43199992f32, 0.99999994f32),
        control_1: Point::new(0.36399996f32, 0.99999994f32),
        anchor_1: Point::new(0.29599994f32, 0.99999994f32),
    },
    Cubic {
        anchor_0: Point::new(0.29599994f32, 0.99999994f32),
        control_0: Point::new(0.29599994f32, 0.97833323f32),
        control_1: Point::new(0.2959999f32, 0.9566666f32),
        anchor_1: Point::new(0.2959999f32, 0.93499994f32),
    },
    Cubic {
        anchor_0: Point::new(0.2959999f32, 0.93499994f32),
        control_0: Point::new(0.2496666f32, 0.93499994f32),
        control_1: Point::new(0.2033333f32, 0.93499994f32),
        anchor_1: Point::new(0.157f32, 0.935f32),
    },
    Cubic {
        anchor_0: Point::new(0.157f32, 0.935f32),
        control_0: Point::new(0.15699998f32, 0.90733325f32),
        control_1: Point::new(0.15699998f32, 0.87966657f32),
        anchor_1: Point::new(0.15699995f32, 0.85199994f32),
    },
    Cubic {
        anchor_0: Point::new(0.15699995f32, 0.85199994f32),
        control_0: Point::new(0.12933327f32, 0.8519999f32),
        control_1: Point::new(0.10166663f32, 0.8519999f32),
        anchor_1: Point::new(0.07399997f32, 0.85199994f32),
    },
    Cubic {
        anchor_0: Point::new(0.07399997f32, 0.85199994f32),
        control_0: Point::new(0.073999986f32, 0.8026666f32),
        control_1: Point::new(0.074000016f32, 0.75333333f32),
        anchor_1: Point::new(0.07400003f32, 0.70400006f32),
    },
    Cubic {
        anchor_0: Point::new(0.07400003f32, 0.70400006f32),
        control_0: Point::new(0.049333353f32, 0.704f32),
        control_1: Point::new(0.024666676f32, 0.704f32),
        anchor_1: Point::new(0.0f32, 0.704f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.704f32),
        control_0: Point::new(3.973643E-8f32, 0.5679999f32),
        control_1: Point::new(7.947286E-8f32, 0.43199986f32),
        anchor_1: Point::new(1.1920929E-7f32, 0.2959998f32),
    },
    Cubic {
        anchor_0: Point::new(1.1920929E-7f32, 0.2959998f32),
        control_0: Point::new(0.024666747f32, 0.29599985f32),
        control_1: Point::new(0.049333375f32, 0.2959999f32),
        anchor_1: Point::new(0.074f32, 0.29599997f32),
    },
    Cubic {
        anchor_0: Point::new(0.074f32, 0.29599997f32),
        control_0: Point::new(0.07400001f32, 0.24666664f32),
        control_1: Point::new(0.07400002f32, 0.19733332f32),
        anchor_1: Point::new(0.07400003f32, 0.148f32),
    },
    Cubic {
        anchor_0: Point::new(0.07400003f32, 0.148f32),
        control_0: Point::new(0.10166672f32, 0.14799997f32),
        control_1: Point::new(0.1293334f32, 0.14799994f32),
        anchor_1: Point::new(0.1570001f32, 0.14799991f32),
    },
    Cubic {
        anchor_0: Point::new(0.1570001f32, 0.14799991f32),
        control_0: Point::new(0.15700008f32, 0.12033325f32),
        control_1: Point::new(0.15700006f32, 0.092666596f32),
        anchor_1: Point::new(0.15700006f32, 0.06499994f32),
    },
    Cubic {
        anchor_0: Point::new(0.15700006f32, 0.06499994f32),
        control_0: Point::new(0.20333338f32, 0.06499995f32),
        control_1: Point::new(0.24966669f32, 0.064999975f32),
        anchor_1: Point::new(0.296f32, 0.065f32),
    },
    Cubic {
        anchor_0: Point::new(0.296f32, 0.065f32),
        control_0: Point::new(0.29599997f32, 0.04333333f32),
        control_1: Point::new(0.29599997f32, 0.021666665f32),
        anchor_1: Point::new(0.29599997f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.29599997f32, 0.0f32),
        control_0: Point::new(0.36399996f32, 0.0f32),
        control_1: Point::new(0.43199998f32, 0.0f32),
        anchor_1: Point::new(0.49999997f32, 0.0f32),
    },
];
static PIXEL_CIRCLE_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: PIXEL_CIRCLE_CUBICS,
}];
pub const PIXEL_CIRCLE: ShapeDataStored = ShapeDataStored {
    features: PIXEL_CIRCLE_FEATURES,
};

static PIXEL_TRIANGLE_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.11100009f32, 0.49999988f32),
        control_0: Point::new(0.11200009f32, 0.33333322f32),
        control_1: Point::new(0.1130001f32, 0.16666661f32),
        anchor_1: Point::new(0.11400011f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.11400011f32, 0.0f32),
        control_0: Point::new(0.17200007f32, 1.9868212E-8f32),
        control_1: Point::new(0.23000005f32, 3.973642E-8f32),
        anchor_1: Point::new(0.28800002f32, 5.960463E-8f32),
    },
    Cubic {
        anchor_0: Point::new(0.28800002f32, 5.960463E-8f32),
        control_0: Point::new(0.28800002f32, 0.029000057f32),
        control_1: Point::new(0.28800005f32, 0.058000054f32),
        anchor_1: Point::new(0.28800005f32, 0.08700005f32),
    },
    Cubic {
        anchor_0: Point::new(0.28800005f32, 0.08700005f32),
        control_0: Point::new(0.33266667f32, 0.08700005f32),
        control_1: Point::new(0.3773333f32, 0.08700005f32),
        anchor_1: Point::new(0.422f32, 0.08700005f32),
    },
    Cubic {
        anchor_0: Point::new(0.422f32, 0.08700005f32),
        control_0: Point::new(0.42199996f32, 0.11466671f32),
        control_1: Point::new(0.42200002f32, 0.14233337f32),
        anchor_1: Point::new(0.42200002f32, 0.17000003f32),
    },
    Cubic {
        anchor_0: Point::new(0.42200002f32, 0.17000003f32),
        control_0: Point::new(0.4683333f32, 0.17000003f32),
        control_1: Point::new(0.5146666f32, 0.17000003f32),
        anchor_1: Point::new(0.561f32, 0.17000003f32),
    },
    Cubic {
        anchor_0: Point::new(0.561f32, 0.17000003f32),
        control_0: Point::new(0.56099993f32, 0.20166668f32),
        control_1: Point::new(0.561f32, 0.23333333f32),
        anchor_1: Point::new(0.561f32, 0.265f32),
    },
    Cubic {
        anchor_0: Point::new(0.561f32, 0.265f32),
        control_0: Point::new(0.599f32, 0.265f32),
        control_1: Point::new(0.63699996f32, 0.265f32),
        anchor_1: Point::new(0.67499995f32, 0.265f32),
    },
    Cubic {
        anchor_0: Point::new(0.67499995f32, 0.265f32),
        control_0: Point::new(0.67533326f32, 0.29133332f32),
        control_1: Point::new(0.6756666f32, 0.31766662f32),
        anchor_1: Point::new(0.67599994f32, 0.34399998f32),
    },
    Cubic {
        anchor_0: Point::new(0.67599994f32, 0.34399998f32),
        control_0: Point::new(0.7139999f32, 0.34399995f32),
        control_1: Point::new(0.752f32, 0.34399998f32),
        anchor_1: Point::new(0.7899999f32, 0.34399998f32),
    },
    Cubic {
        anchor_0: Point::new(0.7899999f32, 0.34399998f32),
        control_0: Point::new(0.7899999f32, 0.37566665f32),
        control_1: Point::new(0.78999996f32, 0.4073333f32),
        anchor_1: Point::new(0.7899999f32, 0.43899995f32),
    },
    Cubic {
        anchor_0: Point::new(0.7899999f32, 0.43899995f32),
        control_0: Point::new(0.8229999f32, 0.43899992f32),
        control_1: Point::new(0.85599995f32, 0.43899995f32),
        anchor_1: Point::new(0.8889999f32, 0.43899995f32),
    },
    Cubic {
        anchor_0: Point::new(0.8889999f32, 0.43899995f32),
        control_0: Point::new(0.8889998f32, 0.47966662f32),
        control_1: Point::new(0.8889999f32, 0.52033335f32),
        anchor_1: Point::new(0.8889999f32, 0.56100005f32),
    },
    Cubic {
        anchor_0: Point::new(0.8889999f32, 0.56100005f32),
        control_0: Point::new(0.8559998f32, 0.561f32),
        control_1: Point::new(0.82299984f32, 0.561f32),
        anchor_1: Point::new(0.78999984f32, 0.561f32),
    },
    Cubic {
        anchor_0: Point::new(0.78999984f32, 0.561f32),
        control_0: Point::new(0.78999984f32, 0.5926666f32),
        control_1: Point::new(0.7899999f32, 0.6243333f32),
        anchor_1: Point::new(0.7899999f32, 0.6559999f32),
    },
    Cubic {
        anchor_0: Point::new(0.7899999f32, 0.6559999f32),
        control_0: Point::new(0.75199986f32, 0.6559999f32),
        control_1: Point::new(0.7139998f32, 0.65599996f32),
        anchor_1: Point::new(0.67599976f32, 0.656f32),
    },
    Cubic {
        anchor_0: Point::new(0.67599976f32, 0.656f32),
        control_0: Point::new(0.6756664f32, 0.68233335f32),
        control_1: Point::new(0.675333f32, 0.7086667f32),
        anchor_1: Point::new(0.6749997f32, 0.7350001f32),
    },
    Cubic {
        anchor_0: Point::new(0.6749997f32, 0.7350001f32),
        control_0: Point::new(0.6369998f32, 0.735f32),
        control_1: Point::new(0.599f32, 0.73499995f32),
        anchor_1: Point::new(0.56100005f32, 0.7349999f32),
    },
    Cubic {
        anchor_0: Point::new(0.56100005f32, 0.7349999f32),
        control_0: Point::new(0.56099993f32, 0.76666653f32),
        control_1: Point::new(0.56099975f32, 0.7983333f32),
        anchor_1: Point::new(0.56099963f32, 0.8299999f32),
    },
    Cubic {
        anchor_0: Point::new(0.56099963f32, 0.8299999f32),
        control_0: Point::new(0.5146664f32, 0.8299999f32),
        control_1: Point::new(0.46833318f32, 0.8299999f32),
        anchor_1: Point::new(0.42199996f32, 0.82999986f32),
    },
    Cubic {
        anchor_0: Point::new(0.42199996f32, 0.82999986f32),
        control_0: Point::new(0.422f32, 0.8576665f32),
        control_1: Point::new(0.42200002f32, 0.8853331f32),
        anchor_1: Point::new(0.42200005f32, 0.9129998f32),
    },
    Cubic {
        anchor_0: Point::new(0.42200005f32, 0.9129998f32),
        control_0: Point::new(0.37733334f32, 0.91299975f32),
        control_1: Point::new(0.33266667f32, 0.9129998f32),
        anchor_1: Point::new(0.288f32, 0.9129998f32),
    },
    Cubic {
        anchor_0: Point::new(0.288f32, 0.9129998f32),
        control_0: Point::new(0.2880001f32, 0.9419998f32),
        control_1: Point::new(0.28800026f32, 0.9709999f32),
        anchor_1: Point::new(0.28800038f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.28800038f32, 1.0f32),
        control_0: Point::new(0.2300003f32, 0.99999976f32),
        control_1: Point::new(0.17200024f32, 1.0f32),
        anchor_1: Point::new(0.1140002f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.1140002f32, 1.0f32),
        control_0: Point::new(0.113000154f32, 0.83333325f32),
        control_1: Point::new(0.11200012f32, 0.6666665f32),
        anchor_1: Point::new(0.11100009f32, 0.49999988f32),
    },
];
static PIXEL_TRIANGLE_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: PIXEL_TRIANGLE_CUBICS,
}];
pub const PIXEL_TRIANGLE: ShapeDataStored = ShapeDataStored {
    features: PIXEL_TRIANGLE_FEATURES,
};

static PUFFY_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.17030418f32),
        control_0: Point::new(0.5057874f32, 0.16142936f32),
        control_1: Point::new(0.51157486f32, 0.15255453f32),
        anchor_1: Point::new(0.5173623f32, 0.14367971f32),
    },
    Cubic {
        anchor_0: Point::new(0.5173623f32, 0.14367971f32),
        control_0: Point::new(0.5339834f32, 0.11819186f32),
        control_1: Point::new(0.5693888f32, 0.10244157f32),
        anchor_1: Point::new(0.6075185f32, 0.10357324f32),
    },
    Cubic {
        anchor_0: Point::new(0.6075185f32, 0.10357324f32),
        control_0: Point::new(0.6452641f32, 0.104693525f32),
        control_1: Point::new(0.67900246f32, 0.12136877f32),
        anchor_1: Point::new(0.69494015f32, 0.14678164f32),
    },
    Cubic {
        anchor_0: Point::new(0.69494015f32, 0.14678164f32),
        control_0: Point::new(0.69744104f32, 0.15076935f32),
        control_1: Point::new(0.69994193f32, 0.15475707f32),
        anchor_1: Point::new(0.7024428f32, 0.15874477f32),
    },
    Cubic {
        anchor_0: Point::new(0.7024428f32, 0.15874477f32),
        control_0: Point::new(0.7112147f32, 0.17273168f32),
        control_1: Point::new(0.71656144f32, 0.1877654f32),
        anchor_1: Point::new(0.71823275f32, 0.20314266f32),
    },
    Cubic {
        anchor_0: Point::new(0.71823275f32, 0.20314266f32),
        control_0: Point::new(0.7190471f32, 0.21063587f32),
        control_1: Point::new(0.7198615f32, 0.21812908f32),
        anchor_1: Point::new(0.72067595f32, 0.22562231f32),
    },
    Cubic {
        anchor_0: Point::new(0.72067595f32, 0.22562231f32),
        control_0: Point::new(0.7465048f32, 0.18171695f32),
        control_1: Point::new(0.824503f32, 0.17182331f32),
        anchor_1: Point::new(0.8685214f32, 0.20686898f32),
    },
    Cubic {
        anchor_0: Point::new(0.8685214f32, 0.20686898f32),
        control_0: Point::new(0.8695357f32, 0.20767654f32),
        control_1: Point::new(0.87055004f32, 0.20848411f32),
        anchor_1: Point::new(0.8715644f32, 0.20929165f32),
    },
    Cubic {
        anchor_0: Point::new(0.8715644f32, 0.20929165f32),
        control_0: Point::new(0.89745915f32, 0.22990802f32),
        control_1: Point::new(0.9132934f32, 0.2563698f32),
        anchor_1: Point::new(0.91653705f32, 0.28444856f32),
    },
    Cubic {
        anchor_0: Point::new(0.91653705f32, 0.28444856f32),
        control_0: Point::new(0.91678935f32, 0.2866334f32),
        control_1: Point::new(0.9170418f32, 0.28881827f32),
        anchor_1: Point::new(0.91729414f32, 0.29100308f32),
    },
    Cubic {
        anchor_0: Point::new(0.91729414f32, 0.29100308f32),
        control_0: Point::new(0.92125475f32, 0.32528818f32),
        control_1: Point::new(0.9074641f32, 0.35940507f32),
        anchor_1: Point::new(0.8787276f32, 0.3864135f32),
    },
    Cubic {
        anchor_0: Point::new(0.8787276f32, 0.3864135f32),
        control_0: Point::new(0.8807654f32, 0.38636225f32),
        control_1: Point::new(0.8828032f32, 0.38631102f32),
        anchor_1: Point::new(0.8848409f32, 0.38625976f32),
    },
    Cubic {
        anchor_0: Point::new(0.8848409f32, 0.38625976f32),
        control_0: Point::new(0.9175362f32, 0.3854374f32),
        control_1: Point::new(0.94850904f32, 0.39713305f32),
        anchor_1: Point::new(0.96685666f32, 0.41722983f32),
    },
    Cubic {
        anchor_0: Point::new(0.96685666f32, 0.41722983f32),
        control_0: Point::new(0.9884553f32, 0.44088754f32),
        control_1: Point::new(1.0f32, 0.46880633f32),
        anchor_1: Point::new(1.0f32, 0.49738124f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.49738124f32),
        control_0: Point::new(1.0f32, 0.4991271f32),
        control_1: Point::new(1.0f32, 0.5008729f32),
        anchor_1: Point::new(1.0f32, 0.5026187f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.5026187f32),
        control_0: Point::new(1.0f32, 0.5311936f32),
        control_1: Point::new(0.98845536f32, 0.5591125f32),
        anchor_1: Point::new(0.96685666f32, 0.58277017f32),
    },
    Cubic {
        anchor_0: Point::new(0.96685666f32, 0.58277017f32),
        control_0: Point::new(0.94850904f32, 0.6028669f32),
        control_1: Point::new(0.9175362f32, 0.6145627f32),
        anchor_1: Point::new(0.8848409f32, 0.61374027f32),
    },
    Cubic {
        anchor_0: Point::new(0.8848409f32, 0.61374027f32),
        control_0: Point::new(0.88280314f32, 0.61368906f32),
        control_1: Point::new(0.8807654f32, 0.61363775f32),
        anchor_1: Point::new(0.8787276f32, 0.6135865f32),
    },
    Cubic {
        anchor_0: Point::new(0.8787276f32, 0.6135865f32),
        control_0: Point::new(0.9074641f32, 0.6405949f32),
        control_1: Point::new(0.9212547f32, 0.67471176f32),
        anchor_1: Point::new(0.9172942f32, 0.70899683f32),
    },
    Cubic {
        anchor_0: Point::new(0.9172942f32, 0.70899683f32),
        control_0: Point::new(0.91704184f32, 0.7111817f32),
        control_1: Point::new(0.9167894f32, 0.7133665f32),
        anchor_1: Point::new(0.91653705f32, 0.7155514f32),
    },
    Cubic {
        anchor_0: Point::new(0.91653705f32, 0.7155514f32),
        control_0: Point::new(0.9132934f32, 0.7436302f32),
        control_1: Point::new(0.8974592f32, 0.77009183f32),
        anchor_1: Point::new(0.87156445f32, 0.79070824f32),
    },
    Cubic {
        anchor_0: Point::new(0.87156445f32, 0.79070824f32),
        control_0: Point::new(0.8705501f32, 0.7915158f32),
        control_1: Point::new(0.86953574f32, 0.79232347f32),
        anchor_1: Point::new(0.8685214f32, 0.793131f32),
    },
    Cubic {
        anchor_0: Point::new(0.8685214f32, 0.793131f32),
        control_0: Point::new(0.82450306f32, 0.8281766f32),
        control_1: Point::new(0.7465048f32, 0.818283f32),
        anchor_1: Point::new(0.72067595f32, 0.7743777f32),
    },
    Cubic {
        anchor_0: Point::new(0.72067595f32, 0.7743777f32),
        control_0: Point::new(0.7198615f32, 0.7818709f32),
        control_1: Point::new(0.7190472f32, 0.78936416f32),
        anchor_1: Point::new(0.71823275f32, 0.79685736f32),
    },
    Cubic {
        anchor_0: Point::new(0.71823275f32, 0.79685736f32),
        control_0: Point::new(0.71656144f32, 0.8122345f32),
        control_1: Point::new(0.7112147f32, 0.82726836f32),
        anchor_1: Point::new(0.7024429f32, 0.8412552f32),
    },
    Cubic {
        anchor_0: Point::new(0.7024429f32, 0.8412552f32),
        control_0: Point::new(0.69994193f32, 0.8452428f32),
        control_1: Point::new(0.6974411f32, 0.8492305f32),
        anchor_1: Point::new(0.6949402f32, 0.8532183f32),
    },
    Cubic {
        anchor_0: Point::new(0.6949402f32, 0.8532183f32),
        control_0: Point::new(0.67900246f32, 0.8786311f32),
        control_1: Point::new(0.645264f32, 0.89530647f32),
        anchor_1: Point::new(0.6075184f32, 0.8964267f32),
    },
    Cubic {
        anchor_0: Point::new(0.6075184f32, 0.8964267f32),
        control_0: Point::new(0.56938875f32, 0.8975583f32),
        control_1: Point::new(0.5339833f32, 0.8818081f32),
        anchor_1: Point::new(0.5173622f32, 0.8563202f32),
    },
    Cubic {
        anchor_0: Point::new(0.5173622f32, 0.8563202f32),
        control_0: Point::new(0.5115748f32, 0.8474454f32),
        control_1: Point::new(0.5057873f32, 0.8385706f32),
        anchor_1: Point::new(0.5f32, 0.8296958f32),
    },
    Cubic {
        anchor_0: Point::new(0.5f32, 0.8296958f32),
        control_0: Point::new(0.49421254f32, 0.8385706f32),
        control_1: Point::new(0.48842508f32, 0.8474455f32),
        anchor_1: Point::new(0.4826377f32, 0.85632026f32),
    },
    Cubic {
        anchor_0: Point::new(0.4826377f32, 0.85632026f32),
        control_0: Point::new(0.46601665f32, 0.8818081f32),
        control_1: Point::new(0.43061125f32, 0.89755845f32),
        anchor_1: Point::new(0.39248157f32, 0.8964267f32),
    },
    Cubic {
        anchor_0: Point::new(0.39248157f32, 0.8964267f32),
        control_0: Point::new(0.3547359f32, 0.89530647f32),
        control_1: Point::new(0.32099742f32, 0.8786311f32),
        anchor_1: Point::new(0.30505976f32, 0.85321826f32),
    },
    Cubic {
        anchor_0: Point::new(0.30505976f32, 0.85321826f32),
        control_0: Point::new(0.30255884f32, 0.8492305f32),
        control_1: Point::new(0.30005795f32, 0.84524274f32),
        anchor_1: Point::new(0.29755703f32, 0.84125507f32),
    },
    Cubic {
        anchor_0: Point::new(0.29755703f32, 0.84125507f32),
        control_0: Point::new(0.28878525f32, 0.82726836f32),
        control_1: Point::new(0.28343862f32, 0.8122346f32),
        anchor_1: Point::new(0.2817673f32, 0.79685754f32),
    },
    Cubic {
        anchor_0: Point::new(0.2817673f32, 0.79685754f32),
        control_0: Point::new(0.28095287f32, 0.78936416f32),
        control_1: Point::new(0.28013843f32, 0.781871f32),
        anchor_1: Point::new(0.279324f32, 0.7743777f32),
    },
    Cubic {
        anchor_0: Point::new(0.279324f32, 0.7743777f32),
        control_0: Point::new(0.25349516f32, 0.818283f32),
        control_1: Point::new(0.17549694f32, 0.8281766f32),
        anchor_1: Point::new(0.13147861f32, 0.793131f32),
    },
    Cubic {
        anchor_0: Point::new(0.13147861f32, 0.793131f32),
        control_0: Point::new(0.13046427f32, 0.79232347f32),
        control_1: Point::new(0.12944992f32, 0.7915158f32),
        anchor_1: Point::new(0.12843557f32, 0.79070824f32),
    },
    Cubic {
        anchor_0: Point::new(0.12843557f32, 0.79070824f32),
        control_0: Point::new(0.10254084f32, 0.7700919f32),
        control_1: Point::new(0.08670654f32, 0.7436301f32),
        anchor_1: Point::new(0.08346296f32, 0.7155514f32),
    },
    Cubic {
        anchor_0: Point::new(0.08346296f32, 0.7155514f32),
        control_0: Point::new(0.08321057f32, 0.7133665f32),
        control_1: Point::new(0.082958184f32, 0.7111817f32),
        anchor_1: Point::new(0.082705826f32, 0.7089969f32),
    },
    Cubic {
        anchor_0: Point::new(0.082705826f32, 0.7089969f32),
        control_0: Point::new(0.07874528f32, 0.6747118f32),
        control_1: Point::new(0.09253596f32, 0.64059484f32),
        anchor_1: Point::new(0.121272385f32, 0.6135864f32),
    },
    Cubic {
        anchor_0: Point::new(0.121272385f32, 0.6135864f32),
        control_0: Point::new(0.119234614f32, 0.6136377f32),
        control_1: Point::new(0.11719684f32, 0.613689f32),
        anchor_1: Point::new(0.11515907f32, 0.6137402f32),
    },
    Cubic {
        anchor_0: Point::new(0.11515907f32, 0.6137402f32),
        control_0: Point::new(0.08246378f32, 0.61456263f32),
        control_1: Point::new(0.051490936f32, 0.6028669f32),
        anchor_1: Point::new(0.033143323f32, 0.5827701f32),
    },
    Cubic {
        anchor_0: Point::new(0.033143323f32, 0.5827701f32),
        control_0: Point::new(0.011544665f32, 0.5591124f32),
        control_1: Point::new(0.0f32, 0.5311936f32),
        anchor_1: Point::new(1.48122865E-8f32, 0.5026187f32),
    },
    Cubic {
        anchor_0: Point::new(1.48122865E-8f32, 0.5026187f32),
        control_0: Point::new(1.48122865E-8f32, 0.50087285f32),
        control_1: Point::new(1.48122865E-8f32, 0.4991271f32),
        anchor_1: Point::new(2.9624573E-8f32, 0.49738124f32),
    },
    Cubic {
        anchor_0: Point::new(2.9624573E-8f32, 0.49738124f32),
        control_0: Point::new(2.9624573E-8f32, 0.46880633f32),
        control_1: Point::new(0.011544737f32, 0.44088748f32),
        anchor_1: Point::new(0.033143386f32, 0.41722977f32),
    },
    Cubic {
        anchor_0: Point::new(0.033143386f32, 0.41722977f32),
        control_0: Point::new(0.051490985f32, 0.39713305f32),
        control_1: Point::new(0.082463704f32, 0.38543734f32),
        anchor_1: Point::new(0.115158916f32, 0.38625965f32),
    },
    Cubic {
        anchor_0: Point::new(0.115158916f32, 0.38625965f32),
        control_0: Point::new(0.11719677f32, 0.3863109f32),
        control_1: Point::new(0.11923462f32, 0.38636214f32),
        anchor_1: Point::new(0.121272475f32, 0.3864134f32),
    },
    Cubic {
        anchor_0: Point::new(0.121272475f32, 0.3864134f32),
        control_0: Point::new(0.092536f32, 0.35940498f32),
        control_1: Point::new(0.07874535f32, 0.32528806f32),
        anchor_1: Point::new(0.08270587f32, 0.29100302f32),
    },
    Cubic {
        anchor_0: Point::new(0.08270587f32, 0.29100302f32),
        control_0: Point::new(0.08295825f32, 0.2888182f32),
        control_1: Point::new(0.08321063f32, 0.2866334f32),
        anchor_1: Point::new(0.08346303f32, 0.2844486f32),
    },
    Cubic {
        anchor_0: Point::new(0.08346303f32, 0.2844486f32),
        control_0: Point::new(0.08670662f32, 0.2563698f32),
        control_1: Point::new(0.10254086f32, 0.22990805f32),
        anchor_1: Point::new(0.12843566f32, 0.20929164f32),
    },
    Cubic {
        anchor_0: Point::new(0.12843566f32, 0.20929164f32),
        control_0: Point::new(0.12944995f32, 0.2084841f32),
        control_1: Point::new(0.13046427f32, 0.20767653f32),
        anchor_1: Point::new(0.13147858f32, 0.20686899f32),
    },
    Cubic {
        anchor_0: Point::new(0.13147858f32, 0.20686899f32),
        control_0: Point::new(0.17549694f32, 0.17182332f32),
        control_1: Point::new(0.25349522f32, 0.1817169f32),
        anchor_1: Point::new(0.27932417f32, 0.22562224f32),
    },
    Cubic {
        anchor_0: Point::new(0.27932417f32, 0.22562224f32),
        control_0: Point::new(0.2801386f32, 0.21812902f32),
        control_1: Point::new(0.280953f32, 0.21063581f32),
        anchor_1: Point::new(0.28176743f32, 0.2031426f32),
    },
    Cubic {
        anchor_0: Point::new(0.28176743f32, 0.2031426f32),
        control_0: Point::new(0.28343874f32, 0.18776543f32),
        control_1: Point::new(0.28878543f32, 0.1727316f32),
        anchor_1: Point::new(0.29755726f32, 0.15874474f32),
    },
    Cubic {
        anchor_0: Point::new(0.29755726f32, 0.15874474f32),
        control_0: Point::new(0.3000582f32, 0.15475702f32),
        control_1: Point::new(0.30255908f32, 0.1507693f32),
        anchor_1: Point::new(0.30506f32, 0.14678156f32),
    },
    Cubic {
        anchor_0: Point::new(0.30506f32, 0.14678156f32),
        control_0: Point::new(0.32099766f32, 0.12136875f32),
        control_1: Point::new(0.35473603f32, 0.104693495f32),
        anchor_1: Point::new(0.39248157f32, 0.10357322f32),
    },
    Cubic {
        anchor_0: Point::new(0.39248157f32, 0.10357322f32),
        control_0: Point::new(0.4306112f32, 0.10244155f32),
        control_1: Point::new(0.46601665f32, 0.11819186f32),
        anchor_1: Point::new(0.4826377f32, 0.1436797f32),
    },
    Cubic {
        anchor_0: Point::new(0.4826377f32, 0.1436797f32),
        control_0: Point::new(0.48842508f32, 0.15255451f32),
        control_1: Point::new(0.49421254f32, 0.16142936f32),
        anchor_1: Point::new(0.5f32, 0.17030418f32),
    },
];
static PUFFY_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: PUFFY_CUBICS,
}];
pub const PUFFY: ShapeDataStored = ShapeDataStored {
    features: PUFFY_FEATURES,
};

static PUFFY_DIAMOND_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.77894926f32, 0.22105066f32),
        control_0: Point::new(0.8125624f32, 0.25466374f32),
        control_1: Point::new(0.8300485f32, 0.3044038f32),
        anchor_1: Point::new(0.81802547f32, 0.3568888f32),
    },
    Cubic {
        anchor_0: Point::new(0.81802547f32, 0.3568888f32),
        control_0: Point::new(0.81801695f32, 0.35692582f32),
        control_1: Point::new(0.81800854f32, 0.3569629f32),
        anchor_1: Point::new(0.818f32, 0.35699996f32),
    },
    Cubic {
        anchor_0: Point::new(0.818f32, 0.35699996f32),
        control_0: Point::new(0.8231876f32, 0.35628733f32),
        control_1: Point::new(0.8283752f32, 0.35557476f32),
        anchor_1: Point::new(0.83356285f32, 0.35486224f32),
    },
    Cubic {
        anchor_0: Point::new(0.83356285f32, 0.35486224f32),
        control_0: Point::new(0.92157155f32, 0.3427731f32),
        control_1: Point::new(1.0f32, 0.41116485f32),
        anchor_1: Point::new(1.0f32, 0.49999997f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.49999997f32),
        control_0: Point::new(1.0f32, 0.5888351f32),
        control_1: Point::new(0.9215715f32, 0.65722686f32),
        anchor_1: Point::new(0.83356285f32, 0.6451378f32),
    },
    Cubic {
        anchor_0: Point::new(0.83356285f32, 0.6451378f32),
        control_0: Point::new(0.8283752f32, 0.64442515f32),
        control_1: Point::new(0.82318765f32, 0.6437126f32),
        anchor_1: Point::new(0.818f32, 0.643f32),
    },
    Cubic {
        anchor_0: Point::new(0.818f32, 0.643f32),
        control_0: Point::new(0.8180085f32, 0.6430371f32),
        control_1: Point::new(0.818017f32, 0.64307415f32),
        anchor_1: Point::new(0.8180255f32, 0.6431112f32),
    },
    Cubic {
        anchor_0: Point::new(0.8180255f32, 0.6431112f32),
        control_0: Point::new(0.84207153f32, 0.7480812f32),
        control_1: Point::new(0.7480812f32, 0.84207153f32),
        anchor_1: Point::new(0.6431112f32, 0.81802547f32),
    },
    Cubic {
        anchor_0: Point::new(0.6431112f32, 0.81802547f32),
        control_0: Point::new(0.64307415f32, 0.81801695f32),
        control_1: Point::new(0.6430371f32, 0.81800854f32),
        anchor_1: Point::new(0.643f32, 0.818f32),
    },
    Cubic {
        anchor_0: Point::new(0.643f32, 0.818f32),
        control_0: Point::new(0.6437126f32, 0.8231876f32),
        control_1: Point::new(0.64442515f32, 0.8283752f32),
        anchor_1: Point::new(0.6451377f32, 0.83356285f32),
    },
    Cubic {
        anchor_0: Point::new(0.6451377f32, 0.83356285f32),
        control_0: Point::new(0.65722686f32, 0.92157155f32),
        control_1: Point::new(0.5888351f32, 1.0f32),
        anchor_1: Point::new(0.49999997f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.49999997f32, 1.0f32),
        control_0: Point::new(0.41116488f32, 1.0f32),
        control_1: Point::new(0.3427731f32, 0.9215715f32),
        anchor_1: Point::new(0.3548622f32, 0.83356285f32),
    },
    Cubic {
        anchor_0: Point::new(0.3548622f32, 0.83356285f32),
        control_0: Point::new(0.3555748f32, 0.8283752f32),
        control_1: Point::new(0.3562874f32, 0.82318765f32),
        anchor_1: Point::new(0.35699996f32, 0.818f32),
    },
    Cubic {
        anchor_0: Point::new(0.35699996f32, 0.818f32),
        control_0: Point::new(0.35696286f32, 0.81800854f32),
        control_1: Point::new(0.35692576f32, 0.818017f32),
        anchor_1: Point::new(0.35688868f32, 0.8180255f32),
    },
    Cubic {
        anchor_0: Point::new(0.35688868f32, 0.8180255f32),
        control_0: Point::new(0.25191876f32, 0.84207153f32),
        control_1: Point::new(0.15792844f32, 0.7480812f32),
        anchor_1: Point::new(0.18197438f32, 0.6431113f32),
    },
    Cubic {
        anchor_0: Point::new(0.18197438f32, 0.6431113f32),
        control_0: Point::new(0.18198289f32, 0.64307415f32),
        control_1: Point::new(0.1819914f32, 0.643037f32),
        anchor_1: Point::new(0.1819999f32, 0.6429999f32),
    },
    Cubic {
        anchor_0: Point::new(0.1819999f32, 0.6429999f32),
        control_0: Point::new(0.17681234f32, 0.6437124f32),
        control_1: Point::new(0.1716248f32, 0.64442503f32),
        anchor_1: Point::new(0.16643724f32, 0.6451376f32),
    },
    Cubic {
        anchor_0: Point::new(0.16643724f32, 0.6451376f32),
        control_0: Point::new(0.07842854f32, 0.65722674f32),
        control_1: Point::new(0.0f32, 0.58883494f32),
        anchor_1: Point::new(3.170385E-9f32, 0.4999998f32),
    },
    Cubic {
        anchor_0: Point::new(3.170385E-9f32, 0.4999998f32),
        control_0: Point::new(3.931346E-8f32, 0.41116473f32),
        control_1: Point::new(0.078428514f32, 0.342773f32),
        anchor_1: Point::new(0.16643716f32, 0.35486206f32),
    },
    Cubic {
        anchor_0: Point::new(0.16643716f32, 0.35486206f32),
        control_0: Point::new(0.17162476f32, 0.35557464f32),
        control_1: Point::new(0.17681237f32, 0.35628727f32),
        anchor_1: Point::new(0.18199997f32, 0.35699984f32),
    },
    Cubic {
        anchor_0: Point::new(0.18199997f32, 0.35699984f32),
        control_0: Point::new(0.18199147f32, 0.35696277f32),
        control_1: Point::new(0.18198298f32, 0.3569257f32),
        anchor_1: Point::new(0.1819745f32, 0.35688868f32),
    },
    Cubic {
        anchor_0: Point::new(0.1819745f32, 0.35688868f32),
        control_0: Point::new(0.15792856f32, 0.25191876f32),
        control_1: Point::new(0.25191888f32, 0.15792844f32),
        anchor_1: Point::new(0.3568888f32, 0.18197441f32),
    },
    Cubic {
        anchor_0: Point::new(0.3568888f32, 0.18197441f32),
        control_0: Point::new(0.35692587f32, 0.1819829f32),
        control_1: Point::new(0.35696295f32, 0.1819914f32),
        anchor_1: Point::new(0.35700002f32, 0.18199989f32),
    },
    Cubic {
        anchor_0: Point::new(0.35700002f32, 0.18199989f32),
        control_0: Point::new(0.35628745f32, 0.17681232f32),
        control_1: Point::new(0.35557488f32, 0.17162476f32),
        anchor_1: Point::new(0.3548623f32, 0.16643721f32),
    },
    Cubic {
        anchor_0: Point::new(0.3548623f32, 0.16643721f32),
        control_0: Point::new(0.34277317f32, 0.0784285f32),
        control_1: Point::new(0.41116497f32, 0.0f32),
        anchor_1: Point::new(0.5000001f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.5000001f32, 0.0f32),
        control_0: Point::new(0.58883524f32, 0.0f32),
        control_1: Point::new(0.6572271f32, 0.078428544f32),
        anchor_1: Point::new(0.64513785f32, 0.16643727f32),
    },
    Cubic {
        anchor_0: Point::new(0.64513785f32, 0.16643727f32),
        control_0: Point::new(0.6444253f32, 0.17162482f32),
        control_1: Point::new(0.6437127f32, 0.1768124f32),
        anchor_1: Point::new(0.6430001f32, 0.18199995f32),
    },
    Cubic {
        anchor_0: Point::new(0.6430001f32, 0.18199995f32),
        control_0: Point::new(0.6430371f32, 0.18199146f32),
        control_1: Point::new(0.64307415f32, 0.181983f32),
        anchor_1: Point::new(0.64311117f32, 0.18197453f32),
    },
    Cubic {
        anchor_0: Point::new(0.64311117f32, 0.18197453f32),
        control_0: Point::new(0.6955961f32, 0.16995151f32),
        control_1: Point::new(0.74533623f32, 0.1874376f32),
        anchor_1: Point::new(0.77894926f32, 0.22105066f32),
    },
];
static PUFFY_DIAMOND_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: PUFFY_DIAMOND_CUBICS,
}];
pub const PUFFY_DIAMOND: ShapeDataStored = ShapeDataStored {
    features: PUFFY_DIAMOND_FEATURES,
};

static SEMI_CIRCLE_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.96949035f32, 0.7819903f32),
        control_0: Point::new(0.95063984f32, 0.80084074f32),
        control_1: Point::new(0.9245982f32, 0.81249994f32),
        anchor_1: Point::new(0.8958334f32, 0.81249994f32),
    },
    Cubic {
        anchor_0: Point::new(0.8958334f32, 0.81249994f32),
        control_0: Point::new(0.6319445f32, 0.81249994f32),
        control_1: Point::new(0.36805555f32, 0.81249994f32),
        anchor_1: Point::new(0.10416668f32, 0.81249994f32),
    },
    Cubic {
        anchor_0: Point::new(0.10416668f32, 0.81249994f32),
        control_0: Point::new(0.04663702f32, 0.81249994f32),
        control_1: Point::new(0.0f32, 0.765863f32),
        anchor_1: Point::new(0.0f32, 0.7083333f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.7083333f32),
        control_0: Point::new(3.7252903E-8f32, 0.70138884f32),
        control_1: Point::new(0.0f32, 0.6944445f32),
        anchor_1: Point::new(0.0f32, 0.6875f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.6875f32),
        control_0: Point::new(0.0f32, 0.41135764f32),
        control_1: Point::new(0.22385763f32, 0.1875f32),
        anchor_1: Point::new(0.5f32, 0.1875f32),
    },
    Cubic {
        anchor_0: Point::new(0.5f32, 0.1875f32),
        control_0: Point::new(0.7761424f32, 0.1875f32),
        control_1: Point::new(1.0f32, 0.41135764f32),
        anchor_1: Point::new(1.0f32, 0.6875f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.6875f32),
        control_0: Point::new(0.99999994f32, 0.6944445f32),
        control_1: Point::new(1.0f32, 0.70138884f32),
        anchor_1: Point::new(1.0f32, 0.7083333f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.7083333f32),
        control_0: Point::new(1.0f32, 0.73709816f32),
        control_1: Point::new(0.98834074f32, 0.76313984f32),
        anchor_1: Point::new(0.96949035f32, 0.7819903f32),
    },
];
static SEMI_CIRCLE_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: SEMI_CIRCLE_CUBICS,
}];
pub const SEMI_CIRCLE: ShapeDataStored = ShapeDataStored {
    features: SEMI_CIRCLE_FEATURES,
};

static SLANTED_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.8755312f32, 0.9140243f32),
        control_0: Point::new(0.8677265f32, 0.92105526f32),
        control_1: Point::new(0.859326f32, 0.9274248f32),
        anchor_1: Point::new(0.85041404f32, 0.93305683f32),
    },
    Cubic {
        anchor_0: Point::new(0.85041404f32, 0.93305683f32),
        control_0: Point::new(0.80464435f32, 0.9619815f32),
        control_1: Point::new(0.74102956f32, 0.96177995f32),
        anchor_1: Point::new(0.61379975f32, 0.9613769f32),
    },
    Cubic {
        anchor_0: Point::new(0.61379975f32, 0.9613769f32),
        control_0: Point::new(0.47651702f32, 0.96094203f32),
        control_1: Point::new(0.3392343f32, 0.9605071f32),
        anchor_1: Point::new(0.2019516f32, 0.9600722f32),
    },
    Cubic {
        anchor_0: Point::new(0.2019516f32, 0.9600722f32),
        control_0: Point::new(0.19244726f32, 0.96004206f32),
        control_1: Point::new(0.1876951f32, 0.96002704f32),
        anchor_1: Point::new(0.1850028f32, 0.95993966f32),
    },
    Cubic {
        anchor_0: Point::new(0.1850028f32, 0.95993966f32),
        control_0: Point::new(0.08062565f32, 0.95655257f32),
        control_1: Point::new(0.0f32, 0.86705464f32),
        anchor_1: Point::new(0.0074887364f32, 0.7628914f32),
    },
    Cubic {
        anchor_0: Point::new(0.0074887364f32, 0.7628914f32),
        control_0: Point::new(0.0076818247f32, 0.76020575f32),
        control_1: Point::new(0.008161218f32, 0.7554774f32),
        anchor_1: Point::new(0.009120003f32, 0.7460208f32),
    },
    Cubic {
        anchor_0: Point::new(0.009120003f32, 0.7460208f32),
        control_0: Point::new(0.022795796f32, 0.61113435f32),
        control_1: Point::new(0.036471594f32, 0.47624788f32),
        anchor_1: Point::new(0.050147384f32, 0.3413614f32),
    },
    Cubic {
        anchor_0: Point::new(0.050147384f32, 0.3413614f32),
        control_0: Point::new(0.062981166f32, 0.21478f32),
        control_1: Point::new(0.069398046f32, 0.15148929f32),
        anchor_1: Point::new(0.10292682f32, 0.10897672f32),
    },
    Cubic {
        anchor_0: Point::new(0.10292682f32, 0.10897672f32),
        control_0: Point::new(0.11598381f32, 0.09242122f32),
        control_1: Point::new(0.13176204f32, 0.07820718f32),
        anchor_1: Point::new(0.14958593f32, 0.066943154f32),
    },
    Cubic {
        anchor_0: Point::new(0.14958593f32, 0.066943154f32),
        control_0: Point::new(0.19535553f32, 0.038018484f32),
        control_1: Point::new(0.25897038f32, 0.038220014f32),
        anchor_1: Point::new(0.38620013f32, 0.038623083f32),
    },
    Cubic {
        anchor_0: Point::new(0.38620013f32, 0.038623083f32),
        control_0: Point::new(0.52348286f32, 0.039057992f32),
        control_1: Point::new(0.66076565f32, 0.03949291f32),
        anchor_1: Point::new(0.79804844f32, 0.03992782f32),
    },
    Cubic {
        anchor_0: Point::new(0.79804844f32, 0.03992782f32),
        control_0: Point::new(0.8075533f32, 0.039957937f32),
        control_1: Point::new(0.81230575f32, 0.039972994f32),
        anchor_1: Point::new(0.8149972f32, 0.040060326f32),
    },
    Cubic {
        anchor_0: Point::new(0.8149972f32, 0.040060326f32),
        control_0: Point::new(0.91937435f32, 0.043447427f32),
        control_1: Point::new(1.0f32, 0.13294539f32),
        anchor_1: Point::new(0.9925112f32, 0.23710865f32),
    },
    Cubic {
        anchor_0: Point::new(0.9925112f32, 0.23710865f32),
        control_0: Point::new(0.9923179f32, 0.23979653f32),
        control_1: Point::new(0.99183863f32, 0.2445241f32),
        anchor_1: Point::new(0.99088f32, 0.2539793f32),
    },
    Cubic {
        anchor_0: Point::new(0.99088f32, 0.2539793f32),
        control_0: Point::new(0.97720414f32, 0.3888657f32),
        control_1: Point::new(0.96352834f32, 0.52375215f32),
        anchor_1: Point::new(0.9498526f32, 0.65863854f32),
    },
    Cubic {
        anchor_0: Point::new(0.9498526f32, 0.65863854f32),
        control_0: Point::new(0.93701875f32, 0.78521997f32),
        control_1: Point::new(0.9306019f32, 0.8485106f32),
        anchor_1: Point::new(0.89707315f32, 0.8910232f32),
    },
    Cubic {
        anchor_0: Point::new(0.89707315f32, 0.8910232f32),
        control_0: Point::new(0.89054465f32, 0.89930093f32),
        control_1: Point::new(0.8833358f32, 0.9069933f32),
        anchor_1: Point::new(0.8755312f32, 0.9140243f32),
    },
];
static SLANTED_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: SLANTED_CUBICS,
}];
pub const SLANTED: ShapeDataStored = ShapeDataStored {
    features: SLANTED_FEATURES,
};

static SOFT_BOOM_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.7339415f32, 0.45381412f32),
        control_0: Point::new(0.7536878f32, 0.45064723f32),
        control_1: Point::new(0.7734341f32, 0.44748044f32),
        anchor_1: Point::new(0.79318035f32, 0.4443136f32),
    },
    Cubic {
        anchor_0: Point::new(0.79318035f32, 0.4443136f32),
        control_0: Point::new(0.82449913f32, 0.43929076f32),
        control_1: Point::new(0.8563485f32, 0.43848857f32),
        anchor_1: Point::new(0.88788044f32, 0.44192842f32),
    },
    Cubic {
        anchor_0: Point::new(0.88788044f32, 0.44192842f32),
        control_0: Point::new(0.8996054f32, 0.4432075f32),
        control_1: Point::new(0.91133034f32, 0.4444866f32),
        anchor_1: Point::new(0.9230553f32, 0.4457656f32),
    },
    Cubic {
        anchor_0: Point::new(0.9230553f32, 0.4457656f32),
        control_0: Point::new(0.94135004f32, 0.4477614f32),
        control_1: Point::new(0.9590074f32, 0.45364285f32),
        anchor_1: Point::new(0.9748449f32, 0.46301606f32),
    },
    Cubic {
        anchor_0: Point::new(0.9748449f32, 0.46301606f32),
        control_0: Point::new(0.9766924f32, 0.46410945f32),
        control_1: Point::new(0.9785399f32, 0.4652029f32),
        anchor_1: Point::new(0.9803874f32, 0.46629632f32),
    },
    Cubic {
        anchor_0: Point::new(0.9803874f32, 0.46629632f32),
        control_0: Point::new(0.9925489f32, 0.47349393f32),
        control_1: Point::new(1.0f32, 0.48658374f32),
        anchor_1: Point::new(0.9999798f32, 0.50071555f32),
    },
    Cubic {
        anchor_0: Point::new(0.9999798f32, 0.50071555f32),
        control_0: Point::new(0.9999595f32, 0.5148474f32),
        control_1: Point::new(0.99247104f32, 0.52791584f32),
        anchor_1: Point::new(0.9802889f32, 0.5350787f32),
    },
    Cubic {
        anchor_0: Point::new(0.9802889f32, 0.5350787f32),
        control_0: Point::new(0.97843826f32, 0.5361668f32),
        control_1: Point::new(0.97658765f32, 0.5372549f32),
        anchor_1: Point::new(0.97473705f32, 0.538343f32),
    },
    Cubic {
        anchor_0: Point::new(0.97473705f32, 0.538343f32),
        control_0: Point::new(0.9588728f32, 0.54767084f32),
        control_1: Point::new(0.9411988f32, 0.5535017f32),
        anchor_1: Point::new(0.92289835f32, 0.5554451f32),
    },
    Cubic {
        anchor_0: Point::new(0.92289835f32, 0.5554451f32),
        control_0: Point::new(0.91116977f32, 0.5566906f32),
        control_1: Point::new(0.89944124f32, 0.55793613f32),
        anchor_1: Point::new(0.88771266f32, 0.55918163f32),
    },
    Cubic {
        anchor_0: Point::new(0.88771266f32, 0.55918163f32),
        control_0: Point::new(0.856171f32, 0.5625312f32),
        control_1: Point::new(0.82432413f32, 0.5616379f32),
        anchor_1: Point::new(0.79301983f32, 0.55652547f32),
    },
    Cubic {
        anchor_0: Point::new(0.79301983f32, 0.55652547f32),
        control_0: Point::new(0.7732827f32, 0.55330205f32),
        control_1: Point::new(0.7535455f32, 0.5500787f32),
        anchor_1: Point::new(0.73380834f32, 0.54685533f32),
    },
    Cubic {
        anchor_0: Point::new(0.73380834f32, 0.54685533f32),
        control_0: Point::new(0.7532634f32, 0.551486f32),
        control_1: Point::new(0.7727185f32, 0.5561169f32),
        anchor_1: Point::new(0.7921736f32, 0.56074774f32),
    },
    Cubic {
        anchor_0: Point::new(0.7921736f32, 0.56074774f32),
        control_0: Point::new(0.8230306f32, 0.56809247f32),
        control_1: Point::new(0.8527626f32, 0.5795396f32),
        anchor_1: Point::new(0.8805779f32, 0.5947843f32),
    },
    Cubic {
        anchor_0: Point::new(0.8805779f32, 0.5947843f32),
        control_0: Point::new(0.8909209f32, 0.60045296f32),
        control_1: Point::new(0.90126383f32, 0.60612166f32),
        anchor_1: Point::new(0.91160685f32, 0.6117903f32),
    },
    Cubic {
        anchor_0: Point::new(0.91160685f32, 0.6117903f32),
        control_0: Point::new(0.9277453f32, 0.6206354f32),
        control_1: Point::new(0.9418076f32, 0.6328261f32),
        anchor_1: Point::new(0.95285267f32, 0.6475466f32),
    },
    Cubic {
        anchor_0: Point::new(0.95285267f32, 0.6475466f32),
        control_0: Point::new(0.9541411f32, 0.64926374f32),
        control_1: Point::new(0.95542955f32, 0.65098095f32),
        anchor_1: Point::new(0.95671797f32, 0.6526981f32),
    },
    Cubic {
        anchor_0: Point::new(0.95671797f32, 0.6526981f32),
        control_0: Point::new(0.9651993f32, 0.6640019f32),
        control_1: Point::new(0.9670739f32, 0.67894673f32),
        anchor_1: Point::new(0.9616472f32, 0.6919951f32),
    },
    Cubic {
        anchor_0: Point::new(0.9616472f32, 0.6919951f32),
        control_0: Point::new(0.95622057f32, 0.70504344f32),
        control_1: Point::new(0.94430095f32, 0.7142514f32),
        anchor_1: Point::new(0.9303051f32, 0.7162071f32),
    },
    Cubic {
        anchor_0: Point::new(0.9303051f32, 0.7162071f32),
        control_0: Point::new(0.9281789f32, 0.71650416f32),
        control_1: Point::new(0.9260528f32, 0.7168013f32),
        anchor_1: Point::new(0.9239266f32, 0.7170983f32),
    },
    Cubic {
        anchor_0: Point::new(0.9239266f32, 0.7170983f32),
        control_0: Point::new(0.90570045f32, 0.7196451f32),
        control_1: Point::new(0.88714015f32, 0.71826845f32),
        anchor_1: Point::new(0.86948913f32, 0.71306074f32),
    },
    Cubic {
        anchor_0: Point::new(0.86948913f32, 0.71306074f32),
        control_0: Point::new(0.8581767f32, 0.7097231f32),
        control_1: Point::new(0.84686434f32, 0.70638555f32),
        anchor_1: Point::new(0.8355519f32, 0.7030479f32),
    },
    Cubic {
        anchor_0: Point::new(0.8355519f32, 0.7030479f32),
        control_0: Point::new(0.8051294f32, 0.694072f32),
        control_1: Point::new(0.7760485f32, 0.68105936f32),
        anchor_1: Point::new(0.7490836f32, 0.6643565f32),
    },
    Cubic {
        anchor_0: Point::new(0.7490836f32, 0.6643565f32),
        control_0: Point::new(0.73208237f32, 0.65382546f32),
        control_1: Point::new(0.71508116f32, 0.6432944f32),
        anchor_1: Point::new(0.69808f32, 0.63276327f32),
    },
    Cubic {
        anchor_0: Point::new(0.69808f32, 0.63276327f32),
        control_0: Point::new(0.714282f32, 0.64448667f32),
        control_1: Point::new(0.73048407f32, 0.6562102f32),
        anchor_1: Point::new(0.7466861f32, 0.6679335f32),
    },
    Cubic {
        anchor_0: Point::new(0.7466861f32, 0.6679335f32),
        control_0: Point::new(0.77238345f32, 0.6865276f32),
        control_1: Point::new(0.79547167f32, 0.7084814f32),
        anchor_1: Point::new(0.81533575f32, 0.73321015f32),
    },
    Cubic {
        anchor_0: Point::new(0.81533575f32, 0.73321015f32),
        control_0: Point::new(0.8227221f32, 0.74240535f32),
        control_1: Point::new(0.8301084f32, 0.7516006f32),
        anchor_1: Point::new(0.83749473f32, 0.7607958f32),
    },
    Cubic {
        anchor_0: Point::new(0.83749473f32, 0.7607958f32),
        control_0: Point::new(0.8490199f32, 0.7751433f32),
        control_1: Point::new(0.8573468f32, 0.7917879f32),
        anchor_1: Point::new(0.8619176f32, 0.8096145f32),
    },
    Cubic {
        anchor_0: Point::new(0.8619176f32, 0.8096145f32),
        control_0: Point::new(0.8624508f32, 0.81169397f32),
        control_1: Point::new(0.862984f32, 0.8137736f32),
        anchor_1: Point::new(0.8635173f32, 0.81585306f32),
    },
    Cubic {
        anchor_0: Point::new(0.8635173f32, 0.81585306f32),
        control_0: Point::new(0.8670273f32, 0.82954204f32),
        control_1: Point::new(0.8630401f32, 0.84406674f32),
        anchor_1: Point::new(0.8530331f32, 0.85404515f32),
    },
    Cubic {
        anchor_0: Point::new(0.8530331f32, 0.85404515f32),
        control_0: Point::new(0.8430261f32, 0.86402357f32),
        control_1: Point::new(0.8284901f32, 0.86796916f32),
        anchor_1: Point::new(0.8148112f32, 0.86441994f32),
    },
    Cubic {
        anchor_0: Point::new(0.8148112f32, 0.86441994f32),
        control_0: Point::new(0.8127332f32, 0.86388075f32),
        control_1: Point::new(0.81065524f32, 0.8633417f32),
        anchor_1: Point::new(0.80857724f32, 0.86280245f32),
    },
    Cubic {
        anchor_0: Point::new(0.80857724f32, 0.86280245f32),
        control_0: Point::new(0.79076374f32, 0.8581805f32),
        control_1: Point::new(0.7741432f32, 0.849806f32),
        anchor_1: Point::new(0.7598286f32, 0.83823985f32),
    },
    Cubic {
        anchor_0: Point::new(0.7598286f32, 0.83823985f32),
        control_0: Point::new(0.7506545f32, 0.8308271f32),
        control_1: Point::new(0.7414804f32, 0.82341456f32),
        anchor_1: Point::new(0.73230636f32, 0.81600183f32),
    },
    Cubic {
        anchor_0: Point::new(0.73230636f32, 0.81600183f32),
        control_0: Point::new(0.7076352f32, 0.79606754f32),
        control_1: Point::new(0.68574685f32, 0.7729157f32),
        anchor_1: Point::new(0.66722685f32, 0.7471659f32),
    },
    Cubic {
        anchor_0: Point::new(0.66722685f32, 0.7471659f32),
        control_0: Point::new(0.6555498f32, 0.7309304f32),
        control_1: Point::new(0.6438728f32, 0.7146948f32),
        anchor_1: Point::new(0.6321957f32, 0.6984592f32),
    },
    Cubic {
        anchor_0: Point::new(0.6321957f32, 0.6984592f32),
        control_0: Point::new(0.6426781f32, 0.71549046f32),
        control_1: Point::new(0.65316045f32, 0.7325218f32),
        anchor_1: Point::new(0.6636428f32, 0.749553f32),
    },
    Cubic {
        anchor_0: Point::new(0.6636428f32, 0.749553f32),
        control_0: Point::new(0.68026865f32, 0.77656585f32),
        control_1: Point::new(0.6931978f32, 0.8056835f32),
        anchor_1: Point::new(0.70208657f32, 0.8361318f32),
    },
    Cubic {
        anchor_0: Point::new(0.70208657f32, 0.8361318f32),
        control_0: Point::new(0.70539176f32, 0.8474537f32),
        control_1: Point::new(0.708697f32, 0.85877556f32),
        anchor_1: Point::new(0.7120023f32, 0.8700975f32),
    },
    Cubic {
        anchor_0: Point::new(0.7120023f32, 0.8700975f32),
        control_0: Point::new(0.7171595f32, 0.8877635f32),
        control_1: Point::new(0.7184829f32, 0.9063275f32),
        anchor_1: Point::new(0.715884f32, 0.92454636f32),
    },
    Cubic {
        anchor_0: Point::new(0.715884f32, 0.92454636f32),
        control_0: Point::new(0.7155807f32, 0.9266717f32),
        control_1: Point::new(0.7152776f32, 0.92879695f32),
        anchor_1: Point::new(0.71497446f32, 0.93092227f32),
    },
    Cubic {
        anchor_0: Point::new(0.71497446f32, 0.93092227f32),
        control_0: Point::new(0.7129787f32, 0.94491243f32),
        control_1: Point::new(0.70373666f32, 0.9568056f32),
        anchor_1: Point::new(0.6906729f32, 0.9621949f32),
    },
    Cubic {
        anchor_0: Point::new(0.6906729f32, 0.9621949f32),
        control_0: Point::new(0.677609f32, 0.96758425f32),
        control_1: Point::new(0.6626696f32, 0.96566683f32),
        anchor_1: Point::new(0.65139014f32, 0.95715314f32),
    },
    Cubic {
        anchor_0: Point::new(0.65139014f32, 0.95715314f32),
        control_0: Point::new(0.64967656f32, 0.9558597f32),
        control_1: Point::new(0.6479631f32, 0.9545664f32),
        anchor_1: Point::new(0.6462496f32, 0.95327306f32),
    },
    Cubic {
        anchor_0: Point::new(0.6462496f32, 0.95327306f32),
        control_0: Point::new(0.6315608f32, 0.942186f32),
        control_1: Point::new(0.6194104f32, 0.9280887f32),
        anchor_1: Point::new(0.6106116f32, 0.9119251f32),
    },
    Cubic {
        anchor_0: Point::new(0.6106116f32, 0.9119251f32),
        control_0: Point::new(0.60497254f32, 0.90156597f32),
        control_1: Point::new(0.5993335f32, 0.89120686f32),
        anchor_1: Point::new(0.59369445f32, 0.88084775f32),
    },
    Cubic {
        anchor_0: Point::new(0.59369445f32, 0.88084775f32),
        control_0: Point::new(0.5785292f32, 0.85298854f32),
        control_1: Point::new(0.5671675f32, 0.8232245f32),
        anchor_1: Point::new(0.5599111f32, 0.7923463f32),
    },
    Cubic {
        anchor_0: Point::new(0.5599111f32, 0.7923463f32),
        control_0: Point::new(0.555336f32, 0.772878f32),
        control_1: Point::new(0.5507609f32, 0.7534098f32),
        anchor_1: Point::new(0.5461859f32, 0.7339415f32),
    },
    Cubic {
        anchor_0: Point::new(0.5461859f32, 0.7339415f32),
        control_0: Point::new(0.5493527f32, 0.75368786f32),
        control_1: Point::new(0.5525196f32, 0.7734342f32),
        anchor_1: Point::new(0.5556865f32, 0.7931805f32),
    },
    Cubic {
        anchor_0: Point::new(0.5556865f32, 0.7931805f32),
        control_0: Point::new(0.5607091f32, 0.8244984f32),
        control_1: Point::new(0.5615113f32, 0.85634923f32),
        anchor_1: Point::new(0.5580716f32, 0.88788027f32),
    },
    Cubic {
        anchor_0: Point::new(0.5580716f32, 0.88788027f32),
        control_0: Point::new(0.55679256f32, 0.8996052f32),
        control_1: Point::new(0.5555134f32, 0.9113302f32),
        anchor_1: Point::new(0.5542344f32, 0.92305523f32),
    },
    Cubic {
        anchor_0: Point::new(0.5542344f32, 0.92305523f32),
        control_0: Point::new(0.5522386f32, 0.9413501f32),
        control_1: Point::new(0.5463571f32, 0.9590074f32),
        anchor_1: Point::new(0.53698385f32, 0.97484493f32),
    },
    Cubic {
        anchor_0: Point::new(0.53698385f32, 0.97484493f32),
        control_0: Point::new(0.5358904f32, 0.9766924f32),
        control_1: Point::new(0.5347971f32, 0.97853994f32),
        anchor_1: Point::new(0.5337037f32, 0.9803874f32),
    },
    Cubic {
        anchor_0: Point::new(0.5337037f32, 0.9803874f32),
        control_0: Point::new(0.52650607f32, 0.9925489f32),
        control_1: Point::new(0.51341623f32, 1.0f32),
        anchor_1: Point::new(0.49928442f32, 0.9999798f32),
    },
    Cubic {
        anchor_0: Point::new(0.49928442f32, 0.9999798f32),
        control_0: Point::new(0.4851526f32, 0.9999595f32),
        control_1: Point::new(0.47208408f32, 0.99247104f32),
        anchor_1: Point::new(0.4649213f32, 0.9802889f32),
    },
    Cubic {
        anchor_0: Point::new(0.4649213f32, 0.9802889f32),
        control_0: Point::new(0.46383315f32, 0.97843826f32),
        control_1: Point::new(0.46274504f32, 0.97658765f32),
        anchor_1: Point::new(0.46165693f32, 0.97473705f32),
    },
    Cubic {
        anchor_0: Point::new(0.46165693f32, 0.97473705f32),
        control_0: Point::new(0.4523291f32, 0.9588728f32),
        control_1: Point::new(0.4464983f32, 0.9411989f32),
        anchor_1: Point::new(0.44455487f32, 0.9228984f32),
    },
    Cubic {
        anchor_0: Point::new(0.44455487f32, 0.9228984f32),
        control_0: Point::new(0.44330934f32, 0.9111699f32),
        control_1: Point::new(0.44206387f32, 0.8994414f32),
        anchor_1: Point::new(0.44081837f32, 0.8877129f32),
    },
    Cubic {
        anchor_0: Point::new(0.44081837f32, 0.8877129f32),
        control_0: Point::new(0.43746883f32, 0.85617155f32),
        control_1: Point::new(0.43836212f32, 0.8243236f32),
        anchor_1: Point::new(0.44347456f32, 0.79301965f32),
    },
    Cubic {
        anchor_0: Point::new(0.44347456f32, 0.79301965f32),
        control_0: Point::new(0.4466979f32, 0.7732826f32),
        control_1: Point::new(0.44992128f32, 0.75354546f32),
        anchor_1: Point::new(0.45314464f32, 0.73380834f32),
    },
    Cubic {
        anchor_0: Point::new(0.45314464f32, 0.73380834f32),
        control_0: Point::new(0.44851387f32, 0.7532634f32),
        control_1: Point::new(0.44388312f32, 0.7727184f32),
        anchor_1: Point::new(0.4392523f32, 0.7921735f32),
    },
    Cubic {
        anchor_0: Point::new(0.4392523f32, 0.7921735f32),
        control_0: Point::new(0.43190762f32, 0.8230304f32),
        control_1: Point::new(0.42046037f32, 0.8527627f32),
        anchor_1: Point::new(0.40521565f32, 0.880578f32),
    },
    Cubic {
        anchor_0: Point::new(0.40521565f32, 0.880578f32),
        control_0: Point::new(0.39954695f32, 0.8909209f32),
        control_1: Point::new(0.3938783f32, 0.90126383f32),
        anchor_1: Point::new(0.38820967f32, 0.9116068f32),
    },
    Cubic {
        anchor_0: Point::new(0.38820967f32, 0.9116068f32),
        control_0: Point::new(0.37936473f32, 0.9277451f32),
        control_1: Point::new(0.36717385f32, 0.9418077f32),
        anchor_1: Point::new(0.3524535f32, 0.9528526f32),
    },
    Cubic {
        anchor_0: Point::new(0.3524535f32, 0.9528526f32),
        control_0: Point::new(0.3507363f32, 0.954141f32),
        control_1: Point::new(0.34901908f32, 0.9554295f32),
        anchor_1: Point::new(0.3473019f32, 0.9567179f32),
    },
    Cubic {
        anchor_0: Point::new(0.3473019f32, 0.9567179f32),
        control_0: Point::new(0.33599815f32, 0.96519923f32),
        control_1: Point::new(0.3210532f32, 0.967074f32),
        anchor_1: Point::new(0.30800486f32, 0.9616472f32),
    },
    Cubic {
        anchor_0: Point::new(0.30800486f32, 0.9616472f32),
        control_0: Point::new(0.2949565f32, 0.9562205f32),
        control_1: Point::new(0.28574854f32, 0.9443009f32),
        anchor_1: Point::new(0.2837929f32, 0.93030506f32),
    },
    Cubic {
        anchor_0: Point::new(0.2837929f32, 0.93030506f32),
        control_0: Point::new(0.28349578f32, 0.92817885f32),
        control_1: Point::new(0.2831987f32, 0.9260528f32),
        anchor_1: Point::new(0.2829016f32, 0.9239266f32),
    },
    Cubic {
        anchor_0: Point::new(0.2829016f32, 0.9239266f32),
        control_0: Point::new(0.2803548f32, 0.90570045f32),
        control_1: Point::new(0.2817314f32, 0.88714015f32),
        anchor_1: Point::new(0.28693917f32, 0.86948913f32),
    },
    Cubic {
        anchor_0: Point::new(0.28693917f32, 0.86948913f32),
        control_0: Point::new(0.29027677f32, 0.8581767f32),
        control_1: Point::new(0.2936144f32, 0.84686434f32),
        anchor_1: Point::new(0.29695204f32, 0.835552f32),
    },
    Cubic {
        anchor_0: Point::new(0.29695204f32, 0.835552f32),
        control_0: Point::new(0.30592796f32, 0.8051293f32),
        control_1: Point::new(0.31894046f32, 0.77604866f32),
        anchor_1: Point::new(0.3356435f32, 0.7490836f32),
    },
    Cubic {
        anchor_0: Point::new(0.3356435f32, 0.7490836f32),
        control_0: Point::new(0.34617457f32, 0.73208237f32),
        control_1: Point::new(0.35670564f32, 0.71508116f32),
        anchor_1: Point::new(0.36723673f32, 0.69808f32),
    },
    Cubic {
        anchor_0: Point::new(0.36723673f32, 0.69808f32),
        control_0: Point::new(0.35551333f32, 0.714282f32),
        control_1: Point::new(0.34378994f32, 0.730484f32),
        anchor_1: Point::new(0.33206654f32, 0.746686f32),
    },
    Cubic {
        anchor_0: Point::new(0.33206654f32, 0.746686f32),
        control_0: Point::new(0.31347248f32, 0.77238333f32),
        control_1: Point::new(0.29151845f32, 0.7954718f32),
        anchor_1: Point::new(0.26678964f32, 0.81533587f32),
    },
    Cubic {
        anchor_0: Point::new(0.26678964f32, 0.81533587f32),
        control_0: Point::new(0.25759444f32, 0.82272214f32),
        control_1: Point::new(0.24839923f32, 0.83010846f32),
        anchor_1: Point::new(0.23920402f32, 0.83749473f32),
    },
    Cubic {
        anchor_0: Point::new(0.23920402f32, 0.83749473f32),
        control_0: Point::new(0.2248565f32, 0.8490198f32),
        control_1: Point::new(0.20821217f32, 0.85734665f32),
        anchor_1: Point::new(0.19038565f32, 0.86191756f32),
    },
    Cubic {
        anchor_0: Point::new(0.19038565f32, 0.86191756f32),
        control_0: Point::new(0.18830608f32, 0.8624508f32),
        control_1: Point::new(0.18622655f32, 0.862984f32),
        anchor_1: Point::new(0.18414699f32, 0.8635172f32),
    },
    Cubic {
        anchor_0: Point::new(0.18414699f32, 0.8635172f32),
        control_0: Point::new(0.17045799f32, 0.8670273f32),
        control_1: Point::new(0.15593328f32, 0.86304003f32),
        anchor_1: Point::new(0.14595488f32, 0.853033f32),
    },
    Cubic {
        anchor_0: Point::new(0.14595488f32, 0.853033f32),
        control_0: Point::new(0.13597654f32, 0.8430261f32),
        control_1: Point::new(0.13203093f32, 0.8284901f32),
        anchor_1: Point::new(0.1355801f32, 0.8148113f32),
    },
    Cubic {
        anchor_0: Point::new(0.1355801f32, 0.8148113f32),
        control_0: Point::new(0.13611925f32, 0.81273323f32),
        control_1: Point::new(0.1366584f32, 0.8106553f32),
        anchor_1: Point::new(0.13719755f32, 0.8085773f32),
    },
    Cubic {
        anchor_0: Point::new(0.13719755f32, 0.8085773f32),
        control_0: Point::new(0.14181949f32, 0.7907638f32),
        control_1: Point::new(0.15019402f32, 0.77414316f32),
        anchor_1: Point::new(0.16176017f32, 0.75982857f32),
    },
    Cubic {
        anchor_0: Point::new(0.16176017f32, 0.75982857f32),
        control_0: Point::new(0.16917282f32, 0.7506545f32),
        control_1: Point::new(0.1765855f32, 0.7414804f32),
        anchor_1: Point::new(0.18399814f32, 0.73230636f32),
    },
    Cubic {
        anchor_0: Point::new(0.18399814f32, 0.73230636f32),
        control_0: Point::new(0.20393234f32, 0.7076353f32),
        control_1: Point::new(0.22708431f32, 0.6857467f32),
        anchor_1: Point::new(0.25283402f32, 0.66722685f32),
    },
    Cubic {
        anchor_0: Point::new(0.25283402f32, 0.66722685f32),
        control_0: Point::new(0.26906958f32, 0.6555498f32),
        control_1: Point::new(0.28530517f32, 0.6438728f32),
        anchor_1: Point::new(0.30154076f32, 0.6321957f32),
    },
    Cubic {
        anchor_0: Point::new(0.30154076f32, 0.6321957f32),
        control_0: Point::new(0.28450954f32, 0.642678f32),
        control_1: Point::new(0.26747835f32, 0.6531604f32),
        anchor_1: Point::new(0.25044712f32, 0.6636427f32),
    },
    Cubic {
        anchor_0: Point::new(0.25044712f32, 0.6636427f32),
        control_0: Point::new(0.22343464f32, 0.6802683f32),
        control_1: Point::new(0.19431588f32, 0.69319797f32),
        anchor_1: Point::new(0.16386795f32, 0.7020866f32),
    },
    Cubic {
        anchor_0: Point::new(0.16386795f32, 0.7020866f32),
        control_0: Point::new(0.15254606f32, 0.70539176f32),
        control_1: Point::new(0.14122419f32, 0.7086971f32),
        anchor_1: Point::new(0.1299023f32, 0.7120022f32),
    },
    Cubic {
        anchor_0: Point::new(0.1299023f32, 0.7120022f32),
        control_0: Point::new(0.11223643f32, 0.7171594f32),
        control_1: Point::new(0.09367255f32, 0.7184828f32),
        anchor_1: Point::new(0.07545373f32, 0.715884f32),
    },
    Cubic {
        anchor_0: Point::new(0.07545373f32, 0.715884f32),
        control_0: Point::new(0.07332841f32, 0.7155807f32),
        control_1: Point::new(0.07120309f32, 0.7152776f32),
        anchor_1: Point::new(0.069077775f32, 0.71497446f32),
    },
    Cubic {
        anchor_0: Point::new(0.069077775f32, 0.71497446f32),
        control_0: Point::new(0.055087604f32, 0.7129787f32),
        control_1: Point::new(0.043194428f32, 0.7037367f32),
        anchor_1: Point::new(0.03780509f32, 0.69067293f32),
    },
    Cubic {
        anchor_0: Point::new(0.03780509f32, 0.69067293f32),
        control_0: Point::new(0.03241576f32, 0.67760915f32),
        control_1: Point::new(0.034333166f32, 0.6626697f32),
        anchor_1: Point::new(0.04284684f32, 0.65139025f32),
    },
    Cubic {
        anchor_0: Point::new(0.04284684f32, 0.65139025f32),
        control_0: Point::new(0.044140186f32, 0.6496767f32),
        control_1: Point::new(0.045433536f32, 0.6479633f32),
        anchor_1: Point::new(0.04672686f32, 0.6462498f32),
    },
    Cubic {
        anchor_0: Point::new(0.04672686f32, 0.6462498f32),
        control_0: Point::new(0.057813946f32, 0.6315609f32),
        control_1: Point::new(0.07191122f32, 0.6194104f32),
        anchor_1: Point::new(0.08807496f32, 0.6106116f32),
    },
    Cubic {
        anchor_0: Point::new(0.08807496f32, 0.6106116f32),
        control_0: Point::new(0.09843405f32, 0.6049726f32),
        control_1: Point::new(0.108793154f32, 0.5993336f32),
        anchor_1: Point::new(0.11915222f32, 0.5936945f32),
    },
    Cubic {
        anchor_0: Point::new(0.11915222f32, 0.5936945f32),
        control_0: Point::new(0.1470107f32, 0.5785296f32),
        control_1: Point::new(0.17677638f32, 0.5671673f32),
        anchor_1: Point::new(0.2076538f32, 0.5599111f32),
    },
    Cubic {
        anchor_0: Point::new(0.2076538f32, 0.5599111f32),
        control_0: Point::new(0.22712202f32, 0.555336f32),
        control_1: Point::new(0.24659027f32, 0.5507609f32),
        anchor_1: Point::new(0.2660585f32, 0.5461859f32),
    },
    Cubic {
        anchor_0: Point::new(0.2660585f32, 0.5461859f32),
        control_0: Point::new(0.24631226f32, 0.5493527f32),
        control_1: Point::new(0.22656603f32, 0.55251956f32),
        anchor_1: Point::new(0.2068198f32, 0.5556865f32),
    },
    Cubic {
        anchor_0: Point::new(0.2068198f32, 0.5556865f32),
        control_0: Point::new(0.17550133f32, 0.56070924f32),
        control_1: Point::new(0.14365095f32, 0.5615114f32),
        anchor_1: Point::new(0.11211934f32, 0.55807155f32),
    },
    Cubic {
        anchor_0: Point::new(0.11211934f32, 0.55807155f32),
        control_0: Point::new(0.10039443f32, 0.5567925f32),
        control_1: Point::new(0.08866952f32, 0.5555134f32),
        anchor_1: Point::new(0.07694463f32, 0.5542343f32),
    },
    Cubic {
        anchor_0: Point::new(0.07694463f32, 0.5542343f32),
        control_0: Point::new(0.058649834f32, 0.5522385f32),
        control_1: Point::new(0.0409926f32, 0.5463571f32),
        anchor_1: Point::new(0.025155108f32, 0.5369839f32),
    },
    Cubic {
        anchor_0: Point::new(0.025155108f32, 0.5369839f32),
        control_0: Point::new(0.023307608f32, 0.53589046f32),
        control_1: Point::new(0.02146011f32, 0.53479713f32),
        anchor_1: Point::new(0.019612623f32, 0.5337037f32),
    },
    Cubic {
        anchor_0: Point::new(0.019612623f32, 0.5337037f32),
        control_0: Point::new(0.0074510965f32, 0.52650607f32),
        control_1: Point::new(0.0f32, 0.5134162f32),
        anchor_1: Point::new(2.0223804E-5f32, 0.49928436f32),
    },
    Cubic {
        anchor_0: Point::new(2.0223804E-5f32, 0.49928436f32),
        control_0: Point::new(4.0451352E-5f32, 0.48515257f32),
        control_1: Point::new(0.007528986f32, 0.4720841f32),
        anchor_1: Point::new(0.01971104f32, 0.46492133f32),
    },
    Cubic {
        anchor_0: Point::new(0.01971104f32, 0.46492133f32),
        control_0: Point::new(0.021561641f32, 0.46383318f32),
        control_1: Point::new(0.023412248f32, 0.4627451f32),
        anchor_1: Point::new(0.025262855f32, 0.461657f32),
    },
    Cubic {
        anchor_0: Point::new(0.025262855f32, 0.461657f32),
        control_0: Point::new(0.041127168f32, 0.4523291f32),
        control_1: Point::new(0.058801234f32, 0.44649827f32),
        anchor_1: Point::new(0.077101745f32, 0.44455487f32),
    },
    Cubic {
        anchor_0: Point::new(0.077101745f32, 0.44455487f32),
        control_0: Point::new(0.088830374f32, 0.44330934f32),
        control_1: Point::new(0.10055902f32, 0.44206387f32),
        anchor_1: Point::new(0.112287655f32, 0.4408183f32),
    },
    Cubic {
        anchor_0: Point::new(0.112287655f32, 0.4408183f32),
        control_0: Point::new(0.1438287f32, 0.43746886f32),
        control_1: Point::new(0.1756762f32, 0.43836215f32),
        anchor_1: Point::new(0.20697987f32, 0.44347456f32),
    },
    Cubic {
        anchor_0: Point::new(0.20697987f32, 0.44347456f32),
        control_0: Point::new(0.22671713f32, 0.4466979f32),
        control_1: Point::new(0.24645439f32, 0.44992125f32),
        anchor_1: Point::new(0.26619166f32, 0.4531446f32),
    },
    Cubic {
        anchor_0: Point::new(0.26619166f32, 0.4531446f32),
        control_0: Point::new(0.24673645f32, 0.44851378f32),
        control_1: Point::new(0.22728127f32, 0.443883f32),
        anchor_1: Point::new(0.2078261f32, 0.43925214f32),
    },
    Cubic {
        anchor_0: Point::new(0.2078261f32, 0.43925214f32),
        control_0: Point::new(0.17696987f32, 0.4319076f32),
        control_1: Point::new(0.14723709f32, 0.4204602f32),
        anchor_1: Point::new(0.11942238f32, 0.4052158f32),
    },
    Cubic {
        anchor_0: Point::new(0.11942238f32, 0.4052158f32),
        control_0: Point::new(0.10907935f32, 0.3995471f32),
        control_1: Point::new(0.09873633f32, 0.39387843f32),
        anchor_1: Point::new(0.08839332f32, 0.3882097f32),
    },
    Cubic {
        anchor_0: Point::new(0.08839332f32, 0.3882097f32),
        control_0: Point::new(0.072254814f32, 0.3793647f32),
        control_1: Point::new(0.058192354f32, 0.3671739f32),
        anchor_1: Point::new(0.047147345f32, 0.3524534f32),
    },
    Cubic {
        anchor_0: Point::new(0.047147345f32, 0.3524534f32),
        control_0: Point::new(0.045858927f32, 0.35073623f32),
        control_1: Point::new(0.044570513f32, 0.34901908f32),
        anchor_1: Point::new(0.043282107f32, 0.3473019f32),
    },
    Cubic {
        anchor_0: Point::new(0.043282107f32, 0.3473019f32),
        control_0: Point::new(0.03480073f32, 0.33599815f32),
        control_1: Point::new(0.03292608f32, 0.32105327f32),
        anchor_1: Point::new(0.038352773f32, 0.30800492f32),
    },
    Cubic {
        anchor_0: Point::new(0.038352773f32, 0.30800492f32),
        control_0: Point::new(0.043779463f32, 0.29495656f32),
        control_1: Point::new(0.05569906f32, 0.28574863f32),
        anchor_1: Point::new(0.0696949f32, 0.28379294f32),
    },
    Cubic {
        anchor_0: Point::new(0.0696949f32, 0.28379294f32),
        control_0: Point::new(0.071821034f32, 0.28349584f32),
        control_1: Point::new(0.07394718f32, 0.28319877f32),
        anchor_1: Point::new(0.07607332f32, 0.28290164f32),
    },
    Cubic {
        anchor_0: Point::new(0.07607332f32, 0.28290164f32),
        control_0: Point::new(0.094299585f32, 0.28035483f32),
        control_1: Point::new(0.11285973f32, 0.28173146f32),
        anchor_1: Point::new(0.13051085f32, 0.28693923f32),
    },
    Cubic {
        anchor_0: Point::new(0.13051085f32, 0.28693923f32),
        control_0: Point::new(0.14182332f32, 0.29027686f32),
        control_1: Point::new(0.15313578f32, 0.2936145f32),
        anchor_1: Point::new(0.16444825f32, 0.29695213f32),
    },
    Cubic {
        anchor_0: Point::new(0.16444825f32, 0.29695213f32),
        control_0: Point::new(0.19487005f32, 0.3059278f32),
        control_1: Point::new(0.22395188f32, 0.31894082f32),
        anchor_1: Point::new(0.2509162f32, 0.33564335f32),
    },
    Cubic {
        anchor_0: Point::new(0.2509162f32, 0.33564335f32),
        control_0: Point::new(0.26791748f32, 0.34617445f32),
        control_1: Point::new(0.2849188f32, 0.35670555f32),
        anchor_1: Point::new(0.30192006f32, 0.36723664f32),
    },
    Cubic {
        anchor_0: Point::new(0.30192006f32, 0.36723664f32),
        control_0: Point::new(0.28571793f32, 0.35551316f32),
        control_1: Point::new(0.2695158f32, 0.34378967f32),
        anchor_1: Point::new(0.25331366f32, 0.3320662f32),
    },
    Cubic {
        anchor_0: Point::new(0.25331366f32, 0.3320662f32),
        control_0: Point::new(0.22761713f32, 0.31347278f32),
        control_1: Point::new(0.20452796f32, 0.29151806f32),
        anchor_1: Point::new(0.18466447f32, 0.2667901f32),
    },
    Cubic {
        anchor_0: Point::new(0.18466447f32, 0.2667901f32),
        control_0: Point::new(0.1772781f32, 0.25759485f32),
        control_1: Point::new(0.16989173f32, 0.2483996f32),
        anchor_1: Point::new(0.16250536f32, 0.23920433f32),
    },
    Cubic {
        anchor_0: Point::new(0.16250536f32, 0.23920433f32),
        control_0: Point::new(0.15098014f32, 0.22485664f32),
        control_1: Point::new(0.1426533f32, 0.20821218f32),
        anchor_1: Point::new(0.13808234f32, 0.19038542f32),
    },
    Cubic {
        anchor_0: Point::new(0.13808234f32, 0.19038542f32),
        control_0: Point::new(0.13754913f32, 0.18830588f32),
        control_1: Point::new(0.13701594f32, 0.18622638f32),
        anchor_1: Point::new(0.13648272f32, 0.18414687f32),
    },
    Cubic {
        anchor_0: Point::new(0.13648272f32, 0.18414687f32),
        control_0: Point::new(0.13297275f32, 0.17045791f32),
        control_1: Point::new(0.13695998f32, 0.15593328f32),
        anchor_1: Point::new(0.14696698f32, 0.14595489f32),
    },
    Cubic {
        anchor_0: Point::new(0.14696698f32, 0.14595489f32),
        control_0: Point::new(0.15697397f32, 0.13597652f32),
        control_1: Point::new(0.17150997f32, 0.1320309f32),
        anchor_1: Point::new(0.18518883f32, 0.13558006f32),
    },
    Cubic {
        anchor_0: Point::new(0.18518883f32, 0.13558006f32),
        control_0: Point::new(0.18726683f32, 0.13611922f32),
        control_1: Point::new(0.18934482f32, 0.13665839f32),
        anchor_1: Point::new(0.1914228f32, 0.13719754f32),
    },
    Cubic {
        anchor_0: Point::new(0.1914228f32, 0.13719754f32),
        control_0: Point::new(0.20923631f32, 0.1418195f32),
        control_1: Point::new(0.22585683f32, 0.15019396f32),
        anchor_1: Point::new(0.2401714f32, 0.16176012f32),
    },
    Cubic {
        anchor_0: Point::new(0.2401714f32, 0.16176012f32),
        control_0: Point::new(0.2493455f32, 0.1691728f32),
        control_1: Point::new(0.25851962f32, 0.17658547f32),
        anchor_1: Point::new(0.2676937f32, 0.18399815f32),
    },
    Cubic {
        anchor_0: Point::new(0.2676937f32, 0.18399815f32),
        control_0: Point::new(0.29236495f32, 0.20393252f32),
        control_1: Point::new(0.3142531f32, 0.22708406f32),
        anchor_1: Point::new(0.33277315f32, 0.25283396f32),
    },
    Cubic {
        anchor_0: Point::new(0.33277315f32, 0.25283396f32),
        control_0: Point::new(0.3444502f32, 0.26906952f32),
        control_1: Point::new(0.3561273f32, 0.28530514f32),
        anchor_1: Point::new(0.36780435f32, 0.3015407f32),
    },
    Cubic {
        anchor_0: Point::new(0.36780435f32, 0.3015407f32),
        control_0: Point::new(0.35732192f32, 0.28450933f32),
        control_1: Point::new(0.34683952f32, 0.26747796f32),
        anchor_1: Point::new(0.3363571f32, 0.25044662f32),
    },
    Cubic {
        anchor_0: Point::new(0.3363571f32, 0.25044662f32),
        control_0: Point::new(0.31973183f32, 0.2234346f32),
        control_1: Point::new(0.30680212f32, 0.19431582f32),
        anchor_1: Point::new(0.29791352f32, 0.16386847f32),
    },
    Cubic {
        anchor_0: Point::new(0.29791352f32, 0.16386847f32),
        control_0: Point::new(0.29460827f32, 0.15254648f32),
        control_1: Point::new(0.29130304f32, 0.1412245f32),
        anchor_1: Point::new(0.28799772f32, 0.12990254f32),
    },
    Cubic {
        anchor_0: Point::new(0.28799772f32, 0.12990254f32),
        control_0: Point::new(0.28284046f32, 0.112236544f32),
        control_1: Point::new(0.281517f32, 0.093672596f32),
        anchor_1: Point::new(0.28411597f32, 0.07545364f32),
    },
    Cubic {
        anchor_0: Point::new(0.28411597f32, 0.07545364f32),
        control_0: Point::new(0.28441915f32, 0.07332837f32),
        control_1: Point::new(0.28472233f32, 0.071203105f32),
        anchor_1: Point::new(0.2850255f32, 0.069077834f32),
    },
    Cubic {
        anchor_0: Point::new(0.2850255f32, 0.069077834f32),
        control_0: Point::new(0.28702122f32, 0.055087637f32),
        control_1: Point::new(0.2962633f32, 0.043194428f32),
        anchor_1: Point::new(0.30932713f32, 0.037805106f32),
    },
    Cubic {
        anchor_0: Point::new(0.30932713f32, 0.037805106f32),
        control_0: Point::new(0.3223909f32, 0.032415796f32),
        control_1: Point::new(0.3373303f32, 0.034333188f32),
        anchor_1: Point::new(0.34860975f32, 0.04284681f32),
    },
    Cubic {
        anchor_0: Point::new(0.34860975f32, 0.04284681f32),
        control_0: Point::new(0.3503232f32, 0.04414013f32),
        control_1: Point::new(0.35203665f32, 0.045433454f32),
        anchor_1: Point::new(0.35375017f32, 0.04672677f32),
    },
    Cubic {
        anchor_0: Point::new(0.35375017f32, 0.04672677f32),
        control_0: Point::new(0.36843908f32, 0.05781386f32),
        control_1: Point::new(0.3805898f32, 0.07191136f32),
        anchor_1: Point::new(0.38938865f32, 0.088075176f32),
    },
    Cubic {
        anchor_0: Point::new(0.38938865f32, 0.088075176f32),
        control_0: Point::new(0.39502764f32, 0.0984343f32),
        control_1: Point::new(0.40066668f32, 0.10879343f32),
        anchor_1: Point::new(0.40630564f32, 0.119152576f32),
    },
    Cubic {
        anchor_0: Point::new(0.40630564f32, 0.119152576f32),
        control_0: Point::new(0.42147055f32, 0.14701119f32),
        control_1: Point::new(0.43283257f32, 0.1767759f32),
        anchor_1: Point::new(0.44008887f32, 0.20765346f32),
    },
    Cubic {
        anchor_0: Point::new(0.44008887f32, 0.20765346f32),
        control_0: Point::new(0.44466397f32, 0.2271218f32),
        control_1: Point::new(0.44923905f32, 0.24659017f32),
        anchor_1: Point::new(0.45381415f32, 0.2660585f32),
    },
    Cubic {
        anchor_0: Point::new(0.45381415f32, 0.2660585f32),
        control_0: Point::new(0.45064726f32, 0.24631225f32),
        control_1: Point::new(0.44748047f32, 0.226566f32),
        anchor_1: Point::new(0.44431362f32, 0.20681976f32),
    },
    Cubic {
        anchor_0: Point::new(0.44431362f32, 0.20681976f32),
        control_0: Point::new(0.43929076f32, 0.17550083f32),
        control_1: Point::new(0.43848857f32, 0.14365146f32),
        anchor_1: Point::new(0.44192845f32, 0.112119354f32),
    },
    Cubic {
        anchor_0: Point::new(0.44192845f32, 0.112119354f32),
        control_0: Point::new(0.4432075f32, 0.10039438f32),
        control_1: Point::new(0.4444866f32, 0.08866942f32),
        anchor_1: Point::new(0.4457657f32, 0.07694446f32),
    },
    Cubic {
        anchor_0: Point::new(0.4457657f32, 0.07694446f32),
        control_0: Point::new(0.4477615f32, 0.058649737f32),
        control_1: Point::new(0.45364276f32, 0.040992692f32),
        anchor_1: Point::new(0.46301588f32, 0.025155235f32),
    },
    Cubic {
        anchor_0: Point::new(0.46301588f32, 0.025155235f32),
        control_0: Point::new(0.46410924f32, 0.023307782f32),
        control_1: Point::new(0.46520266f32, 0.021460328f32),
        anchor_1: Point::new(0.466296f32, 0.019612877f32),
    },
    Cubic {
        anchor_0: Point::new(0.466296f32, 0.019612877f32),
        control_0: Point::new(0.47349364f32, 0.0074512092f32),
        control_1: Point::new(0.48658368f32, 0.0f32),
        anchor_1: Point::new(0.5007156f32, 2.0248819E-5f32),
    },
    Cubic {
        anchor_0: Point::new(0.5007156f32, 2.0248819E-5f32),
        control_0: Point::new(0.5148476f32, 4.0497638E-5f32),
        control_1: Point::new(0.52791613f32, 0.0075291377f32),
        anchor_1: Point::new(0.53507894f32, 0.019711325f32),
    },
    Cubic {
        anchor_0: Point::new(0.53507894f32, 0.019711325f32),
        control_0: Point::new(0.536167f32, 0.02156192f32),
        control_1: Point::new(0.53725517f32, 0.02341252f32),
        anchor_1: Point::new(0.5383432f32, 0.025263106f32),
    },
    Cubic {
        anchor_0: Point::new(0.5383432f32, 0.025263106f32),
        control_0: Point::new(0.5476709f32, 0.04112721f32),
        control_1: Point::new(0.55350167f32, 0.058801185f32),
        anchor_1: Point::new(0.55544513f32, 0.077101395f32),
    },
    Cubic {
        anchor_0: Point::new(0.55544513f32, 0.077101395f32),
        control_0: Point::new(0.55669063f32, 0.08882987f32),
        control_1: Point::new(0.5579362f32, 0.10055835f32),
        anchor_1: Point::new(0.5591817f32, 0.112286836f32),
    },
    Cubic {
        anchor_0: Point::new(0.5591817f32, 0.112286836f32),
        control_0: Point::new(0.56253123f32, 0.14382806f32),
        control_1: Point::new(0.56163794f32, 0.1756769f32),
        anchor_1: Point::new(0.5565255f32, 0.20698078f32),
    },
    Cubic {
        anchor_0: Point::new(0.5565255f32, 0.20698078f32),
        control_0: Point::new(0.5533021f32, 0.22671774f32),
        control_1: Point::new(0.55007875f32, 0.2464547f32),
        anchor_1: Point::new(0.54685545f32, 0.26619166f32),
    },
    Cubic {
        anchor_0: Point::new(0.54685545f32, 0.26619166f32),
        control_0: Point::new(0.55148613f32, 0.24673665f32),
        control_1: Point::new(0.556117f32, 0.22728164f32),
        anchor_1: Point::new(0.56074774f32, 0.20782664f32),
    },
    Cubic {
        anchor_0: Point::new(0.56074774f32, 0.20782664f32),
        control_0: Point::new(0.56809247f32, 0.17696956f32),
        control_1: Point::new(0.5795397f32, 0.14723748f32),
        anchor_1: Point::new(0.59478444f32, 0.11942196f32),
    },
    Cubic {
        anchor_0: Point::new(0.59478444f32, 0.11942196f32),
        control_0: Point::new(0.6004531f32, 0.109079f32),
        control_1: Point::new(0.6061218f32, 0.098736055f32),
        anchor_1: Point::new(0.6117904f32, 0.088393115f32),
    },
    Cubic {
        anchor_0: Point::new(0.6117904f32, 0.088393115f32),
        control_0: Point::new(0.62063545f32, 0.07225473f32),
        control_1: Point::new(0.6328261f32, 0.058192447f32),
        anchor_1: Point::new(0.64754647f32, 0.04714748f32),
    },
    Cubic {
        anchor_0: Point::new(0.64754647f32, 0.04714748f32),
        control_0: Point::new(0.64926356f32, 0.04585907f32),
        control_1: Point::new(0.6509808f32, 0.044570666f32),
        anchor_1: Point::new(0.65269786f32, 0.04328225f32),
    },
    Cubic {
        anchor_0: Point::new(0.65269786f32, 0.04328225f32),
        control_0: Point::new(0.6640017f32, 0.034800798f32),
        control_1: Point::new(0.6789467f32, 0.0329261f32),
        anchor_1: Point::new(0.69199514f32, 0.03835282f32),
    },
    Cubic {
        anchor_0: Point::new(0.69199514f32, 0.03835282f32),
        control_0: Point::new(0.7050436f32, 0.04377955f32),
        control_1: Point::new(0.7142517f32, 0.055699304f32),
        anchor_1: Point::new(0.7162073f32, 0.06969529f32),
    },
    Cubic {
        anchor_0: Point::new(0.7162073f32, 0.06969529f32),
        control_0: Point::new(0.7165044f32, 0.07182138f32),
        control_1: Point::new(0.71680146f32, 0.07394748f32),
        anchor_1: Point::new(0.71709853f32, 0.07607357f32),
    },
    Cubic {
        anchor_0: Point::new(0.71709853f32, 0.07607357f32),
        control_0: Point::new(0.71964526f32, 0.09429974f32),
        control_1: Point::new(0.71826863f32, 0.112859674f32),
        anchor_1: Point::new(0.71306086f32, 0.13051069f32),
    },
    Cubic {
        anchor_0: Point::new(0.71306086f32, 0.13051069f32),
        control_0: Point::new(0.7097232f32, 0.14182319f32),
        control_1: Point::new(0.7063856f32, 0.15313567f32),
        anchor_1: Point::new(0.703048f32, 0.16444817f32),
    },
    Cubic {
        anchor_0: Point::new(0.703048f32, 0.16444817f32),
        control_0: Point::new(0.69407225f32, 0.19487014f32),
        control_1: Point::new(0.6810591f32, 0.22395186f32),
        anchor_1: Point::new(0.6643566f32, 0.25091633f32),
    },
    Cubic {
        anchor_0: Point::new(0.6643566f32, 0.25091633f32),
        control_0: Point::new(0.65382546f32, 0.26791757f32),
        control_1: Point::new(0.64329445f32, 0.28491884f32),
        anchor_1: Point::new(0.6327633f32, 0.30192006f32),
    },
    Cubic {
        anchor_0: Point::new(0.6327633f32, 0.30192006f32),
        control_0: Point::new(0.64448684f32, 0.2857179f32),
        control_1: Point::new(0.6562104f32, 0.26951578f32),
        anchor_1: Point::new(0.6679339f32, 0.25331366f32),
    },
    Cubic {
        anchor_0: Point::new(0.6679339f32, 0.25331366f32),
        control_0: Point::new(0.68652755f32, 0.22761692f32),
        control_1: Point::new(0.7084817f32, 0.20452817f32),
        anchor_1: Point::new(0.7332099f32, 0.18466452f32),
    },
    Cubic {
        anchor_0: Point::new(0.7332099f32, 0.18466452f32),
        control_0: Point::new(0.7424052f32, 0.17727812f32),
        control_1: Point::new(0.7516005f32, 0.16989173f32),
        anchor_1: Point::new(0.7607957f32, 0.16250531f32),
    },
    Cubic {
        anchor_0: Point::new(0.7607957f32, 0.16250531f32),
        control_0: Point::new(0.7751434f32, 0.15098013f32),
        control_1: Point::new(0.79178774f32, 0.14265329f32),
        anchor_1: Point::new(0.8096145f32, 0.13808234f32),
    },
    Cubic {
        anchor_0: Point::new(0.8096145f32, 0.13808234f32),
        control_0: Point::new(0.8116939f32, 0.13754913f32),
        control_1: Point::new(0.81377345f32, 0.13701592f32),
        anchor_1: Point::new(0.81585294f32, 0.13648273f32),
    },
    Cubic {
        anchor_0: Point::new(0.81585294f32, 0.13648273f32),
        control_0: Point::new(0.829542f32, 0.13297272f32),
        control_1: Point::new(0.84406674f32, 0.13695994f32),
        anchor_1: Point::new(0.85404515f32, 0.146967f32),
    },
    Cubic {
        anchor_0: Point::new(0.85404515f32, 0.146967f32),
        control_0: Point::new(0.86402357f32, 0.15697403f32),
        control_1: Point::new(0.86796916f32, 0.17151009f32),
        anchor_1: Point::new(0.86441994f32, 0.185189f32),
    },
    Cubic {
        anchor_0: Point::new(0.86441994f32, 0.185189f32),
        control_0: Point::new(0.8638808f32, 0.18726698f32),
        control_1: Point::new(0.8633417f32, 0.18934496f32),
        anchor_1: Point::new(0.8628025f32, 0.19142291f32),
    },
    Cubic {
        anchor_0: Point::new(0.8628025f32, 0.19142291f32),
        control_0: Point::new(0.8581805f32, 0.20923647f32),
        control_1: Point::new(0.8498062f32, 0.22585686f32),
        anchor_1: Point::new(0.83823997f32, 0.24017146f32),
    },
    Cubic {
        anchor_0: Point::new(0.83823997f32, 0.24017146f32),
        control_0: Point::new(0.83082736f32, 0.24934548f32),
        control_1: Point::new(0.8234148f32, 0.25851953f32),
        anchor_1: Point::new(0.81600213f32, 0.26769352f32),
    },
    Cubic {
        anchor_0: Point::new(0.81600213f32, 0.26769352f32),
        control_0: Point::new(0.7960675f32, 0.29236513f32),
        control_1: Point::new(0.77291614f32, 0.31425312f32),
        anchor_1: Point::new(0.7471658f32, 0.33277336f32),
    },
    Cubic {
        anchor_0: Point::new(0.7471658f32, 0.33277336f32),
        control_0: Point::new(0.73093027f32, 0.34445032f32),
        control_1: Point::new(0.7146948f32, 0.3561273f32),
        anchor_1: Point::new(0.6984592f32, 0.3678043f32),
    },
    Cubic {
        anchor_0: Point::new(0.6984592f32, 0.3678043f32),
        control_0: Point::new(0.7154904f32, 0.35732192f32),
        control_1: Point::new(0.73252165f32, 0.34683964f32),
        anchor_1: Point::new(0.74955285f32, 0.33635727f32),
    },
    Cubic {
        anchor_0: Point::new(0.74955285f32, 0.33635727f32),
        control_0: Point::new(0.7765656f32, 0.31973153f32),
        control_1: Point::new(0.80568373f32, 0.30680215f32),
        anchor_1: Point::new(0.836132f32, 0.29791334f32),
    },
    Cubic {
        anchor_0: Point::new(0.836132f32, 0.29791334f32),
        control_0: Point::new(0.8474539f32, 0.29460815f32),
        control_1: Point::new(0.8587758f32, 0.29130292f32),
        anchor_1: Point::new(0.87009776f32, 0.28799766f32),
    },
    Cubic {
        anchor_0: Point::new(0.87009776f32, 0.28799766f32),
        control_0: Point::new(0.8877636f32, 0.28284046f32),
        control_1: Point::new(0.90632737f32, 0.281517f32),
        anchor_1: Point::new(0.9245461f32, 0.2841159f32),
    },
    Cubic {
        anchor_0: Point::new(0.9245461f32, 0.2841159f32),
        control_0: Point::new(0.9266714f32, 0.28441906f32),
        control_1: Point::new(0.92879665f32, 0.2847222f32),
        anchor_1: Point::new(0.9309219f32, 0.2850254f32),
    },
    Cubic {
        anchor_0: Point::new(0.9309219f32, 0.2850254f32),
        control_0: Point::new(0.9449122f32, 0.28702107f32),
        control_1: Point::new(0.95680547f32, 0.29626322f32),
        anchor_1: Point::new(0.96219486f32, 0.30932716f32),
    },
    Cubic {
        anchor_0: Point::new(0.96219486f32, 0.30932716f32),
        control_0: Point::new(0.96758425f32, 0.3223911f32),
        control_1: Point::new(0.9656668f32, 0.33733064f32),
        anchor_1: Point::new(0.957153f32, 0.3486102f32),
    },
    Cubic {
        anchor_0: Point::new(0.957153f32, 0.3486102f32),
        control_0: Point::new(0.95585966f32, 0.35032365f32),
        control_1: Point::new(0.9545663f32, 0.35203716f32),
        anchor_1: Point::new(0.953273f32, 0.35375065f32),
    },
    Cubic {
        anchor_0: Point::new(0.953273f32, 0.35375065f32),
        control_0: Point::new(0.942186f32, 0.36843926f32),
        control_1: Point::new(0.9280887f32, 0.38058963f32),
        anchor_1: Point::new(0.91192526f32, 0.38938832f32),
    },
    Cubic {
        anchor_0: Point::new(0.91192526f32, 0.38938832f32),
        control_0: Point::new(0.9015662f32, 0.3950273f32),
        control_1: Point::new(0.89120716f32, 0.40066636f32),
        anchor_1: Point::new(0.88084817f32, 0.40630534f32),
    },
    Cubic {
        anchor_0: Point::new(0.88084817f32, 0.40630534f32),
        control_0: Point::new(0.85298973f32, 0.42147025f32),
        control_1: Point::new(0.82322323f32, 0.4328329f32),
        anchor_1: Point::new(0.7923458f32, 0.44008908f32),
    },
    Cubic {
        anchor_0: Point::new(0.7923458f32, 0.44008908f32),
        control_0: Point::new(0.77287763f32, 0.44466406f32),
        control_1: Point::new(0.75340956f32, 0.4492391f32),
        anchor_1: Point::new(0.7339415f32, 0.45381412f32),
    },
];
static SOFT_BOOM_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: SOFT_BOOM_CUBICS,
}];
pub const SOFT_BOOM: ShapeDataStored = ShapeDataStored {
    features: SOFT_BOOM_FEATURES,
};

static SOFT_BURST_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.18663658f32, 0.27241567f32),
        control_0: Point::new(0.19362366f32, 0.26275796f32),
        control_1: Point::new(0.19729336f32, 0.25070024f32),
        anchor_1: Point::new(0.19632591f32, 0.23806655f32),
    },
    Cubic {
        anchor_0: Point::new(0.19632591f32, 0.23806655f32),
        control_0: Point::new(0.19403367f32, 0.20813267f32),
        control_1: Point::new(0.19174144f32, 0.1781988f32),
        anchor_1: Point::new(0.1894492f32, 0.14826494f32),
    },
    Cubic {
        anchor_0: Point::new(0.1894492f32, 0.14826494f32),
        control_0: Point::new(0.18650752f32, 0.10985015f32),
        control_1: Point::new(0.22500725f32, 0.08175997f32),
        anchor_1: Point::new(0.26069158f32, 0.09628506f32),
    },
    Cubic {
        anchor_0: Point::new(0.26069158f32, 0.09628506f32),
        control_0: Point::new(0.28865668f32, 0.10766809f32),
        control_1: Point::new(0.31662178f32, 0.11905111f32),
        anchor_1: Point::new(0.3445869f32, 0.13043413f32),
    },
    Cubic {
        anchor_0: Point::new(0.3445869f32, 0.13043413f32),
        control_0: Point::new(0.3680583f32, 0.13998803f32),
        control_1: Point::new(0.3949967f32, 0.13117528f32),
        anchor_1: Point::new(0.40828314f32, 0.10959624f32),
    },
    Cubic {
        anchor_0: Point::new(0.40828314f32, 0.10959624f32),
        control_0: Point::new(0.42402336f32, 0.084031865f32),
        control_1: Point::new(0.4397636f32, 0.05846751f32),
        anchor_1: Point::new(0.45550385f32, 0.032903142f32),
    },
    Cubic {
        anchor_0: Point::new(0.45550385f32, 0.032903142f32),
        control_0: Point::new(0.47570363f32, 9.586556E-5f32),
        control_1: Point::new(0.52336156f32, 0.0f32),
        anchor_1: Point::new(0.5436931f32, 0.03272575f32),
    },
    Cubic {
        anchor_0: Point::new(0.5436931f32, 0.03272575f32),
        control_0: Point::new(0.5596266f32, 0.058372304f32),
        control_1: Point::new(0.5755601f32, 0.08401886f32),
        anchor_1: Point::new(0.59149355f32, 0.10966539f32),
    },
    Cubic {
        anchor_0: Point::new(0.59149355f32, 0.10966539f32),
        control_0: Point::new(0.6048667f32, 0.13119082f32),
        control_1: Point::new(0.63184035f32, 0.13989513f32),
        anchor_1: Point::new(0.65527314f32, 0.13024688f32),
    },
    Cubic {
        anchor_0: Point::new(0.65527314f32, 0.13024688f32),
        control_0: Point::new(0.6830335f32, 0.11881676f32),
        control_1: Point::new(0.71079403f32, 0.10738664f32),
        anchor_1: Point::new(0.7385545f32, 0.09595652f32),
    },
    Cubic {
        anchor_0: Point::new(0.7385545f32, 0.09595652f32),
        control_0: Point::new(0.77418005f32, 0.08128801f32),
        control_1: Point::new(0.81279254f32, 0.10922307f32),
        anchor_1: Point::new(0.8100054f32, 0.14764938f32),
    },
    Cubic {
        anchor_0: Point::new(0.8100054f32, 0.14764938f32),
        control_0: Point::new(0.8078212f32, 0.17776333f32),
        control_1: Point::new(0.80563706f32, 0.20787731f32),
        anchor_1: Point::new(0.8034528f32, 0.23799127f32),
    },
    Cubic {
        anchor_0: Point::new(0.8034528f32, 0.23799127f32),
        control_0: Point::new(0.8016196f32, 0.26326624f32),
        control_1: Point::new(0.81832546f32, 0.28616285f32),
        anchor_1: Point::new(0.842954f32, 0.2921307f32),
    },
    Cubic {
        anchor_0: Point::new(0.842954f32, 0.2921307f32),
        control_0: Point::new(0.8721311f32, 0.29920074f32),
        control_1: Point::new(0.9013083f32, 0.30627078f32),
        anchor_1: Point::new(0.9304855f32, 0.31334084f32),
    },
    Cubic {
        anchor_0: Point::new(0.9304855f32, 0.31334084f32),
        control_0: Point::new(0.9679292f32, 0.32241395f32),
        control_1: Point::new(0.9827475f32, 0.36770973f32),
        anchor_1: Point::new(0.9579061f32, 0.397159f32),
    },
    Cubic {
        anchor_0: Point::new(0.9579061f32, 0.397159f32),
        control_0: Point::new(0.93843853f32, 0.42023784f32),
        control_1: Point::new(0.91897094f32, 0.44331673f32),
        anchor_1: Point::new(0.89950335f32, 0.4663956f32),
    },
    Cubic {
        anchor_0: Point::new(0.89950335f32, 0.4663956f32),
        control_0: Point::new(0.883164f32, 0.48576593f32),
        control_1: Point::new(0.88322103f32, 0.51410913f32),
        anchor_1: Point::new(0.8996381f32, 0.5334136f32),
    },
    Cubic {
        anchor_0: Point::new(0.8996381f32, 0.5334136f32),
        control_0: Point::new(0.91908723f32, 0.55628324f32),
        control_1: Point::new(0.9385364f32, 0.57915294f32),
        anchor_1: Point::new(0.9579856f32, 0.6020226f32),
    },
    Cubic {
        anchor_0: Point::new(0.9579856f32, 0.6020226f32),
        control_0: Point::new(0.98294514f32, 0.6313717f32),
        control_1: Point::new(0.96830916f32, 0.67672676f32),
        anchor_1: Point::new(0.93090236f32, 0.68595034f32),
    },
    Cubic {
        anchor_0: Point::new(0.93090236f32, 0.68595034f32),
        control_0: Point::new(0.90158725f32, 0.6931788f32),
        control_1: Point::new(0.87227225f32, 0.7004072f32),
        anchor_1: Point::new(0.8429572f32, 0.70763564f32),
    },
    Cubic {
        anchor_0: Point::new(0.8429572f32, 0.70763564f32),
        control_0: Point::new(0.8183528f32, 0.7137026f32),
        control_1: Point::new(0.80173916f32, 0.73666626f32),
        anchor_1: Point::new(0.80367404f32, 0.76193357f32),
    },
    Cubic {
        anchor_0: Point::new(0.80367404f32, 0.76193357f32),
        control_0: Point::new(0.8059663f32, 0.79186743f32),
        control_1: Point::new(0.80825853f32, 0.82180136f32),
        anchor_1: Point::new(0.8105508f32, 0.85173523f32),
    },
    Cubic {
        anchor_0: Point::new(0.8105508f32, 0.85173523f32),
        control_0: Point::new(0.81349254f32, 0.89015f32),
        control_1: Point::new(0.7749928f32, 0.91824013f32),
        anchor_1: Point::new(0.7393085f32, 0.903715f32),
    },
    Cubic {
        anchor_0: Point::new(0.7393085f32, 0.903715f32),
        control_0: Point::new(0.7113433f32, 0.8923319f32),
        control_1: Point::new(0.68337816f32, 0.8809489f32),
        anchor_1: Point::new(0.655413f32, 0.8695659f32),
    },
    Cubic {
        anchor_0: Point::new(0.655413f32, 0.8695659f32),
        control_0: Point::new(0.63194156f32, 0.86001205f32),
        control_1: Point::new(0.6050031f32, 0.86882484f32),
        anchor_1: Point::new(0.5917167f32, 0.8904039f32),
    },
    Cubic {
        anchor_0: Point::new(0.5917167f32, 0.8904039f32),
        control_0: Point::new(0.5759765f32, 0.9159681f32),
        control_1: Point::new(0.5602363f32, 0.9415325f32),
        anchor_1: Point::new(0.544496f32, 0.9670968f32),
    },
    Cubic {
        anchor_0: Point::new(0.544496f32, 0.9670968f32),
        control_0: Point::new(0.5242962f32, 0.99990416f32),
        control_1: Point::new(0.47663832f32, 1.0f32),
        anchor_1: Point::new(0.45630676f32, 0.9672742f32),
    },
    Cubic {
        anchor_0: Point::new(0.45630676f32, 0.9672742f32),
        control_0: Point::new(0.44037333f32, 0.9416277f32),
        control_1: Point::new(0.42443994f32, 0.9159813f32),
        anchor_1: Point::new(0.4085065f32, 0.8903348f32),
    },
    Cubic {
        anchor_0: Point::new(0.4085065f32, 0.8903348f32),
        control_0: Point::new(0.39513338f32, 0.86880934f32),
        control_1: Point::new(0.3681597f32, 0.860105f32),
        anchor_1: Point::new(0.34472692f32, 0.86975324f32),
    },
    Cubic {
        anchor_0: Point::new(0.34472692f32, 0.86975324f32),
        control_0: Point::new(0.31696644f32, 0.8811833f32),
        control_1: Point::new(0.289206f32, 0.8926134f32),
        anchor_1: Point::new(0.26144555f32, 0.9040437f32),
    },
    Cubic {
        anchor_0: Point::new(0.26144555f32, 0.9040437f32),
        control_0: Point::new(0.22581996f32, 0.9187122f32),
        control_1: Point::new(0.18720755f32, 0.8907771f32),
        anchor_1: Point::new(0.18999465f32, 0.8523508f32),
    },
    Cubic {
        anchor_0: Point::new(0.18999465f32, 0.8523508f32),
        control_0: Point::new(0.19217883f32, 0.82223666f32),
        control_1: Point::new(0.19436304f32, 0.7921227f32),
        anchor_1: Point::new(0.19654724f32, 0.7620087f32),
    },
    Cubic {
        anchor_0: Point::new(0.19654724f32, 0.7620087f32),
        control_0: Point::new(0.19838046f32, 0.7367339f32),
        control_1: Point::new(0.1816746f32, 0.71383715f32),
        anchor_1: Point::new(0.157046f32, 0.70786935f32),
    },
    Cubic {
        anchor_0: Point::new(0.157046f32, 0.70786935f32),
        control_0: Point::new(0.12786885f32, 0.7007993f32),
        control_1: Point::new(0.098691724f32, 0.6937292f32),
        anchor_1: Point::new(0.06951458f32, 0.6866593f32),
    },
    Cubic {
        anchor_0: Point::new(0.06951458f32, 0.6866593f32),
        control_0: Point::new(0.03207089f32, 0.6775862f32),
        control_1: Point::new(0.017252645f32, 0.63229036f32),
        anchor_1: Point::new(0.042093914f32, 0.602841f32),
    },
    Cubic {
        anchor_0: Point::new(0.042093914f32, 0.602841f32),
        control_0: Point::new(0.06156149f32, 0.5797622f32),
        control_1: Point::new(0.08102907f32, 0.55668336f32),
        anchor_1: Point::new(0.10049666f32, 0.5336046f32),
    },
    Cubic {
        anchor_0: Point::new(0.10049666f32, 0.5336046f32),
        control_0: Point::new(0.116836004f32, 0.51423424f32),
        control_1: Point::new(0.11677899f32, 0.485891f32),
        anchor_1: Point::new(0.100361854f32, 0.4665866f32),
    },
    Cubic {
        anchor_0: Point::new(0.100361854f32, 0.4665866f32),
        control_0: Point::new(0.0809127f32, 0.4437169f32),
        control_1: Point::new(0.06146356f32, 0.42084724f32),
        anchor_1: Point::new(0.04201442f32, 0.3979776f32),
    },
    Cubic {
        anchor_0: Point::new(0.04201442f32, 0.3979776f32),
        control_0: Point::new(0.0170549f32, 0.36862844f32),
        control_1: Point::new(0.03169083f32, 0.32327345f32),
        anchor_1: Point::new(0.06909768f32, 0.31404978f32),
    },
    Cubic {
        anchor_0: Point::new(0.06909768f32, 0.31404978f32),
        control_0: Point::new(0.09841272f32, 0.30682132f32),
        control_1: Point::new(0.12772776f32, 0.2995929f32),
        anchor_1: Point::new(0.1570428f32, 0.29236445f32),
    },
    Cubic {
        anchor_0: Point::new(0.1570428f32, 0.29236445f32),
        control_0: Point::new(0.16934499f32, 0.28933102f32),
        control_1: Point::new(0.17964949f32, 0.28207338f32),
        anchor_1: Point::new(0.18663658f32, 0.27241567f32),
    },
];
static SOFT_BURST_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: SOFT_BURST_CUBICS,
}];
pub const SOFT_BURST: ShapeDataStored = ShapeDataStored {
    features: SOFT_BURST_FEATURES,
};

static SQUARE_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.912132f32, 0.912132f32),
        control_0: Point::new(0.85784274f32, 0.96642137f32),
        control_1: Point::new(0.7828427f32, 1.0f32),
        anchor_1: Point::new(0.7f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.7f32, 1.0f32),
        control_0: Point::new(0.56666666f32, 1.0f32),
        control_1: Point::new(0.43333334f32, 1.0f32),
        anchor_1: Point::new(0.3f32, 1.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.3f32, 1.0f32),
        control_0: Point::new(0.1343146f32, 1.0f32),
        control_1: Point::new(0.0f32, 0.8656854f32),
        anchor_1: Point::new(0.0f32, 0.7f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.7f32),
        control_0: Point::new(0.0f32, 0.56666666f32),
        control_1: Point::new(0.0f32, 0.43333334f32),
        anchor_1: Point::new(0.0f32, 0.3f32),
    },
    Cubic {
        anchor_0: Point::new(0.0f32, 0.3f32),
        control_0: Point::new(0.0f32, 0.1343146f32),
        control_1: Point::new(0.1343146f32, 0.0f32),
        anchor_1: Point::new(0.3f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.3f32, 0.0f32),
        control_0: Point::new(0.43333334f32, 0.0f32),
        control_1: Point::new(0.56666666f32, 0.0f32),
        anchor_1: Point::new(0.7f32, 0.0f32),
    },
    Cubic {
        anchor_0: Point::new(0.7f32, 0.0f32),
        control_0: Point::new(0.8656854f32, 0.0f32),
        control_1: Point::new(1.0f32, 0.1343146f32),
        anchor_1: Point::new(1.0f32, 0.3f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.3f32),
        control_0: Point::new(1.0f32, 0.43333334f32),
        control_1: Point::new(1.0f32, 0.56666666f32),
        anchor_1: Point::new(1.0f32, 0.7f32),
    },
    Cubic {
        anchor_0: Point::new(1.0f32, 0.7f32),
        control_0: Point::new(1.0f32, 0.7828427f32),
        control_1: Point::new(0.96642137f32, 0.8578427f32),
        anchor_1: Point::new(0.912132f32, 0.912132f32),
    },
];
static SQUARE_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: SQUARE_CUBICS,
}];
pub const SQUARE: ShapeDataStored = ShapeDataStored {
    features: SQUARE_FEATURES,
};

static SUNNY_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.9969052f32, 0.5f32),
        control_0: Point::new(0.9969052f32, 0.5179457f32),
        control_1: Point::new(0.9907156f32, 0.5358914f32),
        anchor_1: Point::new(0.9783364f32, 0.5504177f32),
    },
    Cubic {
        anchor_0: Point::new(0.9783364f32, 0.5504177f32),
        control_0: Point::new(0.9530324f32, 0.58011043f32),
        control_1: Point::new(0.9277284f32, 0.6098032f32),
        anchor_1: Point::new(0.9024245f32, 0.6394959f32),
    },
    Cubic {
        anchor_0: Point::new(0.9024245f32, 0.6394959f32),
        control_0: Point::new(0.89181495f32, 0.6519456f32),
        control_1: Point::new(0.88540316f32, 0.6674249f32),
        anchor_1: Point::new(0.88410205f32, 0.6837303f32),
    },
    Cubic {
        anchor_0: Point::new(0.88410205f32, 0.6837303f32),
        control_0: Point::new(0.8809987f32, 0.72261876f32),
        control_1: Point::new(0.87789536f32, 0.76150733f32),
        anchor_1: Point::new(0.87479204f32, 0.80039585f32),
    },
    Cubic {
        anchor_0: Point::new(0.87479204f32, 0.80039585f32),
        control_0: Point::new(0.87175566f32, 0.8384459f32),
        control_1: Point::new(0.8415407f32, 0.86866087f32),
        anchor_1: Point::new(0.8034906f32, 0.8716972f32),
    },
    Cubic {
        anchor_0: Point::new(0.8034906f32, 0.8716972f32),
        control_0: Point::new(0.76460207f32, 0.87480056f32),
        control_1: Point::new(0.7257136f32, 0.8779039f32),
        anchor_1: Point::new(0.68682504f32, 0.8810072f32),
    },
    Cubic {
        anchor_0: Point::new(0.68682504f32, 0.8810072f32),
        control_0: Point::new(0.67051977f32, 0.8823084f32),
        control_1: Point::new(0.6550404f32, 0.8887202f32),
        anchor_1: Point::new(0.64259064f32, 0.8993297f32),
    },
    Cubic {
        anchor_0: Point::new(0.64259064f32, 0.8993297f32),
        control_0: Point::new(0.6128979f32, 0.9246337f32),
        control_1: Point::new(0.58320516f32, 0.94993764f32),
        anchor_1: Point::new(0.55351245f32, 0.9752416f32),
    },
    Cubic {
        anchor_0: Point::new(0.55351245f32, 0.9752416f32),
        control_0: Point::new(0.5244599f32, 1.0f32),
        control_1: Point::new(0.48172957f32, 1.0f32),
        anchor_1: Point::new(0.452677f32, 0.9752416f32),
    },
    Cubic {
        anchor_0: Point::new(0.452677f32, 0.9752416f32),
        control_0: Point::new(0.4229843f32, 0.94993764f32),
        control_1: Point::new(0.39329156f32, 0.9246337f32),
        anchor_1: Point::new(0.36359888f32, 0.8993297f32),
    },
    Cubic {
        anchor_0: Point::new(0.36359888f32, 0.8993297f32),
        control_0: Point::new(0.35114917f32, 0.8887202f32),
        control_1: Point::new(0.33566982f32, 0.88230836f32),
        anchor_1: Point::new(0.31936446f32, 0.8810072f32),
    },
    Cubic {
        anchor_0: Point::new(0.31936446f32, 0.8810072f32),
        control_0: Point::new(0.28047597f32, 0.8779039f32),
        control_1: Point::new(0.24158747f32, 0.87480056f32),
        anchor_1: Point::new(0.20269896f32, 0.8716972f32),
    },
    Cubic {
        anchor_0: Point::new(0.20269896f32, 0.8716972f32),
        control_0: Point::new(0.16464886f32, 0.86866087f32),
        control_1: Point::new(0.13443397f32, 0.83844596f32),
        anchor_1: Point::new(0.1313975f32, 0.8003959f32),
    },
    Cubic {
        anchor_0: Point::new(0.1313975f32, 0.8003959f32),
        control_0: Point::new(0.1282942f32, 0.76150733f32),
        control_1: Point::new(0.12519082f32, 0.72261876f32),
        anchor_1: Point::new(0.122087464f32, 0.68373024f32),
    },
    Cubic {
        anchor_0: Point::new(0.122087464f32, 0.68373024f32),
        control_0: Point::new(0.120786294f32, 0.66742486f32),
        control_1: Point::new(0.11437453f32, 0.65194553f32),
        anchor_1: Point::new(0.103764996f32, 0.6394958f32),
    },
    Cubic {
        anchor_0: Point::new(0.103764996f32, 0.6394958f32),
        control_0: Point::new(0.07846107f32, 0.6098031f32),
        control_1: Point::new(0.05315712f32, 0.5801104f32),
        anchor_1: Point::new(0.027853196f32, 0.55041766f32),
    },
    Cubic {
        anchor_0: Point::new(0.027853196f32, 0.55041766f32),
        control_0: Point::new(0.003094779f32, 0.52136517f32),
        control_1: Point::new(0.003094779f32, 0.47863477f32),
        anchor_1: Point::new(0.027853228f32, 0.44958225f32),
    },
    Cubic {
        anchor_0: Point::new(0.027853228f32, 0.44958225f32),
        control_0: Point::new(0.05315718f32, 0.41988954f32),
        control_1: Point::new(0.07846113f32, 0.39019683f32),
        anchor_1: Point::new(0.103765056f32, 0.36050412f32),
    },
    Cubic {
        anchor_0: Point::new(0.103765056f32, 0.36050412f32),
        control_0: Point::new(0.11437462f32, 0.34805444f32),
        control_1: Point::new(0.12078636f32, 0.33257508f32),
        anchor_1: Point::new(0.12208756f32, 0.31626973f32),
    },
    Cubic {
        anchor_0: Point::new(0.12208756f32, 0.31626973f32),
        control_0: Point::new(0.1251909f32, 0.27738118f32),
        control_1: Point::new(0.12829423f32, 0.23849264f32),
        anchor_1: Point::new(0.13139759f32, 0.1996041f32),
    },
    Cubic {
        anchor_0: Point::new(0.13139759f32, 0.1996041f32),
        control_0: Point::new(0.13443403f32, 0.16155401f32),
        control_1: Point::new(0.16464892f32, 0.13133912f32),
        anchor_1: Point::new(0.20269899f32, 0.1283027f32),
    },
    Cubic {
        anchor_0: Point::new(0.20269899f32, 0.1283027f32),
        control_0: Point::new(0.24158753f32, 0.12519936f32),
        control_1: Point::new(0.2804761f32, 0.122096024f32),
        anchor_1: Point::new(0.3193646f32, 0.118992716f32),
    },
    Cubic {
        anchor_0: Point::new(0.3193646f32, 0.118992716f32),
        control_0: Point::new(0.33567f32, 0.11769155f32),
        control_1: Point::new(0.35114932f32, 0.111279756f32),
        anchor_1: Point::new(0.36359903f32, 0.10067019f32),
    },
    Cubic {
        anchor_0: Point::new(0.36359903f32, 0.10067019f32),
        control_0: Point::new(0.39329174f32, 0.075366296f32),
        control_1: Point::new(0.42298436f32, 0.05006234f32),
        anchor_1: Point::new(0.45267704f32, 0.024758449f32),
    },
    Cubic {
        anchor_0: Point::new(0.45267704f32, 0.024758449f32),
        control_0: Point::new(0.4817296f32, 0.0f32),
        control_1: Point::new(0.52445996f32, 0.0f32),
        anchor_1: Point::new(0.5535125f32, 0.024758417f32),
    },
    Cubic {
        anchor_0: Point::new(0.5535125f32, 0.024758417f32),
        control_0: Point::new(0.5832052f32, 0.050062403f32),
        control_1: Point::new(0.612898f32, 0.075366355f32),
        anchor_1: Point::new(0.6425907f32, 0.10067031f32),
    },
    Cubic {
        anchor_0: Point::new(0.6425907f32, 0.10067031f32),
        control_0: Point::new(0.65504044f32, 0.111279845f32),
        control_1: Point::new(0.67051977f32, 0.11769164f32),
        anchor_1: Point::new(0.6868251f32, 0.11899281f32),
    },
    Cubic {
        anchor_0: Point::new(0.6868251f32, 0.11899281f32),
        control_0: Point::new(0.7257137f32, 0.12209617f32),
        control_1: Point::new(0.7646022f32, 0.12519951f32),
        anchor_1: Point::new(0.8034907f32, 0.12830281f32),
    },
    Cubic {
        anchor_0: Point::new(0.8034907f32, 0.12830281f32),
        control_0: Point::new(0.8415408f32, 0.13133925f32),
        control_1: Point::new(0.8717558f32, 0.16155417f32),
        anchor_1: Point::new(0.87479216f32, 0.1996043f32),
    },
    Cubic {
        anchor_0: Point::new(0.87479216f32, 0.1996043f32),
        control_0: Point::new(0.8778954f32, 0.23849279f32),
        control_1: Point::new(0.88099873f32, 0.27738124f32),
        anchor_1: Point::new(0.88410205f32, 0.3162697f32),
    },
    Cubic {
        anchor_0: Point::new(0.88410205f32, 0.3162697f32),
        control_0: Point::new(0.8854032f32, 0.33257505f32),
        control_1: Point::new(0.89181495f32, 0.34805444f32),
        anchor_1: Point::new(0.9024245f32, 0.36050415f32),
    },
    Cubic {
        anchor_0: Point::new(0.9024245f32, 0.36050415f32),
        control_0: Point::new(0.9277285f32, 0.39019686f32),
        control_1: Point::new(0.95303243f32, 0.41988957f32),
        anchor_1: Point::new(0.9783364f32, 0.44958228f32),
    },
    Cubic {
        anchor_0: Point::new(0.9783364f32, 0.44958228f32),
        control_0: Point::new(0.9907156f32, 0.46410856f32),
        control_1: Point::new(0.9969052f32, 0.4820543f32),
        anchor_1: Point::new(0.9969052f32, 0.5f32),
    },
];
static SUNNY_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: SUNNY_CUBICS,
}];
pub const SUNNY: ShapeDataStored = ShapeDataStored {
    features: SUNNY_FEATURES,
};

static TRIANGLE_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5f32, 0.077812634f32),
        control_0: Point::new(0.54375f32, 0.07781265f32),
        control_1: Point::new(0.5875f32, 0.0994633f32),
        anchor_1: Point::new(0.6125f32, 0.14276457f32),
    },
    Cubic {
        anchor_0: Point::new(0.6125f32, 0.14276457f32),
        control_0: Point::new(0.725f32, 0.3376203f32),
        control_1: Point::new(0.83750004f32, 0.532476f32),
        anchor_1: Point::new(0.95000005f32, 0.7273317f32),
    },
    Cubic {
        anchor_0: Point::new(0.95000005f32, 0.7273317f32),
        control_0: Point::new(1.0f32, 0.8139343f32),
        control_1: Point::new(0.9375f32, 0.9221874f32),
        anchor_1: Point::new(0.8375f32, 0.9221874f32),
    },
    Cubic {
        anchor_0: Point::new(0.8375f32, 0.9221874f32),
        control_0: Point::new(0.61249995f32, 0.9221874f32),
        control_1: Point::new(0.38749996f32, 0.9221874f32),
        anchor_1: Point::new(0.16249995f32, 0.9221874f32),
    },
    Cubic {
        anchor_0: Point::new(0.16249995f32, 0.9221874f32),
        control_0: Point::new(0.06249995f32, 0.9221874f32),
        control_1: Point::new(0.0f32, 0.81393415f32),
        anchor_1: Point::new(0.04999998f32, 0.7273317f32),
    },
    Cubic {
        anchor_0: Point::new(0.04999998f32, 0.7273317f32),
        control_0: Point::new(0.16249998f32, 0.53247595f32),
        control_1: Point::new(0.27499998f32, 0.33762023f32),
        anchor_1: Point::new(0.3875f32, 0.1427645f32),
    },
    Cubic {
        anchor_0: Point::new(0.3875f32, 0.1427645f32),
        control_0: Point::new(0.4125f32, 0.09946324f32),
        control_1: Point::new(0.45625f32, 0.07781262f32),
        anchor_1: Point::new(0.5f32, 0.077812634f32),
    },
];
static TRIANGLE_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: TRIANGLE_CUBICS,
}];
pub const TRIANGLE: ShapeDataStored = ShapeDataStored {
    features: TRIANGLE_FEATURES,
};

static VERY_SUNNY_CUBICS: &[Cubic] = &[
    Cubic {
        anchor_0: Point::new(0.5000118f32, 0.9932915f32),
        control_0: Point::new(0.47254747f32, 0.9932873f32),
        control_1: Point::new(0.44508523f32, 0.97987604f32),
        anchor_1: Point::new(0.4290178f32, 0.95305926f32),
    },
    Cubic {
        anchor_0: Point::new(0.4290178f32, 0.95305926f32),
        control_0: Point::new(0.41710147f32, 0.93317074f32),
        control_1: Point::new(0.40518516f32, 0.9132823f32),
        anchor_1: Point::new(0.39326876f32, 0.89339364f32),
    },
    Cubic {
        anchor_0: Point::new(0.39326876f32, 0.89339364f32),
        control_0: Point::new(0.3745289f32, 0.8621165f32),
        control_1: Point::new(0.3375305f32, 0.8467848f32),
        anchor_1: Point::new(0.30216053f32, 0.8556394f32),
    },
    Cubic {
        anchor_0: Point::new(0.30216053f32, 0.8556394f32),
        control_0: Point::new(0.2796342f32, 0.8612786f32),
        control_1: Point::new(0.25710788f32, 0.86691785f32),
        anchor_1: Point::new(0.23458159f32, 0.87255716f32),
    },
    Cubic {
        anchor_0: Point::new(0.23458159f32, 0.87255716f32),
        control_0: Point::new(0.17392957f32, 0.88774085f32),
        control_1: Point::new(0.11898567f32, 0.8327805f32),
        anchor_1: Point::new(0.13418755f32, 0.772133f32),
    },
    Cubic {
        anchor_0: Point::new(0.13418755f32, 0.772133f32),
        control_0: Point::new(0.13982475f32, 0.7496435f32),
        control_1: Point::new(0.14546195f32, 0.72715414f32),
        anchor_1: Point::new(0.15109915f32, 0.70466465f32),
    },
    Cubic {
        anchor_0: Point::new(0.15109915f32, 0.70466465f32),
        control_0: Point::new(0.15996431f32, 0.66929734f32),
        control_1: Point::new(0.14464365f32, 0.6322943f32),
        anchor_1: Point::new(0.11337218f32, 0.61354506f32),
    },
    Cubic {
        anchor_0: Point::new(0.11337218f32, 0.61354506f32),
        control_0: Point::new(0.09345609f32, 0.60160404f32),
        control_1: Point::new(0.073540024f32, 0.58966315f32),
        anchor_1: Point::new(0.053623952f32, 0.5777222f32),
    },
    Cubic {
        anchor_0: Point::new(0.053623952f32, 0.5777222f32),
        control_0: Point::new(0.0f32, 0.5455712f32),
        control_1: Point::new(1.1638057E-5f32, 0.46785715f32),
        anchor_1: Point::new(0.053645235f32, 0.4357223f32),
    },
    Cubic {
        anchor_0: Point::new(0.053645235f32, 0.4357223f32),
        control_0: Point::new(0.07353379f32, 0.42380592f32),
        control_1: Point::new(0.09342234f32, 0.41188958f32),
        anchor_1: Point::new(0.11331088f32, 0.39997324f32),
    },
    Cubic {
        anchor_0: Point::new(0.11331088f32, 0.39997324f32),
        control_0: Point::new(0.14458798f32, 0.3812334f32),
        control_1: Point::new(0.15991972f32, 0.34423497f32),
        anchor_1: Point::new(0.15106517f32, 0.30886498f32),
    },
    Cubic {
        anchor_0: Point::new(0.15106517f32, 0.30886498f32),
        control_0: Point::new(0.1454259f32, 0.2863387f32),
        control_1: Point::new(0.13978665f32, 0.26381236f32),
        anchor_1: Point::new(0.13414738f32, 0.24128608f32),
    },
    Cubic {
        anchor_0: Point::new(0.13414738f32, 0.24128608f32),
        control_0: Point::new(0.118963696f32, 0.18063405f32),
        control_1: Point::new(0.1739241f32, 0.12569015f32),
        anchor_1: Point::new(0.23457156f32, 0.14089203f32),
    },
    Cubic {
        anchor_0: Point::new(0.23457156f32, 0.14089203f32),
        control_0: Point::new(0.25706097f32, 0.14652923f32),
        control_1: Point::new(0.27955043f32, 0.15216643f32),
        anchor_1: Point::new(0.30203986f32, 0.15780361f32),
    },
    Cubic {
        anchor_0: Point::new(0.30203986f32, 0.15780361f32),
        control_0: Point::new(0.3374072f32, 0.16666879f32),
        control_1: Point::new(0.3744102f32, 0.15134814f32),
        anchor_1: Point::new(0.39315948f32, 0.12007666f32),
    },
    Cubic {
        anchor_0: Point::new(0.39315948f32, 0.12007666f32),
        control_0: Point::new(0.4051004f32, 0.10016058f32),
        control_1: Point::new(0.41704142f32, 0.080244504f32),
        anchor_1: Point::new(0.42898235f32, 0.060328428f32),
    },
    Cubic {
        anchor_0: Point::new(0.42898235f32, 0.060328428f32),
        control_0: Point::new(0.46113333f32, 0.0067044497f32),
        control_1: Point::new(0.5388475f32, 0.0067161093f32),
        anchor_1: Point::new(0.57098234f32, 0.060349725f32),
    },
    Cubic {
        anchor_0: Point::new(0.57098234f32, 0.060349725f32),
        control_0: Point::new(0.5828987f32, 0.08023827f32),
        control_1: Point::new(0.594815f32, 0.10012682f32),
        anchor_1: Point::new(0.6067313f32, 0.12001535f32),
    },
    Cubic {
        anchor_0: Point::new(0.6067313f32, 0.12001535f32),
        control_0: Point::new(0.6254711f32, 0.15129244f32),
        control_1: Point::new(0.66246957f32, 0.1666242f32),
        anchor_1: Point::new(0.6978396f32, 0.15776965f32),
    },
    Cubic {
        anchor_0: Point::new(0.6978396f32, 0.15776965f32),
        control_0: Point::new(0.7203659f32, 0.15213038f32),
        control_1: Point::new(0.74289227f32, 0.14649116f32),
        anchor_1: Point::new(0.7654186f32, 0.14085189f32),
    },
    Cubic {
        anchor_0: Point::new(0.7654186f32, 0.14085189f32),
        control_0: Point::new(0.8260706f32, 0.1256682f32),
        control_1: Point::new(0.8810145f32, 0.18062861f32),
        anchor_1: Point::new(0.86581254f32, 0.24127607f32),
    },
    Cubic {
        anchor_0: Point::new(0.86581254f32, 0.24127607f32),
        control_0: Point::new(0.8601753f32, 0.2637655f32),
        control_1: Point::new(0.85453814f32, 0.28625494f32),
        anchor_1: Point::new(0.8489009f32, 0.30874437f32),
    },
    Cubic {
        anchor_0: Point::new(0.8489009f32, 0.30874437f32),
        control_0: Point::new(0.84003574f32, 0.34411174f32),
        control_1: Point::new(0.85535645f32, 0.38111478f32),
        anchor_1: Point::new(0.886628f32, 0.39986402f32),
    },
    Cubic {
        anchor_0: Point::new(0.886628f32, 0.39986402f32),
        control_0: Point::new(0.90654397f32, 0.4118049f32),
        control_1: Point::new(0.92646f32, 0.42374587f32),
        anchor_1: Point::new(0.9463761f32, 0.43568683f32),
    },
    Cubic {
        anchor_0: Point::new(0.9463761f32, 0.43568683f32),
        control_0: Point::new(1.0f32, 0.46783778f32),
        control_1: Point::new(0.9999884f32, 0.5455519f32),
        anchor_1: Point::new(0.94635475f32, 0.5776867f32),
    },
    Cubic {
        anchor_0: Point::new(0.94635475f32, 0.5776867f32),
        control_0: Point::new(0.9264662f32, 0.58960307f32),
        control_1: Point::new(0.90657777f32, 0.60151947f32),
        anchor_1: Point::new(0.8866892f32, 0.61343575f32),
    },
    Cubic {
        anchor_0: Point::new(0.8866892f32, 0.61343575f32),
        control_0: Point::new(0.8554121f32, 0.63217556f32),
        control_1: Point::new(0.8400804f32, 0.669174f32),
        anchor_1: Point::new(0.8489349f32, 0.70454395f32),
    },
    Cubic {
        anchor_0: Point::new(0.8489349f32, 0.70454395f32),
        control_0: Point::new(0.85457414f32, 0.72707033f32),
        control_1: Point::new(0.8602134f32, 0.7495967f32),
        anchor_1: Point::new(0.86585265f32, 0.77212316f32),
    },
    Cubic {
        anchor_0: Point::new(0.86585265f32, 0.77212316f32),
        control_0: Point::new(0.8810363f32, 0.8327752f32),
        control_1: Point::new(0.8260759f32, 0.88771904f32),
        anchor_1: Point::new(0.7654284f32, 0.8725171f32),
    },
    Cubic {
        anchor_0: Point::new(0.7654284f32, 0.8725171f32),
        control_0: Point::new(0.74293894f32, 0.8668799f32),
        control_1: Point::new(0.7204495f32, 0.86124265f32),
        anchor_1: Point::new(0.6979601f32, 0.8556054f32),
    },
    Cubic {
        anchor_0: Point::new(0.6979601f32, 0.8556054f32),
        control_0: Point::new(0.66259277f32, 0.84674025f32),
        control_1: Point::new(0.6255897f32, 0.86206096f32),
        anchor_1: Point::new(0.6068405f32, 0.8933325f32),
    },
    Cubic {
        anchor_0: Point::new(0.6068405f32, 0.8933325f32),
        control_0: Point::new(0.59489954f32, 0.9132485f32),
        control_1: Point::new(0.5829587f32, 0.93316454f32),
        anchor_1: Point::new(0.57101774f32, 0.95308053f32),
    },
    Cubic {
        anchor_0: Point::new(0.57101774f32, 0.95308053f32),
        control_0: Point::new(0.55494225f32, 0.9798925f32),
        control_1: Point::new(0.527476f32, 0.99329555f32),
        anchor_1: Point::new(0.5000118f32, 0.9932915f32),
    },
];
static VERY_SUNNY_FEATURES: &[FeatureStored] = &[FeatureStored {
    kind: FeatureKind::Edge,
    cubics: VERY_SUNNY_CUBICS,
}];
pub const VERY_SUNNY: ShapeDataStored = ShapeDataStored {
    features: VERY_SUNNY_FEATURES,
};
