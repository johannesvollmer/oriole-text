use crate::font::Font;
use crate::font::GlyphQuad;
use crate::rectangle::Rectangle;


pub struct LayoutGlyphs<'f, S: Iterator<Item = char>> {
    chars: S,
    font: &'f Font,
    caret: (f32, f32),
    previous_char: Option<char>,

    line: usize,
    index_in_line: usize,
}

pub struct BuiltGlyph {
    pub character: char,
    pub index_in_line: usize,
    pub line: usize,
    pub quad: GlyphQuad,
}


/*pub struct TextCaretPositions {
    position_x: Vec<f32>
}*/



impl<'f, S> LayoutGlyphs<'f, S>
    where S: Iterator<Item=char>
{
    pub fn new(font: &'f Font, chars: S) -> Self {
        LayoutGlyphs {
            font, chars,
            caret: (0.0, 0.0),

            line: 0,
            index_in_line: 0,

            previous_char: None,
        }
    }
}

impl<'f, S> Iterator for LayoutGlyphs<'f, S>
    where S: Iterator<Item=char>
{
    type Item = BuiltGlyph;

    fn next(&mut self) -> Option<BuiltGlyph> {
        self.chars.next().and_then(|character|{
            let displayable_character = match character {
                '\n' => {
                    self.caret.1 += self.font.layout.advance_y;
                    self.caret.0 = 0.0;

                    self.line += 1;
                    self.index_in_line = 0;

                    None
                },

                /*' ' => {
                    self.caret.0 += self.font.layout.space_advance_x;
                    self.index_in_line += 1;
                    None
                },

                '\t' => {
                    self.caret.0 += self.font.layout.tab_advance_x;
                    self.index_in_line += 1;
                    None
                },*/

                character if !character.is_control() => { // handle displayable characters
                    // adjust caret for all following characters according to the kerning
                    if let Some(previous) = self.previous_char {
                        self.caret.0 += self.font.kerning.get(&(previous, character)).unwrap_or(&0.0);
                    }

                    self.previous_char = Some(character);

                    // acquire font metrics
                    let mesh_layout = self.font.glyphs.get(&character).unwrap();
                        // TODO .unwrap_or(&self.font.glyphs['\0']);

                    let current_caret = self.caret;
                    let current_index_in_line = self.index_in_line;

                    // advance caret and update indices
                    self.caret.0 += mesh_layout.advance_x;
                    self.index_in_line += 1;

                    // a character like ' ' or '\t' advances the caret but does not render anything,
                    // thus return none for those
                    mesh_layout.quad.map(|quad|{
                        BuiltGlyph {
                            character,
                            line: self.line,
                            index_in_line: current_index_in_line,
                            quad: GlyphQuad {
                                texture: quad.texture,
                                geometry: Rectangle {
                                    dimensions: quad.geometry.dimensions,
                                    position: (
                                        current_caret.0 + quad.geometry.position.0,
                                        current_caret.1 + quad.geometry.position.1
                                    ),
                                },
                            }
                        }
                    })
                },

                _ => None
            };

            // return the current character if it is renderable,
            // or continue with the next character otherwise
            displayable_character.or_else(|| self.next())
        })
    }
}

