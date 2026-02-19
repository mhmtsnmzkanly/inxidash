// Responsibility: Provide ANSI stripping helpers consumed by the parser and other text-processing services.
// Design reasoning: Custom stripping avoids pulling in regex-heavy crates, keeping the binary lean and deterministic.
// Extension guidance: Update the parser to track additional control sequences if new inxi versions emit them.
// Security considerations: Avoid reconstructing untrusted control sequences to prevent terminal escape injection.

pub fn strip_ansi(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\x1b' => {
                // Skip common ANSI escape sequences (CSI / OSC).
                if let Some(next) = chars.peek() {
                    if *next == '[' {
                        chars.next();
                        while let Some(csi) = chars.next() {
                            if ('@'..='~').contains(&csi) {
                                break;
                            }
                        }
                        continue;
                    }
                    if *next == ']' {
                        chars.next();
                        while let Some(osc) = chars.next() {
                            if osc == '\x07' {
                                break;
                            }
                            if osc == '\x1b' && chars.peek() == Some(&'\\') {
                                chars.next();
                                break;
                            }
                        }
                        continue;
                    }
                    chars.next();
                }
            }
            '\x03' => {
                // Strip mIRC color codes used by inxi output.
                consume_digits(&mut chars, 2);
                if chars.peek() == Some(&',') {
                    chars.next();
                    consume_digits(&mut chars, 2);
                }
                continue;
            }
            // Strip mIRC formatting controls.
            '\x02' | '\x0f' | '\x16' | '\x1d' | '\x1f' => continue,
            // Keep structural whitespace, drop other control bytes.
            c if c.is_control() && c != '\n' && c != '\r' && c != '\t' => continue,
            _ => output.push(ch),
        }
    }

    output
}

fn consume_digits<I>(chars: &mut std::iter::Peekable<I>, max: usize)
where
    I: Iterator<Item = char>,
{
    let mut seen = 0usize;
    while seen < max {
        match chars.peek() {
            Some(c) if c.is_ascii_digit() => {
                chars.next();
                seen += 1;
            }
            _ => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::strip_ansi;

    #[test]
    fn strips_csi_sequences() {
        let input = "\x1b[32mhello\x1b[0m world";
        assert_eq!(strip_ansi(input), "hello world");
    }

    #[test]
    fn strips_mirc_color_sequences() {
        let input = "\x0312System:\x03\n  \x0312Kernel\x03 6.12";
        assert_eq!(strip_ansi(input), "System:\n  Kernel 6.12");
    }

    #[test]
    fn keeps_line_breaks() {
        let input = "\x0312CPU:\x03\r\n  Info quad core";
        assert_eq!(strip_ansi(input), "CPU:\r\n  Info quad core");
    }
}
