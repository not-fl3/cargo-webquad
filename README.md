# cargo-webquad

A little helper to run debug builds of miniquad/macroquad-based projects on web.

`cargo webquad serve --example instancing` from miniquad root will start a server on `http://localhost:8080` with the example built for web.

Note that this is not a recommended way to build examples, `cargo-webquad` is super limited and is in its earliest days. Project specific bash/make script works A LOT better.

However, if you jump through a lot of projects and occasionally needs to check if they still works on web - `cargo-webquad` might be useful.

`cargo-webquad` might be a good starting point for a private, usecase-specific fork. Its a very simple tool!

# What `cargo-webquad` could possibly be in the future?

There is an ongoing problem with miniquad, that JS part for cargo dependencies are not delivered through cargo - you need to download JS files and keep them updated.
In theory, its very easy for `cargo-webquad` to go through dependency tree, collect all JS files and put them all together.

# Usage

```
> cargo webquad serve

> cargo webquad serve --example example_name

> cargo webquad serve --example example_name --assets path/to/assets/to/deploy
```
