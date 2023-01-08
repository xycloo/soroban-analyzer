macro_rules! color {
    ( $stdout: ident, $color: ident) => {
        $stdout.set_color(ColorSpec::new().set_fg(Some(Color::$color)))?;
    };
    ( $stdout: ident, $color: ident, $intense: ident) => {
        $stdout.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::$color))
                .set_intense($intense),
        )?;
    };
}
