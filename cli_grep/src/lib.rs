pub fn find_matches(content: &str, pattern: &str, mut writter: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writter, "{}", line);
        }
    }
}
