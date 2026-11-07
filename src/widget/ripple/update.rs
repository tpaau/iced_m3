use std::time::Instant;

use iced::{Point, Size};

use super::{
    constant::*,
    message::Message,
    model::{Model, Phase, Pointer, Press},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    pub size: Size,
    pub enabled: bool,
}

/// What the caller has to do after an update.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effect {
    Idle,
    Redraw,
    WakeAt(Instant),
}

#[must_use]
pub fn update(model: Model, message: Message, context: Context, now: Instant) -> (Model, Effect) {
    if !context.enabled {
        return (Model::default(), Effect::Idle);
    }

    let mut model = model;
    match message {
        Message::Hover(hovered) => {
            model.hovered = hovered;
            let target = if hovered { 1.0 } else { 0.0 };
            model.hover = model.hover.retarget(target, HOVER_FADE, now);
            // `pointerleave` ends the press, but only for the pointer that owns it.
            if !hovered
                && model.phase != Phase::Inactive
                && !matches!(model.pointer, Some(Pointer::Touch(_)))
            {
                end_press(&mut model, now);
            }
        }
        Message::Press { pointer, at } => {
            if model.pointer.is_none() {
                model.pointer = Some(pointer);
                match pointer {
                    Pointer::Mouse => {
                        model.phase = Phase::Holding;
                        start_press(&mut model, Some(at), context.size, now);
                    }
                    Pointer::Touch(_) => {
                        model.phase = Phase::TouchDelay {
                            deadline: now + TOUCH_DELAY,
                        };
                        model.touch_origin = Some(at);
                    }
                }
            }
        }
        Message::Release { pointer } => {
            if model.pointer == Some(pointer) {
                match model.phase {
                    Phase::Holding => model.phase = Phase::WaitingForClick,
                    Phase::TouchDelay { .. } => {
                        model.phase = Phase::WaitingForClick;
                        let origin = model.touch_origin;
                        start_press(&mut model, origin, context.size, now);
                    }
                    _ => {}
                }
            }
        }
        Message::Click => match model.phase {
            Phase::WaitingForClick => end_press(&mut model, now),
            Phase::Inactive => {
                start_press(&mut model, None, context.size, now);
                end_press(&mut model, now);
            }
            _ => {}
        },
        Message::Cancel { pointer } => {
            if model.pointer == Some(pointer) {
                end_press(&mut model, now);
            }
        }
        Message::ContextMenu => end_press(&mut model, now),
        Message::Tick => {
            if let Phase::TouchDelay { deadline } = model.phase
                && now >= deadline
            {
                model.phase = Phase::Holding;
                let origin = model.touch_origin;
                start_press(&mut model, origin, context.size, now);
            }
            if let Some(press) = &mut model.press
                && press.pressed
                && press.release_at.is_some_and(|at| now >= at)
            {
                press.pressed = false;
                press.release_at = None;
                model.fade = model.fade.retarget(0.0, PRESS_FADE_OUT, now);
            }
        }
    }

    let effect = effect_of(&model, now);
    (model, effect)
}

fn start_press(model: &mut Model, origin: Option<Point>, size: Size, now: Instant) {
    model.press = Some(Press::new(origin, size, now));
    model.fade = model.fade.retarget(1.0, PRESS_FADE_IN, now);
}

fn end_press(model: &mut Model, now: Instant) {
    model.pointer = None;
    model.touch_origin = None;
    model.phase = Phase::Inactive;

    let Some(press) = &mut model.press else {
        return;
    };
    if !press.pressed {
        return;
    }
    let release_at = press.started_at + MINIMUM_PRESS;
    if now >= release_at {
        press.pressed = false;
        press.release_at = None;
        model.fade = model.fade.retarget(0.0, PRESS_FADE_OUT, now);
    } else {
        press.release_at = Some(release_at);
    }
}

fn effect_of(model: &Model, now: Instant) -> Effect {
    if model.is_animating(now) {
        Effect::Redraw
    } else if let Some(deadline) = model.next_deadline() {
        Effect::WakeAt(deadline)
    } else {
        Effect::Idle
    }
}
