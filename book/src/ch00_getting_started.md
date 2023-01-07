# Getting Started

> # To discuss
> - winit + glutin vs sdl2
> 	- creating a window
> 	- create a context, loading gl functions
> 	- debug callbacks + debug context
> - framework for rest of tutorial
> 	- to cope with differences between winit/sdl2 if I choose to provide both

Before you can start rendering anything, first you need a window and an OpenGL context.
There are several ways to do this each with different pros and cons, but for the sake of simplicity I will just be sticking to [`sdl2`].

SDL2 is a C library that has been used for windowing and co since antiquity, so it has the advantage that it is very easy to get started with. _However_, being a C library means that including it in your project will require either you have the SDL2 library available on your system, _or_ you have cmake and a C compiler installed. Using C libraries in rust projects always adds some friction, so I will be structuring my examples such that SDL2 can be swapped out for something else later if need be.

The alternative, rust-only option is to use the [`winit`] and [`glutin`] crates - which would make building much easier, but in my opinion are significantly more difficult to get started with, so I will skip over them for now.

## Getting SDL2 set up

To start you need to add the [`sdl2`] crate as a dependency. So in your `Cargo.toml` add the following:
```toml
[dependencies.sdl2]
version = "0.35"
features = ["bundled", "static-link"]
```
The version is not super important - `0.35` is just the latest minor version available to me at time of writing. The important part here are the features: `bundled` and `static-link`. Long story short, these features together instruct the [`sdl2`] crate to fetch the SDL2 source, build it with the local compiler, and statically link it into your binary. As mentioned previously, this requires that you have [`cmake`] and a C compiler available on your system, but will ensure that we can build our project on any platform that SDL2 supports without much hassle.


## Create a window

Next its time to actually use [`sdl2`]. Add the following to your `main.rs`:
```rs
{{#include ../../examples/ch00/src/main.rs:just_window}}
```
If everything is set up correctly, then when you build and run this you should see a window appear for three seconds.

## Create an OpenGL context

Next we have to create an OpenGL context. This happens in a few parts:
- First, _before we make a window_ we have to describe to [`sdl2`] what kind of context we want.
	- e.g., what version? what profile? whats our backbuffer format? etc etc.
- Then, after we've created our window, we can go about actually asking for the context.
	- Note that before we use it we **have** to make it 'current'.
- Finally, once our new context has been created and made current, we can load our OpenGL functions from it.

### Describing the context

```rs
{{#include ../../examples/ch00/src/main.rs:describe_context}}
```

### Creating the context
```rs
{{#include ../../examples/ch00/src/main.rs:create_context_and_make_current}}
```

### Loading functions
```rs
{{#include ../../examples/ch00/src/main.rs:load_gl_funcs}}
```

### Making sure it works
```rs
{{#include ../../examples/ch00/src/main.rs:make_sure_it_works}}
```

## Debug contexts





[`sdl2`]: https://docs.rs/sdl2/latest/sdl2/
[`winit`]: https://docs.rs/winit/latest/winit/
[`glutin`]: https://docs.rs/glutin/latest/glutin/
[`cmake`]: https://cmake.org/download/