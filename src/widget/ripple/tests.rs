use std::time::{Duration, Instant};

use iced::{Point, Size};

use super::{constant::*, model::*, *};
use crate::animation::standard;

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

fn ctx() -> Context {
    Context {
        size: Size::new(100.0, 40.0),
        enabled: true,
    }
}

fn step(model: Model, message: Message, t0: Instant, at: u64) -> Model {
    update(model, message, ctx(), t0 + ms(at)).0
}

fn press(pointer: Pointer) -> Message {
    Message::Press {
        pointer,
        at: Point::new(10.0, 10.0),
    }
}

fn near(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

#[test]
fn press_geometry_matches_the_reference_vectors() {
    let now = Instant::now();
    let button = Press::new(None, Size::new(100.0, 40.0), now);
    assert_eq!(button.initial_size, 20.0);
    assert!(near(button.scale, 9.635, 1e-3), "{}", button.scale);

    let fab = Press::new(None, Size::new(56.0, 56.0), now);
    assert_eq!(fab.initial_size, 11.0);
    assert!(near(fab.scale, 14.927, 1e-3), "{}", fab.scale);
}

#[test]
fn standard_easing_endpoints_and_accuracy() {
    assert_eq!(standard(0.0), 0.0);
    assert!(near(standard(1.0), 1.0, 1e-6));

    // Bisection reference for cubic-bezier(0.2, 0, 0, 1).
    let reference = |t: f32| {
        let x = |u: f32| 3.0 * (1.0 - u).powi(2) * u * 0.2 + u.powi(3);
        let y = |u: f32| 3.0 * (1.0 - u) * u * u + u.powi(3);
        let (mut lo, mut hi) = (0.0f32, 1.0f32);
        for _ in 0..60 {
            let mid = (lo + hi) / 2.0;
            if x(mid) < t {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        y((lo + hi) / 2.0)
    };
    for i in 0..=100 {
        let t = i as f32 / 100.0;
        assert!(
            (standard(t) - reference(t)).abs() < 1e-4,
            "t = {t}: {} vs {}",
            standard(t),
            reference(t)
        );
    }
}

#[test]
fn mouse_press_phases() {
    let t0 = Instant::now();
    let mut model = Model::default();
    assert_eq!(model.phase, Phase::Inactive);

    model = step(model, press(Pointer::Mouse), t0, 0);
    assert_eq!(model.phase, Phase::Holding);
    assert!(model.press.is_some());

    model = step(
        model,
        Message::Release {
            pointer: Pointer::Mouse,
        },
        t0,
        300,
    );
    assert_eq!(model.phase, Phase::WaitingForClick);

    model = step(model, Message::Click, t0, 300);
    assert_eq!(model.phase, Phase::Inactive);
    assert!(!model.press.unwrap().pressed);
}

#[test]
fn short_touch_press_starts_on_release() {
    let t0 = Instant::now();
    let touch = Pointer::Touch(1);
    let mut model = step(Model::default(), press(touch), t0, 0);
    assert!(matches!(model.phase, Phase::TouchDelay { .. }));
    assert!(model.press.is_none());

    model = step(model, Message::Release { pointer: touch }, t0, 50);
    assert_eq!(model.phase, Phase::WaitingForClick);
    assert!(model.press.is_some());

    model = step(model, Message::Click, t0, 50);
    assert_eq!(model.phase, Phase::Inactive);
}

#[test]
fn long_touch_press_starts_after_the_delay() {
    let t0 = Instant::now();
    let touch = Pointer::Touch(1);
    let mut model = step(Model::default(), press(touch), t0, 0);

    model = step(model, Message::Tick, t0, 149);
    assert!(matches!(model.phase, Phase::TouchDelay { .. }));
    assert!(model.press.is_none());

    model = step(model, Message::Tick, t0, 150);
    assert_eq!(model.phase, Phase::Holding);
    assert_eq!(model.press.unwrap().started_at, t0 + ms(150));

    model = step(model, Message::Release { pointer: touch }, t0, 400);
    assert_eq!(model.phase, Phase::WaitingForClick);
}

#[test]
fn cancel_during_touch_delay_never_starts() {
    let t0 = Instant::now();
    let touch = Pointer::Touch(7);
    let mut model = step(Model::default(), press(touch), t0, 0);
    model = step(model, Message::Cancel { pointer: touch }, t0, 20);
    model = step(model, Message::Tick, t0, 200);
    assert_eq!(model.phase, Phase::Inactive);
    assert!(model.press.is_none());
}

#[test]
fn second_pointer_is_ignored() {
    let t0 = Instant::now();
    let model = step(Model::default(), press(Pointer::Mouse), t0, 0);
    let after = step(model, press(Pointer::Touch(2)), t0, 10);
    assert_eq!(after, model);
}

#[test]
fn leaving_ends_a_mouse_press_but_not_a_touch() {
    let t0 = Instant::now();
    let model = step(Model::default(), press(Pointer::Mouse), t0, 0);
    let model = step(model, Message::Hover(false), t0, 300);
    assert_eq!(model.phase, Phase::Inactive);

    let touch = step(Model::default(), press(Pointer::Touch(1)), t0, 0);
    let touch = step(touch, Message::Hover(false), t0, 10);
    assert!(matches!(touch.phase, Phase::TouchDelay { .. }));
}

#[test]
fn release_is_delayed_to_the_minimum_press_time() {
    let t0 = Instant::now();

    let mut model = step(Model::default(), press(Pointer::Mouse), t0, 0);
    model = step(
        model,
        Message::Release {
            pointer: Pointer::Mouse,
        },
        t0,
        100,
    );
    model = step(model, Message::Click, t0, 100);
    let pending = model.press.unwrap();
    assert!(pending.pressed);
    assert_eq!(pending.release_at, Some(t0 + ms(225)));
    assert_eq!(model.next_deadline(), Some(t0 + ms(225)));

    model = step(model, Message::Tick, t0, 224);
    assert!(model.press.unwrap().pressed);
    model = step(model, Message::Tick, t0, 225);
    assert!(!model.press.unwrap().pressed);

    let mut late = step(Model::default(), press(Pointer::Mouse), t0, 0);
    late = step(
        late,
        Message::Release {
            pointer: Pointer::Mouse,
        },
        t0,
        300,
    );
    late = step(late, Message::Click, t0, 300);
    assert!(!late.press.unwrap().pressed);
    assert_eq!(late.press.unwrap().release_at, None);
}

#[test]
fn opacity_reaches_its_target_in_the_specified_time() {
    let t0 = Instant::now();

    let hover = Transition::default().retarget(1.0, HOVER_FADE, t0);
    assert!(near(hover.value(t0 + ms(15)), 1.0, 1e-6));
    assert!(hover.value(t0 + ms(14)) < 1.0);
    assert!(!hover.is_running(t0 + ms(15)));

    let fade_in = Transition::default().retarget(1.0, PRESS_FADE_IN, t0);
    assert!(near(fade_in.value(t0 + ms(105)), 1.0, 1e-6));
    assert!(fade_in.value(t0 + ms(104)) < 1.0);

    let fade_out = Transition::settled(1.0).retarget(0.0, PRESS_FADE_OUT, t0);
    assert!(near(fade_out.value(t0 + ms(375)), 0.0, 1e-6));
    assert!(fade_out.value(t0 + ms(374)) > 0.0);
}

#[test]
fn interrupted_transition_reverses_faster() {
    let t0 = Instant::now();
    let forward = Transition::default().retarget(1.0, PRESS_FADE_OUT, t0);
    // 25 % of the way there, then reverse: the way back takes 25 % of the time.
    let reversed = forward.retarget(0.0, PRESS_FADE_OUT, t0 + ms(375 / 4));
    assert!(near(reversed.value(t0 + ms(375 / 4)), 0.25, 1e-2));
    let end = t0 + ms(375 / 4) + ms(375 / 4) + ms(1);
    assert!(near(reversed.value(end), 0.0, 1e-6));
    assert!(!reversed.is_running(end));
}

#[test]
fn view_hides_the_press_for_tiny_containers() {
    let t0 = Instant::now();
    let tiny = Context {
        size: Size::new(4.0, 4.0),
        enabled: true,
    };
    let (model, _) = update(Model::default(), press(Pointer::Mouse), tiny, t0);
    assert_eq!(view(&model, tiny, t0 + ms(50)).press, None);
}

#[test]
fn view_disc_grows_from_the_press_point_to_the_center() {
    let t0 = Instant::now();
    let (model, _) = update(Model::default(), press(Pointer::Mouse), ctx(), t0);

    let start = view(&model, ctx(), t0 + ms(110)).press.unwrap();
    let end = view(&model, ctx(), t0 + ms(450)).press.unwrap();
    assert!(start.radius < end.radius);
    assert!(near(end.center.x, 50.0, 1e-3) && near(end.center.y, 20.0, 1e-3));
    // Final radius = (max_radius + soft_edge) / 2
    assert!(near(end.radius, 9.635 * 10.0, 0.1), "{}", end.radius);
    assert!(near(end.solid_ratio, GRADIENT_SOLID_MIN_RATIO, 1e-6));
}

#[test]
fn disabled_context_resets_everything() {
    let t0 = Instant::now();
    let (model, _) = update(Model::default(), press(Pointer::Mouse), ctx(), t0);
    let disabled = Context {
        enabled: false,
        ..ctx()
    };
    let (model, effect) = update(model, Message::Tick, disabled, t0);
    assert_eq!(model, Model::default());
    assert_eq!(effect, Effect::Idle);
    assert_eq!(view(&model, disabled, t0), Scene::default());
}

#[test]
fn effects_request_what_is_needed() {
    let t0 = Instant::now();
    let touch = Pointer::Touch(1);
    let (_, effect) = update(Model::default(), press(touch), ctx(), t0);
    assert_eq!(effect, Effect::WakeAt(t0 + TOUCH_DELAY));

    let (model, effect) = update(Model::default(), press(Pointer::Mouse), ctx(), t0);
    assert_eq!(effect, Effect::Redraw);
    let (_, effect) = update(model, Message::Tick, ctx(), t0 + ms(1000));
    assert!(matches!(effect, Effect::Redraw | Effect::Idle));
    let (model, _) = update(
        model,
        Message::Cancel {
            pointer: Pointer::Mouse,
        },
        ctx(),
        t0 + ms(1000),
    );
    let (_, effect) = update(model, Message::Tick, ctx(), t0 + ms(2000));
    assert_eq!(effect, Effect::Idle);
}
