pub fn minify_xml(s: &str) -> String {
    let simply_minified = s.replace("\n", "");
    let simply_minified = simply_minified.trim();
    let mut minified = String::with_capacity(simply_minified.len());
    let mut space_count = 0;
    let mut prev_char = '\0';

    for (i, c) in simply_minified.chars().enumerate() {
        let next_char = simply_minified.chars().nth(i + 1).unwrap_or('\0');

        if c == ' ' {
            space_count += 1;
            if space_count == 1
                && prev_char != '>'
                && next_char != '<'
                && prev_char != '='
                && next_char != '='
                && prev_char != ':'
                && next_char != ':'
                && prev_char != ';'
                && next_char != ';'
                && prev_char != '{'
                && next_char != '}'
                && prev_char != '}'
                && next_char != '{'
            {
                minified.push(c);
            }
        } else {
            space_count = 0;
            minified.push(c);
        }

        prev_char = c;
    }

    minified
}
