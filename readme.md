# Bevy Ahoy!

[![crates.io](https://img.shields.io/crates/v/bevy_ahoy)](https://crates.io/crates/bevy_ahoy)
[![docs.rs](https://docs.rs/bevy_ahoy/badge.svg)](https://docs.rs/bevy_ahoy)

A cool KCC for Avian, heck yeah

I'll write more here later :)

## Inspiration

- The underlying move-and-slide uses Avian's implementation, the ispirations for which are [listed in the implementing PR](https://github.com/avianphysics/avian/pull/894).
- The core principles of the KCC derive from the Quake KCC. I highly recommend reading [myria666/qMovementDoc](https://github.com/myria666/qMovementDoc) if you want to know how it works :)
- The specific implementation flavor of the KCC is heavily inspired by Counter Strike: Source

## Alternatives / Comparison

- [tnua](https://github.com/idanarye/bevy-tnua): Floating dynamic character controller. Use this if you want more cartoony physics, or want your character to be affected by forces in a fully simulated way. Supports both Avian and Rapier.
- [bevy_fps_controller](https://github.com/qhdwight/bevy_fps_controller): KCC also inspired by Source. Does not integrate with BEI or with fixed update based workflows. Supports both Avian and Rapier.

## Design Philosophy

KCCs are incredibly closely tied to their games. At the same time, a lot of games or prototypes need something that Just Works.
For this reason, many KCC libraries try to be extremely configurable. However, I found that in my personal projects, I never 
really vibed with all that API baggage. For most use-cases, I just wanted a sensible set of defaults that Just Worked, thank you kindly. On the other hand, features that the library didn't plan for, like wall running, often lead me to explore the guts of the
library anyways, and a fork was more practical than indirectly charming the library into doing my bidding through its configuration.

As such, I designed Bevy Ahoy to be what I wished existed when I started Bevy: something simple that I don't need to spend much time learning, that I can just plug in and use if I need basic first person movement. To enable this, I consciously decided to limit the
configurability of Ahoy. If you need specific features to your game that Ahoy doesn't bring out of the box, I encourage you to fork 
it. Feel free to open an issue or ping me on the Bevy Discord if you need help with that :)

With that said, here are some goals of Ahoy:
- Require minimum setup
- Handle most terrain you throw at it
- Handle common collider shapes: Cuboids, cylinders, spheres, and in a pinch capsules
  - Sorry, Parry is not very good at capsules. You may want to use a cylinder instead for now :/
  - Other shapes may or may not work, at your discretion
- Be tightly integrated with `bevy_enhanced_input`
  - If you don't use BEI already, you really should :)
  - This allows Ahoy to neatly abstracts away some nasty internal business like input accumulation,
    while allowing you to bind its behaviors to whatever you want. 
  - Plus, BEI has a lovely lovely input mocking API, allowing us to treat player and NPC input the same way.
- Be tightly integrated with Avian
  - Supporting multiple physics engines directly brings with it the need to create a big layer of abstractions and some extra glue crates, which makes forking the library for your own needs much more complex.
  - Additionally, I prefer to upstream things I need directly to Avian, to make the Avian ecosystem a better place for everyone.
    I don't have the time or energy to do that for multiple physics engines, and I don't want to "polyfill" APIs that only some engines support.
  - I also just really like Avian <3
- Give that flowy-snappy-freeing movement you know and love from the Source Engine and early id Tech games like Quake.
  - This includes cool movement tech like air strafing and surfing.

In contrast, here are some deliberate non-goals:
- Deep configurability: just fork it instead, it should hopefully be simple.
- Be engine-agnostic
- Code specifically for disabling tech like air strafing
- Support schedules outside Bevy's fixed timestep. 
  - You can configure the schedule, but it must run as part of the fixed main loop to correctly work with Ahoy.
- Work without `bevy_enhanced_input`
- Other up-axis than Y
- Work as a dynamic character controller
- Support every possible collider shape
- Reproduce the behavior of Quake or Source exactly.

## Compatibility

| bevy        | bevy_bae               |
|-------------|------------------------|
| 0.17        | `main`                 |
