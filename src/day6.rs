use std::collections::{HashSet, VecDeque};

enum Marker {
    StartOfPacket = 4,
    StartOfMessage = 14,
}

pub fn day_main(filename: &str) {
    let f = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut stream = f.chars();
    match find_marker_start(&mut stream, Marker::StartOfMessage) {
        Some(marker_start) => {
            println!("Marker starts at index {}", marker_start);
        }
        None => {
            println!("No marker found!");
        }
    }
}

fn find_marker_start(
    stream: &mut impl Iterator<Item = char>,
    marker_type: Marker,
) -> Option<usize> {
    let marker_len = marker_type as usize;
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(marker_len);
    let mut check: HashSet<&char> = HashSet::with_capacity(marker_len);
    let mut e = stream.enumerate();
    while let Some((idx, chr)) = e.next() {
        if buffer.len() == marker_len {
            buffer.pop_front();
        }
        buffer.push_back(chr);
        check = buffer.iter().collect();
        if check.len() == marker_len {
            return Some(idx + 1);
        }
        check.clear();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases taken from examples
    #[test]
    fn first() {
        assert_eq!(
            find_marker_start(
                &mut "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars(),
                Marker::StartOfPacket
            ),
            Some(7)
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            find_marker_start(
                &mut "bvwbjplbgvbhsrlpgdmjqwftvncz".chars(),
                Marker::StartOfPacket
            ),
            Some(5)
        );
    }

    #[test]
    fn third() {
        assert_eq!(
            find_marker_start(
                &mut "nppdvjthqldpwncqszvftbrmjlhg".chars(),
                Marker::StartOfPacket
            ),
            Some(6)
        );
    }

    #[test]
    fn forth() {
        assert_eq!(
            find_marker_start(
                &mut "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars(),
                Marker::StartOfPacket
            ),
            Some(10)
        );
    }

    #[test]
    fn fifth() {
        assert_eq!(
            find_marker_start(
                &mut "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars(),
                Marker::StartOfPacket
            ),
            Some(11)
        );
    }

    #[test]
    fn sixth() {
        assert_eq!(
            find_marker_start(
                &mut "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars(),
                Marker::StartOfMessage
            ),
            Some(19)
        );
    }

    #[test]
    fn seventh() {
        assert_eq!(
            find_marker_start(
                &mut "bvwbjplbgvbhsrlpgdmjqwftvncz".chars(),
                Marker::StartOfMessage
            ),
            Some(23)
        );
    }

    #[test]
    fn eighth() {
        assert_eq!(
            find_marker_start(
                &mut "nppdvjthqldpwncqszvftbrmjlhg".chars(),
                Marker::StartOfMessage
            ),
            Some(23)
        );
    }
    #[test]
    fn ninth() {
        assert_eq!(
            find_marker_start(
                &mut "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars(),
                Marker::StartOfMessage
            ),
            Some(29)
        );
    }
    #[test]
    fn tenth() {
        assert_eq!(
            find_marker_start(
                &mut "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars(),
                Marker::StartOfMessage
            ),
            Some(26)
        );
    }
    #[test]
    fn none_check() {
        assert_eq!(
            find_marker_start(
                &mut "abcabcabcabcabcabcabcabc".chars(),
                Marker::StartOfPacket
            ),
            None
        );
    }
}
