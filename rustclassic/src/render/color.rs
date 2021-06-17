pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[macro_export]
macro_rules! color {
    (hex $col:literal) => {{
        Color(
            (($col >> 24) & 0xff) as u8,
            (($col >> 16) & 0xff) as u8,
            (($col >> 8) & 0xff) as u8,
            ($col & 0xff) as u8,
        )
    }};
    ($r:literal, $g:literal, $b:literal) => {
        Color($r, $g, $b, 0xff)
    };
    ($r:literal, $g:literal, $b:literal, $a:literal) => {
        Color($r, $g, $b, $a)
    };
}
