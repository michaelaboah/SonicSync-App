use genpdf::elements::{FrameCellDecorator, PaddedElement, Paragraph, Text};
use genpdf::style::{Style, StyledString};
use genpdf::*;

#[derive(Default)]
struct AveryLabelPage {
    page: usize,
}

impl PageDecorator for AveryLabelPage {
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

#[derive(Default)]
struct AveryLabel {
    title: StyledString,
    desc: StyledString,
    source: StyledString,
    bundle: StyledString,
    connector: StyledString,
    alignment: Alignment,
}

impl AveryLabel {
    pub fn new(s: impl Into<StyledString>) -> Self {
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
        let StyledString {
            ref s,
            ref mut style,
        } = self.title;

        // area.add_margins(Margins::trbl(0.0, -3.35, 0.0, 6.35));

        area.set_height(12.7.into());
        area.set_width(44.45.into());

        let mut result = RenderResult::default();
        let Context { font_cache, .. } = &context;

        // Top section

        let line_style = Style::new().with_color(style::Color::Rgb(0, 0, 100));
        for i in 0..=3 {
            let points = vec![
                Position::new(0.0, i as f64),
                Position::new(area.size().width, i as f64),
            ];
            area.draw_line(points, line_style);
        } // .with_font_size(60).line_height(font_cache.clone().)

        if let Some(mut section) = area.text_section(font_cache, Position::default(), *style) {
            // area.add_margins(Margins::trbl(0.0, 6.35, 0.0, 6.35));
            let style = style
                .with_color(style::Color::Rgb(0, 0, 0))
                .with_font_size(10);
            section.print_str("Title", style).unwrap();
        }

        if let Some(mut section) = area.text_section(font_cache, Position::new(0.0, 3.0), *style) {
            // area.add_margins(Margins::trbl(0.0, 6.35, 0.0, 6.35));
            let style = style
                .with_color(style::Color::Rgb(255, 255, 255))
                .with_font_size(10);
            section.print_str("Description?", style).unwrap();
            // assert!(section.add_newline());
            // section.print_str("Description 2?", style).unwrap();
        }

        if let Some(mut section) = area.text_section(font_cache, Position::new(0.0, 6.0), *style) {
            // area.add_margins(Margins::trbl(0.0, 6.35, 0.0, 6.35));
            let style = style
                .with_color(style::Color::Rgb(100, 0, 0))
                .with_font_size(10);
            section.print_str("Title", style).unwrap();
        }

        if let Some(mut section) = area.text_section(font_cache, Position::new(0.0, 9.0), *style) {
            // area.add_margins(Margins::trbl(0.0, 6.35, 0.0, 6.35));
            let style = style
                .with_color(style::Color::Rgb(100, 0, 0))
                .with_font_size(10);
            section.print_str("Description?", style).unwrap();
        }

        // let mut zones = area.split_horizontally(&[1, 1]);

        // if area.print_str(font_cache, Position::new(0.0, 5.0), *style, &s)? {
        result.size = area.size(); // } else {
                                   // result.has_more = true;
                                   // }

        Ok(result)
    }
}

#[test]
fn example() {
    let font_family = fonts::from_files("assets/fonts/Roboto", "Roboto", None).unwrap();
    let mut doc = Document::new(font_family);

    doc.set_paper_size(PaperSize::Letter);
    doc.set_title("Labels");
    let decorator = AveryLabelPage::default();

    doc.set_page_decorator(decorator);

    // let mut table = elements::TableLayout::new(vec![7, 2, 7, 2, 7, 1, 7]);
    let mut table = elements::TableLayout::new(vec![1, 1, 1, 1]);

    table.set_cell_decorator(FrameCellDecorator::new(false, false, false));

    let mut row = table.row();

    row.push_element(AveryLabel::new("Content of Some Kind"));

    row.push_element(PaddedElement::new(
        AveryLabel::new("Content of Some Kind"),
        Margins::trbl(0.0, 0.0, 0.0, 2.0),
    ));

    row.push_element(PaddedElement::new(
        AveryLabel::new("Content of Some Kind"),
        Margins::trbl(0.0, 0.0, 0.0, 4.0),
    ));

    row.push_element(PaddedElement::new(
        AveryLabel::new("Content of Some Kind"),
        Margins::trbl(0.0, 0.0, 0.0, 6.0),
    ));

    row.push().unwrap();

    doc.push(table);

    doc.render_to_file("test_pdf.pdf").unwrap();
}
