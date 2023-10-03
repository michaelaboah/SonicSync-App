use genpdf::elements::{FrameCellDecorator, PaddedElement};
use genpdf::fonts::FontCache;
use genpdf::style::{Color, Style};
use genpdf::*;
use std::fs;
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
struct AveryEmptyLabel;

impl Element for AveryEmptyLabel {
    fn render(
        &mut self,
        context: &Context,
        mut area: render::Area<'_>,
        style: style::Style,
    ) -> Result<RenderResult, error::Error> {
        let mut result = RenderResult::default();
        area.set_height(12.7.into());
        area.set_width(44.45.into());

        result.size = area.size();

        Ok(result)
    }
}

#[derive(Default, Debug)]
struct AveryLabel {
    label: CableLabel,
}

impl AveryLabel {
    pub fn new(label: CableLabel) -> Self {
        Self { label }
    }
    fn print_text_section(
        &mut self,
        area: &mut render::Area<'_>,
        position: Position,
        style: style::Style,
        text: &str,
        font_cache: &FontCache,
    ) -> Result<(), error::Error> {
        if let Some(mut section) = area.text_section(font_cache, position, style) {
            section.print_str(text, style)?;
        }
        Ok(())
    }

    // fn set_alignment(&mut self, alignment: Alignment) {
    //     self.alignment = alignment;
    // }
    //
    // fn get_offset(&self, width: Mm, max_width: Mm, alignment: Alignment) -> Mm {
    //     match self.alignment {
    //         Alignment::Left => Mm::default(),
    //         Alignment::Center => (max_width - width) / 2.0,
    //         Alignment::Right => max_width - width,
    //     }
    // }
}

const COEFFICIENT: f64 = 3.5;
const LABEL_HEIGHT: f64 = 12.7;
const LABEL_WIDTH: f64 = 44.45;

impl Element for AveryLabel {
    fn render(
        &mut self,
        context: &Context,
        mut area: render::Area<'_>,
        mut style: style::Style,
    ) -> Result<RenderResult, error::Error> {
        area.set_height(LABEL_HEIGHT.into());
        area.set_width(LABEL_WIDTH.into());

        style.set_font_size(12);
        let mut result = RenderResult::default();
        let Context { font_cache, .. } = context;

        let CableData {
            name,
            description,
            bundle,
            // model,
            length,
            destination_name,
            // cable_kind,
            ..
        } = self.label.data.clone();

        let LabelMeta { alignment, color } = &self.label.metadata;

        // Top bar
        let top_bar = Style::new().with_color(Color::Rgb(color.0, color.1, color.2));
        for i in 0..=15 {
            let points = vec![
                Position::new(0.0, i as f64 / COEFFICIENT),
                Position::new(LABEL_WIDTH, i as f64 / COEFFICIENT),
            ];
            area.draw_line(points, top_bar);
        }

        // Title
        self.print_text_section(
            &mut area,
            Position::new(0.0, -1.5),
            style.with_color(Color::Rgb(255, 255, 255)),
            &name,
            font_cache,
        );

        style.set_color(Color::Rgb(0, 0, 0));
        style.set_font_size(8);

        // "t2" text
        self.print_text_section(
            &mut area,
            Position::new(0.0, 2.0),
            style,
            &description,
            font_cache,
        );

        // "b2" text
        self.print_text_section(
            &mut area,
            Position::new(0.0, 4.0),
            style,
            &destination_name,
            font_cache,
        );

        // Gray bottom divider
        let bottom_divider = Style::new().with_color(Color::Rgb(200, 200, 200));
        let points = vec![Position::new(0.0, 9.0), Position::new(LABEL_WIDTH, 9.0)];
        area.draw_line(points, bottom_divider);

        // Bottom left text // Bundle Zone
        let mut bottom_zones = area.split_horizontally(&[1, 1]);

        self.print_text_section(
            &mut bottom_zones[0],
            Position::new(0.0, 4.0),
            style,
            &bundle.unwrap_or("".into()),
            font_cache,
        );

        // Bottom right text // Cable Termination Zone
        self.print_text_section(
            &mut bottom_zones[1],
            Position::new(0.0, 4.0),
            style,
            &format!("XLR, {}' Male", length),
            font_cache,
        );

        result.size = area.size();

        Ok(result)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CableLabel {
    metadata: LabelMeta,
    data: CableData,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CableData {
    name: String,
    description: String,
    cable_kind: String,
    bundle: Option<String>,
    model: String,
    length: f64,
    destination_name: String,
    connection_name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum LabelAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelMeta {
    alignment: LabelAlignment,
    color: (u8, u8, u8),
}

impl Default for LabelMeta {
    fn default() -> Self {
        Self {
            color: (255, 255, 255),
            alignment: Default::default(),
        }
    }
}

enum PossibleLabels {
    CableLabel {
        metadata: LabelMeta,
        data: CableData,
    },
    EmptyLabel,
}

pub struct AveryLabelPage {
    labels: Vec<CableLabel>,
    document: Document,
}

impl AveryLabelPage {
    pub fn new(labels: Vec<CableLabel>) -> Self {
        let font_family = fonts::from_files("assets/fonts/Roboto", "Roboto", None).unwrap();
        let doc = Document::new(font_family);
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

        for rows in self.labels.chunks(4) {
            let mut row = table.row();
            for (i, label) in rows.iter().enumerate() {
                let margin = match i {
                    0 => 0.0,
                    1 => 2.0,
                    2 => 4.0,
                    _ => 6.0,
                };
                row.push_element(create_padded_element(
                    AveryLabel::new(CableLabel::default()),
                    margin,
                ));
            }

            let rem = 4 - rows.len();

            for _ in 0..rem {
                row.push_element(AveryEmptyLabel::default());
            }

            row.push().unwrap();
        }

        fn create_padded_element<E: genpdf::Element>(label: E, margin: f32) -> PaddedElement<E> {
            PaddedElement::new(label, Margins::trbl(0.0, 0.0, 0.0, margin))
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

    pub fn write_to_file(mut self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        self.render();
        self.document.render_to_file(path).unwrap();
        Ok(())
    }
}
