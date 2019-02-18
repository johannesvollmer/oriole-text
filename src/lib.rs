pub mod atlas;
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
    use crate::atlas::Atlas;
    use crate::font::FontLayout;
    use crate::font::GlyphLayout;
    use crate::rectangle::Rectangle;
    use crate::text::BuiltGlyph;

    #[test]
    pub fn construct_text(){
        let stub_atlas = Atlas {
            glyphs: vec![
                ('a', Rectangle {
                    position: (0.01, 0.02),
                    dimensions: (8.01, 8.02)
                }),
                ('b', Rectangle {
                    position: (0.04, 0.03),
                    dimensions: (8.04, 8.03)
                }),

            ].into_iter().collect(),
            resolution: (0, 0),
            distance_field: vec![]
        };

        let font = Font {
            atlas: stub_atlas,
            glyphs: vec![
                ('a', GlyphLayout {
                    advance_x: 12.1,
                    bounds: Rectangle {
                        position: (0.1, 0.2),
                        dimensions: (8.1, 8.2)
                    },
                }),
                ('b', GlyphLayout {
                    advance_x: 12.4,
                    bounds: Rectangle {
                        position: (0.4, 0.3),
                        dimensions: (8.4, 8.3)
                    },
                }),

            ].into_iter().collect(),

            kerning: vec![
                (('a', 'b'), 3.1)
            ].into_iter().collect(),

            layout: FontLayout {
                advance_y: 15.0,
                space_advance_x: 6.3,
                tab_advance_x: 3.0 * 6.3,
                ascent: 10.0,
                descent: -10.0
            }
        };

        for x in font.layout_glyphs("aab".chars()) {
            println!("built glyph {}", x.character);
        }

        let layout: Vec<BuiltGlyph> = font.layout_glyphs("aab".chars()).collect();

        assert_eq!(layout.len(), 3);

        assert_eq!(layout[0].character, 'a');
        assert_eq!(layout[1].character, 'a');
        assert_eq!(layout[2].character, 'b');

        assert_eq!(layout[0].index_in_line, 0);
        assert_eq!(layout[1].index_in_line, 1);
        assert_eq!(layout[2].index_in_line, 2);

        assert_eq!(layout[0].layout.in_mesh.position, (0.1, 0.2));
        assert_eq!(layout[1].layout.in_mesh.position, (0.1 + 12.1, 0.2));
        assert_eq!(layout[2].layout.in_mesh.position, (0.4 + 2.0*12.1 + 3.1, 0.3));
    }

}