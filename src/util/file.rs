pub fn get_file_extension(filename: &str) -> &str {
    let mut last_dot_index = 0;
    for (i, c) in filename.char_indices() {
        if c == '.' {
            last_dot_index = i;
        }
    }
    &filename[last_dot_index + 1..]
}
