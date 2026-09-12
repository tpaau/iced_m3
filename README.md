<h1 align="center">iced_m3</h1>

<div align="center"><a href="https://m3.material.io">Material Design 3</a> widgets for the <a href="https://iced.rs/">iced GUI library</a></div>


## Usage
Run this in your project:
```bash
cargo add iced_m3 --git https://github.com/tpaau/iced_m3.git
```

This library is largely for use in my personal projects (like
[Chilen](https://github.com/tpaau/chilen)), and constantly changing, so consider pinning a commit if
you wish to use it in your project.

I also don't plan on releasing this on [crates.io](https://crates.io) anytime soon, largely for the
same reason, but also because I don't think `iced_m3` is mature enough to be put there.

`iced_m3` is compatible with iced **`0.14.0`** and most likely will be kept up to date with stable
releases.


## Features


## Material Widgets
Implementations of material widgets. Some of them more complete, some of them less complete. Some
animated, others not... You get the idea.

Note that those **aren't** styles, but rather widget wrappers, completely custom widgets or forks of
widgets from `iced_widget`.


### Buttons
![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/buttons-demo.gif)

[reference](https://m3.material.io/components/buttons/overview)

[demo](https://github.com/tpaau/iced_m3/blob/main/demos/buttons)

TODOs:
- Animated ripple effect
- Corner radius animation

### Dialog
~![showcase]()~

[reference](https://m3.material.io/components/dialogs/overview)

~[demo]()~

### FABs
![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/fab.jpg)

Reference: [FABs](https://m3.material.io/components/floating-action-button/overview), [Extended FABs](https://m3.material.io/components/extended-fab/overview) (yes, adding a label makes it a separate widget somehow)

[demo](https://github.com/tpaau/iced_m3/blob/main/demos/fab)

### FAB Menu
![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/fab-menu-demo.gif)

[reference](https://m3.material.io/components/fab-menu/overview)

[demo](https://github.com/tpaau/iced_m3/blob/main/demos/fab_menu)

TODOs:
- Animation


### Vertical Menu
~![showcase]()~

[reference](https://m3.material.io/components/menus/overview)

~[demo]()~

TODOs:
- Nested menus
- Menu entry animation/ripple effect

### Navigation Bar
~![showcase]()~

[reference](https://m3.material.io/components/navigation-bar/overview)

~[demo]()~

TODOs:
- Animation

### Progress Bar
![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/progress-bar-demo.gif)

[reference](https://m3.material.io/components/progress-indicators/overview)

[demo](https://github.com/tpaau/iced_m3/blob/main/demos/fab_menu)

TODOs:
- Squiggly variant!!
- Fix jank in animation

### Slider
~![showcase]()~

[reference](https://m3.material.io/components/sliders/overview)

~[demo]()~

### Text Field
~![showcase]()~

[reference](https://m3.material.io/components/text-fields/overview)

~[demo]()~


## Advanced
Widgets used internally by `iced_m3` for other widgets that may also be useful outside of `iced_m3`.
Those are gated behind the `advanced` feature.
