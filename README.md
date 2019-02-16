# oriole-text

This crate simplifies text rending with the GPU.
It is meant to be used for rendering scalable text
within higher-level UI crates.

# Concept
Inside your project's `build.rs` file, you use
`oriole-text-dev` to bake the signed distance fields 
of the characters of a font into a single texture atlas.

```rust
// inside build.rs, at compile time
fn main(){
    let font = oriole_text_dev::generate_font("Roboto.ttf");
    
    let mut baked_font_file = std::io::File::create("Roboto.baked");
    font.write(&mut baked_font_file);
}
```

That texture atlas can be quickly loaded at runtime. 
This avoids rendering individual
glyphs vectors with high resolution 
every time the app is started, 
while enabling scalable renderings
with reasonable quality.

```rust
// inside your project's runtime
fn main(){
    let font = oriole_text::Font::read(include_bytes!("Roboto.baked"));
    
    // draw a text using the glium renderer:
    let text_mesh = oriole_text::glium_render::Text::new(&font, "Hello Wörld!");
    text_mesh.draw(&display).unwrap();
}
```

## Cons
- The set of supported glyphs is specified at compile-time

## Pros
- Truly scalable text without repeated rasterization 
  of glyphs at runtime
- Resolution independent: At runtime, normalized coordinates are
  used so that you will never have to worry about pixels.

## Rendering the atlas
The rendering process is backend agnostic, and can be used with any
rendering crate, for example `glium`, `vulkano`, or `gfx`. The output
of this library is the font metrics and the texture atlas,
which can be uploaded directly to the GPU.

Furthermore, the optional feature `glium_text` will provides builtin
shaders and meshes to render the distance field text using `glium`. 
Builtin `vulkano` and/or `gfx` support is planned.
