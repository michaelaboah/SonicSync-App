use genpdf::elements::{FrameCellDecorator, PaddedElement, Paragraph, Text};
use genpdf::style::{Style, StyledString};
use genpdf::*;
use std::{fs, io};
#[derive(Default)]
struct AveryLabelLayout {
    page: usize,
}

impl PageDecorator for AveryLabelLayout {
    fn decorate_page<'a>(
        &mut self,
        context: &Context,
        mut area: render::Area<'a>,
        style: style::Style,
    ) -> Result<render::Area<'a>, error::Error> {
        self.page += 1;

        area.add_margins(Margins::trbl(6.5 * 2.0, 6.5, 6.5 * 2.0, 6.9));

        Ok(area)
    }
}

#[derive(Default, Debug)]
struct AveryLabel {
    title: String,
    desc: String,
    source: String,
    bundle: String,
    connector: String,
    alignment: Alignment,
}

impl AveryLabel {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            title: s.into(),
            ..Default::default()
        }
    }

    fn set_alignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    fn get_offset(&self, width: Mm, max_width: Mm, alignment: Alignment) -> Mm {
        match self.alignment {
            Alignment::Left => Mm::default(),
            Alignment::Center => (max_width - width) / 2.0,
            Alignment::Right => max_width - width,
        }
    }
}

impl Element for AveryLabel {
    fn render(
        &mut self,
        context: &Context,
        mut area: render::Area<'_>,
        style: style::Style,
    ) -> Result<RenderResult, error::Error> {
        area.set_height(12.7.into());
        area.set_width(44.45.into());

        let base_style = Style::default();
        let mut result = RenderResult::default();
        let Context { font_cache, .. } = &context;

        // Top Bar

        let top_bar = Style::new().with_color(style::Color::Rgb(0, 0, 255));
        let coefficient = 3.5;
        for i in 0..=15 {
            let points = vec![
                Position::new(0.0, i as f64 / coefficient),
                Position::new(area.size().width, i as f64 / coefficient),
            ];
            area.draw_line(points, top_bar);
        }

        if let Some(mut section) =
            area.text_section(font_cache, Position::new(0.0, -1.5), base_style)
        {
            let style = style
                .with_color(style::Color::Rgb(255, 255, 255))
                .with_font_size(12);
            section.print_str(&self.title, style).unwrap();
        }

        if let Some(mut section) =
            area.text_section(font_cache, Position::new(0.0, 2.0), base_style)
        {
            let style = style
                .with_color(style::Color::Rgb(0, 0, 0))
                .with_font_size(8);
            section.print_str("T2", style).unwrap();
        }

        if let Some(mut section) =
            area.text_section(font_cache, Position::new(0.0, 4.0), base_style)
        {
            // area.add_margins(Margins::trbl(0.0, 6.35, 0.0, 6.35));
            let style = style
                .with_color(style::Color::Rgb(0, 0, 0))
                .with_font_size(8);
            section.print_str("B2", style).unwrap();
        }

        // Gray
        let bottom_divider = Style::new().with_color(style::Color::Rgb(200, 200, 200));

        let points = vec![
            Position::new(0.0, 9.0),
            Position::new(area.size().width, 9.0),
        ];
        area.draw_line(points, bottom_divider);

        let bottom_zones = area.split_horizontally(&[1, 1]);
        if let Some(mut section) =
            bottom_zones[0].text_section(font_cache, Position::new(0.0, 6.0), base_style)
        {
            let style = style
                .with_color(style::Color::Rgb(0, 0, 0))
                .with_font_size(8);
            section.print_str("Bottom Left", style).unwrap();
        }

        if let Some(mut section) =
            bottom_zones[1].text_section(font_cache, Position::new(0.0, 6.0), base_style)
        {
            let style = style
                .with_color(style::Color::Rgb(0, 0, 0))
                .with_font_size(8);
            section.print_str("Bottom Right", style).unwrap();
        }

        result.size = area.size();

        Ok(result)
    }
}

pub struct AveryLabelPage {
    labels: Vec<String>,
    document: Document,
}

impl AveryLabelPage {
    pub fn new(labels: Vec<String>) -> Self {
        let font_family = fonts::from_files("assets/fonts/Roboto", "Roboto", None).unwrap();
        let mut doc = Document::new(font_family);
        Self {
            labels,
            document: doc,
        }
    }

    fn render(&mut self) {
        self.document.set_paper_size(PaperSize::Letter);
        self.document.set_title("Labels");
        let decorator = AveryLabelLayout::default();

        self.document.set_page_decorator(decorator);

        let mut table = elements::TableLayout::new(vec![1, 1, 1, 1]);

        table.set_cell_decorator(FrameCellDecorator::new(false, false, false));

        'main_loop: for rows in self.labels.chunks(4) {
            dbg!(rows);
            let mut curr_index = 1;
            let mut row = table.row();
            'label_loop: for label in rows {
                if curr_index == 1 {
                    row.push_element(PaddedElement::new(
                        AveryLabel::new(label),
                        Margins::default(),
                    ));
                    curr_index += 1;
                    continue 'label_loop;
                }

                if curr_index == 2 {
                    row.push_element(PaddedElement::new(
                        AveryLabel::new(label),
                        Margins::trbl(0.0, 0.0, 0.0, 2.0),
                    ));
                    curr_index += 1;
                    continue 'label_loop;
                }

                if curr_index == 3 {
                    row.push_element(PaddedElement::new(
                        AveryLabel::new(label),
                        Margins::trbl(0.0, 0.0, 0.0, 4.0),
                    ));
                    curr_index += 1;

                    continue 'label_loop;
                }

                if curr_index == 4 {
                    row.push_element(PaddedElement::new(
                        AveryLabel::new(label),
                        Margins::trbl(0.0, 0.0, 0.0, 6.0),
                    ));
                    curr_index = 1;
                    break;
                }
                // row = table.row();
            }

            row.push().unwrap();
        }

        self.document.push(table);
    }

    /// This contains a super dumb workaround for getting some bytes in memory.
    /// Go into the [genpdf] crate and expose a method for getting some damn bytes with a
    /// [std::io::Cursor<Vec<u8>>]
    pub fn write_to_bytes(mut self, write_buffer: &mut impl std::io::Write) -> std::io::Result<()> {
        self.render();
        self.document.render_to_file("temp.pdf").unwrap();

        let buffer = fs::read("temp.pdf").unwrap();
        fs::remove_file("temp.pdf").unwrap();
        let written_bytes = write_buffer.write(&buffer).unwrap();
        assert_eq!(buffer.len(), written_bytes);
        Ok(())
    }
}

#[test]
fn example() {
    let font_family = fonts::from_files("assets/fonts/Roboto", "Roboto", None).unwrap();
    let mut doc = Document::new(font_family);
}
