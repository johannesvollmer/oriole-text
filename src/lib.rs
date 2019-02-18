pub mod font;
pub mod text;
pub mod rectangle;

pub mod prelude {
    pub use crate::font::Font;
//    pub use crate::text::TextCaretPositions;

}


#[cfg(test)]
pub mod test {
    use crate::font::Font;
    use crate::font::FontLayout;
    use crate::font::GlyphLayout;
    use crate::rectangle::Rectangle;
    use crate::text::BuiltGlyph;
    use crate::font::GlyphQuad;
    use crate::font::Atlas;

    #[test]
    pub fn construct_text(){
        let stub_atlas = Atlas {
            resolution: (0, 0),
            distance_field: vec![]
        };

        let font = Font {
            atlas: stub_atlas,
            glyphs: vec![
                ('a', GlyphLayout {
                    advance_x: 12.1,
                    quad: Some(GlyphQuad {
                        geometry: Rectangle {
                            position: (0.1, 0.2),
                            dimensions: (8.1, 8.2)
                        },
                        texture: Rectangle {
                            position: (0.01, 0.02),
                            dimensions: (8.01, 8.02)
                        }
                    })
                }),

                ('b', GlyphLayout {
                    advance_x: 12.4,
                    quad: Some(GlyphQuad {
                        geometry: Rectangle {
                            position: (0.4, 0.3),
                            dimensions: (8.4, 8.3)
                        },
                        texture: Rectangle {
                            position: (0.04, 0.03),
                            dimensions: (8.04, 8.03)
                        }
                    })
                }),

                (' ', GlyphLayout {
                    advance_x: 5.4,
                    quad: None,
                }),

            ].into_iter().collect(),

            kerning: vec![
                (('a', 'b'), 3.1)
            ].into_iter().collect(),

            layout: FontLayout {
                advance_y: 15.0,
                ascent: 10.0,
                descent: -10.0
            }
        };

        for x in font.layout_glyphs("aab".chars()) {
            println!("built glyph {}", x.character);
        }

        let layout: Vec<BuiltGlyph> = font.layout_glyphs("aab a".chars()).collect();

        assert_eq!(layout.len(), 4);

        assert_eq!(layout[0].character, 'a');
        assert_eq!(layout[1].character, 'a');
        assert_eq!(layout[2].character, 'b');
        assert_eq!(layout[3].character, 'a');

        assert_eq!(layout[0].index_in_line, 0);
        assert_eq!(layout[1].index_in_line, 1);
        assert_eq!(layout[2].index_in_line, 2);
        assert_eq!(layout[3].index_in_line, 4);

        assert_eq!(layout[0].quad.geometry.position, (0.1, 0.2));
        assert_eq!(layout[1].quad.geometry.position, (0.1 + 12.1, 0.2));
        assert_eq!(layout[2].quad.geometry.position, (0.4 + 2.0*12.1 + 3.1, 0.3));
    }

}