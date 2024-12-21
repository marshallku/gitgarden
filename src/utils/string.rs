pub fn minify_xml(s: &str) -> String {
    let without_linebreak = s.replace("\n", "");
    let mut minified = String::with_capacity(without_linebreak.len());
    let mut space_count = 0;
    let mut prev_char = '\0';
    let chars = without_linebreak.chars().collect::<Vec<_>>();

    for (i, &c) in chars.iter().enumerate() {
        let next_char = chars.get(i + 1).unwrap_or(&'\0');
        if c == ' ' {
            if prev_char == '>'
                || prev_char == '{'
                || prev_char == '}'
                || prev_char == ';'
                || prev_char == ':'
                || *next_char == '<'
                || *next_char == '{'
                || *next_char == '}'
                || *next_char == ';'
                || *next_char == ':'
            {
                continue;
            }

            space_count += 1;
            if space_count == 1 {
                minified.push(c);
            }
        } else {
            space_count = 0;
            minified.push(c);
            prev_char = c;
        }
    }

    minified
}
