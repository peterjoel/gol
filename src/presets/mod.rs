

pub fn get_preset_from<'a>(data: &'a [u8]) -> impl Iterator<Item = (usize, usize)> + 'a {
    data.splitn(usize::max_value(), |&c| c == b'\n')
        .zip(0..)
        .flat_map(|(line, y)| line.into_iter()
            .zip(0..)
            .filter(|(&c, _)| c != b' ')
            .map(move |(_, x)| (x, y)))
}

pub fn get_preset(index: u8) -> impl Iterator<Item = (usize, usize)> {
    let shape: &[u8] = match index {
        1 => include_bytes!("glider_1.txt"),
        2 => include_bytes!("glider_2.txt"),
        3 => include_bytes!("glider_3.txt"),
        4 => include_bytes!("glider_4.txt"),
        0 => include_bytes!("glider_gun.txt"),
        _ => b"",
    };
    get_preset_from(shape)
}
