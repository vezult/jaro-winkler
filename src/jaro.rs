use std::cmp;

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn similarity(s1: &str, s2: &str) -> (f32, f32, f32, f32, f32) {
    // Max distance that can be considered a match
    let s1_len = s1.len();
    let s2_len = s2.len();
    let max_trans = (cmp::max(s1_len, s2_len) / 2) - 1;
    dbg!((cmp::max(&s1_len, &s2_len) / 2) - 1);
    let mut match_count = 0;
    let mut trans_count = 0;
    let mut prefix_len = 0;
    let mut last_match_idx = 0;
    for (idx, c1) in s1.char_indices() {
        let s2_chars = s2.char_indices();

        for (s2_idx, c2) in s2_chars {
            if c1 == c2 {
                let idx_delta = abs_diff(idx, s2_idx);
                // Same position
                if idx_delta == 0 {
                    dbg!(c1, idx, s2_idx, "match exact");
                    // Matching prefix
                    if prefix_len < 4 && idx == prefix_len {
                        prefix_len += 1;
                    }
                    match_count += 1;
                    last_match_idx = s2_idx;
                    break;
                // Match within allowed range
                } else if idx_delta <= max_trans {
                    // Out of order match
                    if s2_idx < last_match_idx {
                        dbg!(c1, idx, s2_idx, "match trans");
                        trans_count += 1;
                    } else if idx < last_match_idx {
                        dbg!(c1, idx, s2_idx, "match");
                        trans_count += 1;
                    } else {
                        dbg!(c1, idx, s2_idx, "match trans??");
                        // This makes almost all tests fail
                        //trans_count += 1;
                    }
                    match_count += 1;
                    last_match_idx = s2_idx;
                    break;
                } else {
                    // Char exists and is in order, but is out of
                    // match range
                    if s2_idx < last_match_idx {
                        last_match_idx = s2_idx;
                        dbg!(c1, idx, s2_idx, "trans");
                        trans_count += 1;
                    } else {
                        dbg!(c1, idx, s2_idx, "??");
                    }
                }
            }
        }
    }

    let m = match_count as f32;
    let s1_len = s1.len() as f32;
    let s2_len = s2.len() as f32;
    let t = (trans_count / 2) as f32;
    let l = prefix_len as f32;

    (s1_len, s2_len, m, t, l)
}

fn jaro_score(s1: f32, s2: f32, m: f32, t: f32) -> f32 {
    if m > 0.0 {
        dbg!(s1, s2, m, t);
        dbg!(((m / s1) + (m / s2) + ((m - t) / m)) / 3.0);
        ((m / s1) + (m / s2) + ((m - t) / m)) / 3.0
    } else {
        0.0
    }
}

pub fn jaro(string1: &str, string2: &str) -> f32 {
    let (s1, s2, m, t, _) = similarity(string1, string2);

    jaro_score(s1, s2, m, t)
}

pub fn winkler(string1: &str, string2: &str) -> f32 {
    let (s1, s2, m, t, l) = similarity(string1, string2);
    let sim_j = jaro_score(s1, s2, m, t);

    // Prefix weight
    let p = 0.1;

    dbg!(&p);
    dbg!(&l);
    dbg!(sim_j + (p * l * (1.0 - sim_j)));
    sim_j + (p * l * (1.0 - sim_j))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn jaro_identical() -> () {
        let s1 = "aaaa".to_string();
        let s2 = "aaaa".to_string();
        assert_eq!(jaro(&s1, &s2), 1.0);
    }

    #[test]
    fn jaro_transposition() -> () {
        let s1 = "crate".to_string();
        let s2 = "trace".to_string();
        assert_eq!(jaro(&s1, &s2), 0.73333335);

        let s1 = "dixon".to_string();
        let s2 = "dicksonx".to_string();
        assert_eq!(jaro(&s1, &s2), 0.76666666);
    }

    #[test]
    fn jaro_transpositional_match_order_preserved() -> () {
        let s1 = "dwayne".to_string();
        let s2 = "duane".to_string();
        assert_eq!(jaro(&s1, &s2), 0.82222223);
    }

    #[test]
    fn jaro_transpositional_match_order_not_preserved() -> () {
        let s1 = "martha".to_string();
        let s2 = "marhta".to_string();
        assert_eq!(jaro(&s1, &s2), 0.94444444);
    }

    #[test]
    fn jaro_substitution() -> () {
        let s1 = "jellyfish".to_string();
        let s2 = "smellyfish".to_string();
        assert_eq!(jaro(&s1, &s2), 0.8962963);
    }

    #[test]
    fn jaro_score_indepentent_of_dissimilar_position() -> () {
        let s1 = "abcdefg".to_string();
        let s2_1 = "abcdefx".to_string();
        let s2_2 = "abcdexg".to_string();
        let s2_3 = "xbcdefg".to_string();

        let score_1 = jaro(&s1, &s2_1);
        let score_2 = jaro(&s1, &s2_2);
        let score_3 = jaro(&s1, &s2_3);
        assert_eq!(score_1, score_2);
        assert_eq!(score_1, score_3);
    }

    #[test]
    fn jaro_winkler_identical() -> () {
        let s1 = "aaaa".to_string();
        let s2 = "aaaa".to_string();
        assert_eq!(winkler(&s1, &s2), 1.0);
    }

    #[test]
    fn jaro_winkler_transposition() -> () {
        let s1 = "crate".to_string();
        let s2 = "trace".to_string();
        assert_eq!(winkler(&s1, &s2), 0.73333335);
    }

    #[test]
    fn jaro_winkler_transpositional_match() -> () {
        let s1 = "dwayne".to_string();
        let s2 = "duane".to_string();
        assert_eq!(winkler(&s1, &s2), 0.84000003);
    }

    #[test]
    fn jaro_winkler_prefix_weight() -> () {
        let s1 = "abcdx".to_string();
        // Same number of matcing characters, but the
        // second has a longer matching prefix
        let s2_1 = "abfcd".to_string();
        let s2_2 = "abcfd".to_string();

        let score_1 = winkler(&s1, &s2_1);
        let score_2 = winkler(&s1, &s2_2);
        dbg!(&score_1, &score_2);
        assert!(score_1 < score_2);
    }

    #[test]
    fn jaro_winkler_test_cases() -> () {
        // From Winkler paper
        let tests: [(&str, &str, f32); 17] = [
            ("SHACKLEFORD", "SHACKELFORD", 0.982),
            ("DUNNINGHAM", "CUNNIGHAM", 0.896),
            ("NICHLESON", "NICHULSON", 0.956),
            ("JONES", "JOHNSON", 0.832),
            ("MASSEY", "MASSIE", 0.933),
            ("ABROMS", "ABRAMS", 0.922),
            ("HARDIN", "MARTINEZ", 0.000),
            ("ITMAN", "SMITH", 0.000),
            ("JERALDINE", "GERALDINE", 0.926),
            ("MARHTA", "MARTHA", 0.961),
            ("MICHELLE", "MICHAEL", 0.921),
            ("JULIES", "JULIUS", 0.933),
            ("TANYA", "TONYA", 0.880),
            ("DWAYNE", "DUANE", 0.840),
            ("SEAN", "SUSAN", 0.805),
            ("JON", "JOHN", 0.933),
            ("JON", "JAN", 0.000),
        ];

        for (s1, s2, score) in tests.iter() {
            let wscore = winkler(&s1.to_string(), &s2.to_string());
            let score_round = (wscore * 1000.0).round() / 1000.0;

            dbg!(s1, s2);
            dbg!(&score_round, score);

            assert!(score_round == *score);
        }
    }

    #[test]
    fn foo() -> () {
        let mut b: BTreeSet<char> = BTreeSet::new();

        b.insert('a');
        assert!(b.contains(&'a') == true);
        assert!(b.contains(&'b') == false);
    }
}
