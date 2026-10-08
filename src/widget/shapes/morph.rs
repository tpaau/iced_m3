use std::time::Instant;

use crate::{
    animation::{
        Interpolable,
        motion::{SimulatedSpring, Spring},
    },
    widget::shapes::values::{Cubic, Feature, FeatureKind, ShapeData},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FeatureMorphPlan {
    kind: FeatureKind,
    interpolation: Vec<(Cubic, Cubic)>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MorphPlan {
    features: Vec<FeatureMorphPlan>,
}

#[derive(Debug)]
pub enum MorphPlanError {
    FeatureCount {
        from: usize,
        to: usize,
    },
    FeatureKind {
        index: usize,
    },
    CubicCount {
        feature: usize,
        from: usize,
        to: usize,
    },
}

impl MorphPlan {
    pub fn new(from: &ShapeData, to: &ShapeData) -> Result<Self, MorphPlanError> {
        if from.features.len() != to.features.len() {
            return Err(MorphPlanError::FeatureCount {
                from: from.features.len(),
                to: to.features.len(),
            });
        }

        let mut features = Vec::with_capacity(from.features.len());

        for (index, (a, b)) in from.features.iter().zip(&to.features).enumerate() {
            if a.kind != b.kind {
                return Err(MorphPlanError::FeatureKind { index });
            }

            if a.cubics.len() != b.cubics.len() {
                return Err(MorphPlanError::CubicCount {
                    feature: index,
                    from: a.cubics.len(),
                    to: b.cubics.len(),
                });
            }

            features.push(FeatureMorphPlan {
                kind: a.kind,
                interpolation: a
                    .cubics
                    .iter()
                    .copied()
                    .zip(b.cubics.iter().copied())
                    .collect(),
            });
        }

        Ok(Self { features })
    }

    pub fn sample(&self, t: f32) -> ShapeData {
        ShapeData {
            features: self
                .features
                .iter()
                .map(|feature| Feature {
                    kind: feature.kind,
                    cubics: feature
                        .interpolation
                        .iter()
                        .map(|(a, b)| (*a).interpolate(*b, t))
                        .collect(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MorphTransition {
    pub spring: SimulatedSpring,
    pub plan: MorphPlan,
}

impl MorphTransition {
    pub fn new(spring: Spring, plan: MorphPlan, now: Instant) -> Self {
        Self {
            spring: SimulatedSpring::new(spring, now),
            plan,
        }
    }

    pub fn value(&self) -> ShapeData {
        self.plan.sample(self.spring.position)
    }

    pub fn step(&mut self, now: Instant) {
        self.spring.step(now);
    }

    pub fn set_target(&mut self, target: MorphPlan, now: Instant) {
        if self.plan == target {
            return;
        }
        self.plan = target;

        self.spring.reset(now);
    }

    pub fn is_at_rest(&self) -> bool {
        self.spring.is_at_rest()
    }
}
