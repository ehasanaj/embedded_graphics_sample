use std::sync::{Arc, Mutex};

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle, Styled},
    text::{Alignment, Text},
};

#[derive(Debug)]
pub enum DrawableElement<'a, T: PixelColor> {
    Rectangle(Styled<Rectangle, PrimitiveStyle<T>>),
    Circle(Styled<Circle, PrimitiveStyle<T>>),
    Text(Text<'a, MonoTextStyle<'a, T>>),
    Line(Styled<Line, PrimitiveStyle<T>>),
    Arc(Styled<embedded_graphics::primitives::Arc, PrimitiveStyle<T>>),
}

impl<'a, T: PixelColor> Drawable for DrawableElement<'a, T> {
    type Color = T;
    type Output = ();

    fn draw<D: DrawTarget<Color = Self::Color>>(
        &self,
        display: &mut D,
    ) -> Result<Self::Output, D::Error> {
        match self {
            DrawableElement::Rectangle(rectangle) => rectangle.draw(display),
            DrawableElement::Text(text) => {
                text.draw(display)?;
                Ok(())
            }
            DrawableElement::Circle(circle) => circle.draw(display),
            DrawableElement::Line(line) => line.draw(display),
            DrawableElement::Arc(arc) => arc.draw(display),
        }
    }
}

pub struct Renderer<'a, T: PixelColor> {
    elements: Arc<Mutex<Vec<DrawableElement<'a, T>>>>,
}

impl<'a, T: PixelColor> Renderer<'a, T> {
    pub fn new() -> Renderer<'a, T> {
        Renderer {
            elements: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_element(&self, element: DrawableElement<'a, T>) {
        let elements = Arc::clone(&self.elements);
        let mut elements = elements.lock().unwrap();
        elements.push(element);
    }

    pub fn add_text_with_alignment(
        &self,
        text: &'a str,
        position: Point,
        style: MonoTextStyle<'a, T>,
        alignment: Alignment,
    ) {
        let text = Text::with_alignment(text, position, style, alignment);
        self.add_element(text.into());
    }

    pub fn add_rectangle(&self, rectangle: Rectangle, style: PrimitiveStyle<T>) {
        let styled_rectangle = rectangle.into_styled(style);
        self.add_element(styled_rectangle.into());
    }

    pub fn render<D: DrawTarget<Color = T>>(&self, display: &mut D) -> Result<(), D::Error> {
        self.elements
            .lock()
            .unwrap()
            .iter()
            .try_for_each(|element| {
                element.draw(display)?;
                Ok(())
            })
    }
}

impl<'a, T: PixelColor> From<Styled<Rectangle, PrimitiveStyle<T>>> for DrawableElement<'a, T> {
    fn from(styled_rect: Styled<Rectangle, PrimitiveStyle<T>>) -> Self {
        DrawableElement::Rectangle(styled_rect)
    }
}

impl<'a, T: PixelColor> From<Text<'a, MonoTextStyle<'a, T>>> for DrawableElement<'a, T> {
    fn from(text: Text<'a, MonoTextStyle<'a, T>>) -> Self {
        DrawableElement::Text(text)
    }
}

impl<'a, T: PixelColor> From<Styled<Circle, PrimitiveStyle<T>>> for DrawableElement<'a, T> {
    fn from(styled_circle: Styled<Circle, PrimitiveStyle<T>>) -> Self {
        DrawableElement::Circle(styled_circle)
    }
}

impl<'a, T: PixelColor> From<Styled<Line, PrimitiveStyle<T>>> for DrawableElement<'a, T> {
    fn from(styled_line: Styled<Line, PrimitiveStyle<T>>) -> Self {
        DrawableElement::Line(styled_line)
    }
}

impl<'a, T: PixelColor> From<Styled<embedded_graphics::primitives::Arc, PrimitiveStyle<T>>>
    for DrawableElement<'a, T>
{
    fn from(styled_arc: Styled<embedded_graphics::primitives::Arc, PrimitiveStyle<T>>) -> Self {
        DrawableElement::Arc(styled_arc)
    }
}
