use std::fs;

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

    let titles: Vec<String> = vec![
        "Guitar".into(),
        "Oboe".into(),
        "Violin 1".into(),
        "Violin 2".into(),
        "Cello".into(),
        "Viola 1".into(),
        "Viola 2".into(),
        "".into(),
    ];

    'main_loop: for rows in titles.chunks(4) {
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

    doc.push(table);

    doc.render_to_file("temp.pdf").unwrap();

    let buffer = fs::read("temp.pdf").unwrap();
    fs::remove_file("temp.pdf").unwrap();

    dbg!(buffer.len());
}
