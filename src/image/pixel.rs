#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Pixel(Color);

impl Pixel {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Rgba((u8, u8, u8, u8)),
    Rgb((u8, u8, u8)),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ColorType {
    Rgba,
    Rgb
}

impl Color {
    fn empty_rgba() -> Self {
        Color::Rgba((0, 0, 0, 0))
    }

    fn empty_rgb() -> Self {
        Color::Rgb((0, 0, 0))
    }

    fn apply_color(&mut self, color: Color, mode: ColorMode) -> Result<(), ()> {
        match self {
            Color::Rgb((r, g, b)) => {
                if let Color::Rgb((r1, g1, b1)) = color {
                    *self = Color::Rgb((
                        mode.apply_op(r, &r1),
                        mode.apply_op(g, &g1),
                        mode.apply_op(b, &b1),
                    ));
                    return Ok(());
                } else {
                    return Err(());
                }
            },
            Color::Rgba((r, g, b, a)) => {
                if let Color::Rgba((r1, g1, b1, a1)) = color {
                    *self = Color::Rgba((
                        mode.apply_op(r, &r1),
                        mode.apply_op(g, &g1),
                        mode.apply_op(b, &b1),
                        mode.apply_op(a, &a1),
                    ));

                    return Ok(());
                } else {
                    return Err(());
                }
            },
        };
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ColorMode {
    Add,
    Subtract,
}

impl ColorMode {
    fn apply_op(&self, a: &u8, b: &u8) -> u8 {
        match self {
            ColorMode::Add => a + b,
            ColorMode::Subtract => a - b,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply_color() {
        let mut col1 = Color::Rgb((10, 10, 10));
        let col2 = Color::Rgb((1, 2, 3));

        assert_eq!(
            col1.apply_color(col2, ColorMode::Add),
            Ok(())
        );
        assert_eq!(col1, Color::Rgb((11, 12, 13)));

        assert_eq!(
            col1.apply_color(col2, ColorMode::Subtract),
            Ok(())
        );
        assert_eq!(col1, Color::Rgb((10, 10, 10)));

        let col3 = Color::Rgba((0, 0, 0, 0));
        assert_eq!(col1.apply_color(col3, ColorMode::Add), Err(()));
    }
}
