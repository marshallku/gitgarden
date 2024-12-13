pub fn minify_xml(s: &str) -> String {
    let without_linebreak = s.replace("\n", "");
    let mut minified = String::with_capacity(without_linebreak.len());
    let mut space_count = 0;

    for c in without_linebreak.chars() {
        if c == ' ' {
            space_count += 1;
            if space_count == 1 {
                minified.push(c);
            }
        } else {
            space_count = 0;
            minified.push(c);
        }
    }

    minified
}
