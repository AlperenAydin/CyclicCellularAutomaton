// This modules returns an appropirate color_map for a configuration
// Currently only the num_states is taken into account

static BASE_COLORMAP: [u8; 18] = [
    0xB2, 0x1F, 0x35, 
    0xFF, 0x74, 0x35, 
    0xFF, 0xF7, 0x35, 
    0x16, 0xDD, 0x36, 
    0x00, 0x79, 0xE7, 
    0xBD, 0x7A, 0xF6,
];

pub fn get_colormap(num_states: u8) -> Vec<u8> {
    let mut colormap = vec![0; (num_states * 3) as usize];
    let base_num_states = BASE_COLORMAP.len();
    for i in 0..num_states {
        for j in 0..3 {
            let index = (i * 3 + j) as usize;
            colormap[index] = BASE_COLORMAP[index % base_num_states];
        }
    }
    colormap
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_colormap() {
        let ground_truth: Vec<u8> = vec![
            0xB2, 0x1F, 0x35, 
            0xFF, 0x74, 0x35, 
            0xFF, 0xF7, 0x35];
        let colormap = get_colormap(3);
        assert_eq!(ground_truth, colormap);

        let ground_truth: Vec<u8> = vec![
            0xB2, 0x1F, 0x35, 
            0xFF, 0x74, 0x35, 
            0xFF, 0xF7, 0x35, 
            0x16, 0xDD, 0x36, 
            0x00, 0x79, 0xE7, 
            0xBD, 0x7A, 0xF6];
        let colormap = get_colormap(6);
        assert_eq!(ground_truth, colormap);

        let ground_truth: Vec<u8> = vec![];
        let colormap = get_colormap(0);
        assert_eq!(ground_truth, colormap);
    }
}
