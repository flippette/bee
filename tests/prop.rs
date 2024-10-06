use bee::fits;
use proptest::{prelude::*, test_runner::FileFailurePersistence};

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(
            FileFailurePersistence::WithSource("cases")
        )),
        ..ProptestConfig::default()
    })]

    #[test]
    fn fits1(word in ".{0,256}") {
        let word = word.as_bytes();
        let set = word.iter().fold(vec![], |mut acc, ch| {
            if !acc.contains(ch) { acc.push(*ch); }
            acc
        });

        assert!(fits::<1>(word, &set));
    }

    #[test]
    fn fits2(word in ".{0,256}") {
        let word = word.as_bytes();
        let set = word.iter().fold(vec![], |mut acc, ch| {
            if !acc.contains(ch) { acc.push(*ch); }
            acc
        });

        assert!(fits::<2>(word, &set));
    }

    #[test]
    fn fits4(word in ".{0,256}") {
        let word = word.as_bytes();
        let set = word.iter().fold(vec![], |mut acc, ch| {
            if !acc.contains(ch) { acc.push(*ch); }
            acc
        });

        assert!(fits::<4>(word, &set));
    }

    #[test]
    fn fits8(word in ".{0,256}") {
        let word = word.as_bytes();
        let set = word.iter().fold(vec![], |mut acc, ch| {
            if !acc.contains(ch) { acc.push(*ch); }
            acc
        });

        assert!(fits::<8>(word, &set));
    }

    #[test]
    fn fits16(word in ".{0,256}") {
        let word = word.as_bytes();
        let set = word.iter().fold(vec![], |mut acc, ch| {
            if !acc.contains(ch) { acc.push(*ch); }
            acc
        });

        assert!(fits::<16>(word, &set));
    }

    #[test]
    fn fits32(word in ".{0,256}") {
        let word = word.as_bytes();
        let set = word.iter().fold(vec![], |mut acc, ch| {
            if !acc.contains(ch) { acc.push(*ch); }
            acc
        });

        assert!(fits::<32>(word, &set));
    }

    #[test]
    fn fits64(word in ".{0,256}") {
        let word = word.as_bytes();
        let set = word.iter().fold(vec![], |mut acc, ch| {
            if !acc.contains(ch) { acc.push(*ch); }
            acc
        });

        assert!(fits::<64>(word, &set));
    }
}
