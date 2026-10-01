<h1 align="center">iced_m3</h1>


<div align="center">
  
  ![](https://img.shields.io/github/last-commit/tpaau/iced_m3?&style=for-the-badge&color=FFFFFF&logo=git&logoColor=C9C9C9&labelColor=252525)
  ![](https://img.shields.io/github/repo-size/tpaau/iced_m3?&style=for-the-badge&color=FFFFFF&logo=git&logoColor=C9C9C9&labelColor=252525)
</div>

<div align="center">
  Implementation of the <a href="https://m3.material.io">Material Design 3</a> system for the <a href="https://iced.rs/">Iced GUI library</a>
</div>


## Features
- [Motion Physics](https://m3.material.io/styles/motion/overview) system
- [Dynamic Color](https://m3.material.io/styles/color/system) system
- [Elevation](https://m3.material.io/styles/elevation/overview) system
- Support for text or SVG icons
- [Widgets](#widgets)
  - [Badges](#widgets-badges)
  - [Common Buttons](#widgets-common-buttons)
  - [Cards](#widgets-cards)
  - [Dialog](#widgets-dialog)
  - [Floating Action Buttons](#widgets-fabs)
  - [FAB Menu](#widgets-fab-menu)
  - [Progress Bar](#widgets-progress-bar)
  - [Navigation Bar](#widgets-navbar)
  - [Navigation Rail](#widgets-navrail)
  - [Vertical Menu](#widgets-vertical-menu)
  - [Switch](#widgets-switch)
  - [Slider](#widgets-slider)
  - [Text Field](#widgets-text-field)

You can go see the demos [here](https://github.com/tpaau/iced_m3/blob/main/demos/)!

[Chilen](https://github.com/tpaau/chilen) uses `iced_m3` extensively, you can check how the library
is used there too!


## Usage
Run this in your project:
```bash
cargo add iced_m3 --git https://github.com/tpaau/iced_m3.git
```

This library is largely for use in my personal projects (like
[Chilen](https://github.com/tpaau/chilen)), and constantly changing, so consider pinning a commit if
you wish to use it in your project. I also don't plan on releasing this on
[crates.io](https://crates.io) anytime soon, largely for the same reason.

`iced_m3` is compatible with Iced **`0.14.0`** and will most likely be kept up to date with stable
Iced releases.


<a name="widgets"></a>
## Widgets


<a name="widgets-badges"></a>
### Badges
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/badges)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/badges.jpg)

> Badges show notifications, counts, or status information on navigation items and icons
>
> [Reference](https://m3.material.io/components/badges/overview)


<a name="widgets-common-buttons"></a>
### Common Buttons
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/buttons)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/buttons-demo.gif)

> Buttons prompt most actions in a UI
>
> [Reference](https://m3.material.io/components/buttons/overview)

TODOs:
- Animated ripple effect


<a name="widgets-cards"></a>
### Cards
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/cards)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/cards.gif)

> Cards display content and actions about a single subject
>
> [Reference](https://m3.material.io/components/cards/overview)

TODOs:
- Animated ripple affect
- Drag-and-drop


<a name="widgets-dialog"></a>
### Dialog
~[demo]()~

~![showcase]()~

> Dialogs provide important prompts in a user flow
>
> [Reference](https://m3.material.io/components/dialogs/overview)


<a name="widgets-fabs"></a>
### Floating Action Buttons
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/fab)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/fab.jpg)

> Floating action buttons (FABs) help people take primary actions
>
> Reference: [FABs](https://m3.material.io/components/floating-action-button/overview), [Extended FABs](https://m3.material.io/components/extended-fab/overview) (yes, adding a label makes it a separate widget somehow)

TODOs:
- Animated ripple affect


<a name="widgets-fab-menu"></a>
### FAB Menu
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/fab_menu)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/fab-menu-demo.gif)

> The floating action button (FAB) menu opens from a FAB to display multiple related actions
>
> [Reference](https://m3.material.io/components/fab-menu/overview)

TODOs:
- Nested menus
- Menu entry animation/ripple effect


<a name="widgets-navbar"></a>
### Navigation Bar
~[demo]()~

~![showcase]()~

> Navigation bars let people switch between UI views on smaller devices
>
> [Reference](https://m3.material.io/components/navigation-bar/overview)

TODOs:
- Animation


<a name="widgets-navrail"></a>
### Navigation Rail
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/navrail)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/navigation-rail.gif)

> Navigation rails let people switch between UI views on mid-sized devices
>
> [Reference](https://m3.material.io/components/navigation-rail/overview)

TODOs:
- Expand/Contract animation


<a name="widgets-progress-bar"></a>
### Progress Bar
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/progress_indicators)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/progress-bar-demo.gif)

> Progress indicators show the status of a process in real time
>
> [Reference](https://m3.material.io/components/progress-indicators/overview)

TODOs:
- Circular progress indicator
- Squiggly variant!!
- Fix jank in animation


<a name="widgets-vertical-menu"></a>
### Vertical Menu
~[demo]()~

~![showcase]()~

> Menus display a list of choices on a temporary surface
>
> [Reference](https://m3.material.io/components/menus/overview)

TODOs:
- Item state layer ripple


<a name="widgets-switch"></a>
### Switch
[![demo](https://img.shields.io/badge/demo-a?&style=for-the-badge&color=D0BCFF&logo=material-design&logoColor=D0BCFF&labelColor=381E72)](https://github.com/tpaau/iced_m3/blob/main/demos/switch)

![showcase](https://github.com/tpaau/iced_m3/blob/main/showcase/switch.gif)

> Switches toggle the selection of an item on and off
>
> [Reference](https://m3.material.io/components/switch/overview)

TODOs:
- State layer ripple effect


<a name="widgets-slider"></a>
### Slider
~[demo]()~

~![showcase]()~

> Sliders allow users to make selections from a range of values
>
> [Reference](https://m3.material.io/components/sliders/overview)

TODOs:
- Other value type variants
- Snapping


<a name="widgets-text-field"></a>
### Text Field
~[demo]()~

~![showcase]()~

> Text fields let users enter text into a UI
>
> [Reference](https://m3.material.io/components/text-fields/overview)


## Advanced
Widgets used internally by `iced_m3` for other widgets that may also be useful outside of `iced_m3`.
Those are gated behind the `advanced` feature.
