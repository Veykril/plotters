# Plotters - Rust Drawing Library for Visualization  🦀 📈🚀 

<a href="https://crates.io/crates/plotters">
    <img style="display: inline!important" src="https://img.shields.io/crates/v/plotters.svg"></img>
</a>
<a href="https://docs.rs/plotters">
    <img style="display: inline!important" src="https://docs.rs/plotters/badge.svg"></img>
</a>
<a href="https://plumberserver.com/plotters-docs/plotters">
	<img style="display: inline! important" src="https://img.shields.io/badge/docs-development-lightgrey.svg"></img>
</a>
<a href="https://travis-ci.org/38/plotters">
	<img style="display: inline! important" src="https://travis-ci.org/38/plotters.svg?branch=master"></img>
</a>

Plotters is drawing library designed for rendering figures, plots, and charts, in pure rust. Plotters supports various types of backends, 
including bitmap, vector graph, piston window and WebAssembly. 

- You can try Plotters with Jupyter notebook, or view [here](https://plotters-rs.github.io/plotters-doc-data/evcxr-jupyter-integration.html) for the static HTML version.

## Gallery

$$gallery$$


$$[TOC]$$

## Quick Start

To use Plotters, you can simply add Plotters into your `Cargo.toml`
```toml
[dependencies]
plotters = "^$LATEST_VERSION"
```

And the following code draws a quadratic function. `src/main.rs`,

```rust
$$examples/quick_start.rs$$
```

![](https://plotters-rs.github.io/plotters-doc-data/0.png)


## Trying with Jupyter evcxr Kernel Interactively

Plotters now supports integrate with `evcxr` and is able to interactively drawing plots in Jupyter Notebook.
The feature `evcxr` should be enabled when including Plotters to Jupyter Notebook.

The following code shows a minimal example of this.

```text
:dep plotters = { git = "https://github.com/38/plotters", default_features = false, features = ["evcxr"] }
extern crate plotters;
use plotters::prelude::*;

let figure = evcxr_figure((640, 480), |root| {
    root.fill(&WHITE);
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
        &RED,
    )).unwrap()
        .label("y = x^2")
        .legend(|(x,y)| Path::new(vec![(x,y), (x + 20,y)], &RED));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
});
figure
```

<img src="https://plotters-rs.github.io/plotters-doc-data/evcxr_animation.gif" width="450px"></img>

## Interactive Tutorial with Jupyter Notebook

*This tutorial is now working in progress and isn't complete*

Thanks to the evcxr, now we have an interactive tutorial for Plotters!
To use the interactive notebook, you must have Jupyter and evcxr installed on your computer. 
Follow the instruction on [this page](https://github.com/google/evcxr/tree/master/evcxr_jupyter) below to install it.

After that, you should be able to start your Jupyer server locally and load the tutorial!

```bash
git clone https://github.com/38/plotters-doc-data
cd plotteres-doc-data
jupyter notebook
```

And select the notebook called `evcxr-jupyter-integration.ipynb`.

Also, there's a static HTML version of this notebook available at the [this location](https://plumberserver.com/plotters-docs/evcxr-jupyter-integration.html)

## Plotting in Rust

Rust is a perfect language for data visualization. Although there are many mature visualization libraries in many different languages.
But Rust is one of the best languages fits the need.

* **Easy to use** Rust has a very good iterator system built into the standard library. With the help of iterators,
Plotting in Rust can be as easy as most of the high-level programming languages. The Rust based plotting library
can be very easy to use.

* **Fast** If you need rendering a figure with trillions of data points, 
Rust is a good choice. Rust's performance allows you to combine data processing step 
and rendering step into a single application. When plotting in high-level programming languages,
e.g. Javascript or Python, data points must be downsampled before feeding into the plotting 
program because of the performance considerations. Rust is fast enough to do the data processing and visualization 
within a single program. You can also integrate the 
figure rendering code into your application handling a huge amount of data and visualize it in real-time.

* **WebAssembly Support** Rust is one of few the language with the best WASM support. Plotting in Rust could be 
very useful for visualization on a web page and would have a huge performance improvement comparing to Javascript.

## Plotting on HTML5 canvas with WASM Backend

Plotters currently supports backend that uses the HTML5 canvas. To use the WASM support, you can simply use 
`CanvasBackend` instead of other backend and all other API remains the same!

There's a small demo for Plotters + WASM under `examples/wasm-demo` directory of this repo. 
To play with the deployed version, follow this [link](https://plumberserver.com/plotters-wasm-demo/index.html).


## What types of figure are supported?

Plotters is not limited to any specific type of figure.
You can create your own types of figures easily with the Plotters API.

But Plotters provides some builtin figure types for convenience.
Currently, we support line series, point series, candlestick series, and histogram.
And the library is designed to be able to render multiple figure into a single image.
But Plotter is aimed to be a platform that is fully extendable to support any other types of figure.

## Concepts by examples

### Drawing Backends
Plotters can use different drawing backends, including SVG, BitMap, and even real-time rendering. For example, a bitmap drawing backend.

```rust
$$examples/drawing_backends.rs$$
```

![](https://plotters-rs.github.io/plotters-doc-data/1.png)

### Drawing Area
Plotters uses a concept called drawing area for layout purpose.
Plotters support multiple integrating into a single image.
This is done by creating sub-drawing-areas.

Besides that, the drawing area also allows the customized coordinate system, by doing so, the coordinate mapping is done by the drawing area automatically.

```rust
$$examples/drawing_area.rs$$
```

![](https://plotters-rs.github.io/plotters-doc-data/2.png)

### Elements

In Plotters, elements are build blocks of figures. All elements are able to draw on a drawing area.
There are different types of built-in elements, like lines, texts, circles, etc.
You can also define your own element in the application code.

You may also combine existing elements to build a complex element.

To learn more about the element system, please read the [element module documentation](./element/index.html).

```rust
$$examples/elements.rs$$
```

![](https://plotters-rs.github.io/plotters-doc-data/3.png)

### Composable Elements

Besides the built-in elements, elements can be composed into a logic group we called composed elements.
When composing new elements, the upper-left corner is given in the target coordinate, and a new pixel-based 
coordinate which has the upper-left corner defined as `(0,0)` is used for further element composition purpose.

For example, we can have an element which includes a dot and its coordinate.

```rust
$$examples/composable_elements.rs$$
```

![](https://plotters-rs.github.io/plotters-doc-data/4.png)

### Chart Context

In order to draw a chart, Plotters need a data object built on top of the drawing area called `ChartContext`.
The chart context defines even higher level constructs compare to the drawing area.
For example, you can define the label areas, meshes, and put a data series onto the drawing area with the help
of the chart context object.

```rust
$$examples/chart.rs$$
```

![](https://plotters-rs.github.io/plotters-doc-data/5.png)

## Misc

### Development Version

To use the latest development version, pull https://github.com/38/plotters.git. In `Cargo.toml`

```toml
[dependencies]
plotters = { git = "https://github.com/38/plotters.git" }
```

### Reducing Depending Libraries && Turning Off Backends
Plotters now supports use features to control the backend dependencies. By default, `BitMapBackend` and `SVGBackend` are supported,
use `default_features = false` in the dependency description in `Cargo.toml` and you can cherrypick the backend implementations.

- `svg` Enable the `SVGBackend`
- `bitmap` Enable the `BitMapBackend`

For example, the following dependency description would avoid compiling with bitmap support:

```toml
[dependencies]
plotters = { git = "https://github.com/38/plotters.git", default_features = false, features = ["svg"] }
```

The library also allows consumers to make use of the [`Palette`](https://crates.io/crates/palette/) crate's color types by default.
This behaviour can also be turned off by setting `default_features = false`.

$$style$$
