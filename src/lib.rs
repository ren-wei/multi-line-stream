#[cfg(feature = "regex")]
use regex::Regex;

/// # MultiLineStream
/// Quick movement on multiple lines of text.
///
/// Indexes are measured in bytes
pub struct MultiLineStream<'a> {
    pub source: &'a str,
    position: usize,
}

impl MultiLineStream<'_> {
    pub fn new<'a>(source: &'a str, position: usize) -> MultiLineStream<'a> {
        MultiLineStream { source, position }
    }

    pub fn eos(&self) -> bool {
        self.source.len() <= self.position
    }

    pub fn pos(&self) -> usize {
        self.position
    }

    pub fn go_back(&mut self, n: usize) {
        self.position -= n;
    }

    pub fn advance(&mut self, n: usize) {
        self.position += n;
    }

    pub fn go_to_end(&mut self) {
        self.position = self.source.len();
    }

    pub fn peek_char(&self, n: isize) -> Option<u8> {
        let index = if n >= 0 {
            self.position + n as usize
        } else {
            self.position - (-n) as usize
        };
        Some(self.source.bytes().nth(index)?)
    }

    pub fn advance_if_char(&mut self, ch: u8) -> bool {
        if let Some(char) = self.source.bytes().nth(self.position) {
            if char == ch {
                self.position += 1;
                return true;
            }
        }
        false
    }

    pub fn advance_if_chars(&mut self, ch: &str) -> bool {
        if self.position + ch.len() > self.source.len() {
            return false;
        }

        if !self
            .source
            .get(self.position..self.position + ch.len())
            .is_some_and(|v| v == ch)
        {
            return false;
        }

        self.advance(ch.len());
        true
    }

    #[cfg(feature = "regex")]
    pub fn advance_if_regexp(&mut self, regexp: &Regex) -> Option<&str> {
        let haystack = &self.source[self.position..];
        let captures = regexp.captures(haystack)?;
        let m = captures.get(0).unwrap();
        self.position += m.end();
        Some(m.as_str())
    }

    #[cfg(feature = "regex")]
    pub fn advance_until_regexp(&mut self, regexp: &Regex) -> Option<&str> {
        let haystack = &self.source[self.position..];
        if let Some(captures) = regexp.captures(haystack) {
            let m = captures.get(0).unwrap();
            self.position += m.start();
            Some(m.as_str())
        } else {
            self.go_to_end();
            None
        }
    }

    pub fn advance_until_char(&mut self, ch: u8) -> bool {
        while self.position < self.source.len() {
            if self.source.bytes().nth(self.position) == Some(ch) {
                return true;
            }
            self.advance(1);
        }
        false
    }

    pub fn advance_until_chars(&mut self, ch: &str) -> bool {
        while self.position + ch.len() <= self.source.len() {
            if self
                .source
                .get(self.position..self.position + ch.len())
                .is_some_and(|v| v == ch)
            {
                return true;
            }
            self.advance(1);
        }
        self.go_to_end();
        false
    }

    pub fn skip_whitespace(&mut self) -> bool {
        let n = self.advance_while_char(|ch| vec![b' ', b'\t', b'\n', 12, b'\r'].contains(&ch));
        n > 0
    }

    pub fn advance_while_char<F>(&mut self, condition: F) -> usize
    where
        F: Fn(u8) -> bool,
    {
        let pos_now = self.position;
        while self.position < self.source.len()
            && condition(self.source.bytes().nth(self.position).unwrap())
        {
            self.advance(1);
        }
        self.position - pos_now
    }
}
