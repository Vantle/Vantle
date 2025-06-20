#[cfg(test)]
mod tests {
    mod view {
        use std::io::{Cursor, Read};
        use symbolic::translator::{rule, Translation};
        use utility::logging::debug;

        #[test]
        pub fn quantity() {
            let source = "This is a viewable string";
            let mut cursor = Cursor::new(source.as_bytes());

            let viewed = Translation::rules()
                .limiter(4)
                .view(cursor.by_ref())
                .expect("Failed to get initial view");

            debug!("{:?}", viewed);

            assert_eq!(
                viewed.initial(),
                0,
                "Initial position {:?} should be 0.",
                viewed.initial()
            );

            assert_eq!(
                viewed.terminal(),
                4,
                "Terminal position {:?} should be 4.",
                viewed.terminal()
            );

            let repeated = Translation::rules()
                .limiter(viewed.elements().len())
                .view(cursor.by_ref())
                .expect("Failed to get repeated view");

            debug!("{:?}", repeated);

            assert_eq!(viewed, repeated, "Viewed and repeated should be equal.");
        }

        #[test]
        pub fn termination() {
            let string = "This is a viewable string";
            let mut cursor = Cursor::new(string.as_bytes());

            let predicated = Translation::rules()
                .terminator(rule::is(b'i'))
                .view(cursor.by_ref())
                .expect("Failed to get predicated view");

            debug!("{:?}", predicated);

            assert_eq!(
                predicated.initial(),
                0,
                "Initial position {:?} should be 0.",
                predicated.initial()
            );

            assert_eq!(
                predicated.terminal(),
                2,
                "Terminal position {:?} should be 2.",
                predicated.terminal()
            );

            assert_eq!(
                *predicated.characterize().elements(),
                ['T', 'h'],
                "Translation {:?} should be ['T', 'h']",
                *predicated.characterize().elements()
            );

            let reviewed = Translation::rules()
                .limiter(predicated.elements().len())
                .view(cursor.by_ref())
                .expect("Failed to get reviewed predicated view");

            debug!("{:?}", reviewed);

            assert_eq!(
                reviewed.terminal(),
                2,
                "Reviewed terminal position {:?} should be 2",
                reviewed.terminal()
            );
        }
    }

    mod consume {
        use std::io::{Cursor, Read};
        use symbolic::translator::{rule, Translation};
        use utility::logging::debug;

        #[test]
        pub fn quantity() {
            let string = "This is a viewable string";
            let mut test = Cursor::new(string.as_bytes());

            let viewed = Translation::rules()
                .limiter(4)
                .consume(test.by_ref())
                .expect("Failed to consume initial view");

            debug!("{:?}", viewed);

            assert_eq!(
                viewed.initial(),
                0,
                "Initial position {:?} should be 0",
                viewed.initial()
            );

            assert_eq!(
                viewed.terminal(),
                4,
                "Terminal position {:?} should be 4",
                viewed.terminal()
            );

            assert_eq!(
                *viewed.characterize().elements(),
                ['T', 'h', 'i', 's'],
                "Elements {:?} should be ['T', 'h', 'i', 's']",
                *viewed.characterize().elements()
            );

            let repeated = Translation::rules()
                .limiter(viewed.elements().len())
                .view(test.by_ref())
                .expect("Failed to view repeated view elements");

            debug!("{:?}", repeated);

            assert_eq!(
                repeated.initial(),
                4,
                "Repeated initial position {:?} should be 4",
                repeated.initial()
            );

            assert_eq!(
                repeated.terminal(),
                8,
                "Repeated terminal position {:?} should be 8",
                repeated.terminal()
            );

            assert_ne!(
                viewed, repeated,
                "Viewed {:?} and repeated {:?} should not be equal",
                viewed, repeated
            );

            assert_eq!(
                *repeated.characterize().elements(),
                [' ', 'i', 's', ' '],
                "Elements {:?} should be [' ', 'i', 's', ' ']",
                *repeated.characterize().elements()
            );
        }

        #[test]
        pub fn termination() {
            let string = "This is a consumable string";
            let mut cursor = Cursor::new(string.as_bytes());

            let predicated = Translation::rules()
                .terminator(rule::is(b'i'))
                .consume(cursor.by_ref())
                .expect("Failed to consume with predicate");

            debug!("{:?}", predicated);

            assert_eq!(
                predicated.initial(),
                0,
                "Initial position {:?} should be 0",
                predicated.initial()
            );
            assert_eq!(
                predicated.terminal(),
                2,
                "Terminal position {:?} should be 2",
                predicated.terminal()
            );
            assert_eq!(
                *predicated.characterize().elements(),
                ['T', 'h'],
                "Elements {:?} should be ['T', 'h']",
                *predicated.characterize().elements()
            );

            let reviewed = Translation::rules()
                .limiter(predicated.elements().len())
                .view(cursor.by_ref())
                .expect("Failed to view repeated elements");

            debug!("{:?}", reviewed);

            assert_eq!(
                reviewed.initial(),
                2,
                "Reviewed initial position {:?} should be 2",
                reviewed.initial()
            );
            assert_eq!(
                reviewed.terminal(),
                4,
                "Reviewed terminal position {:?} should be 4",
                reviewed.terminal()
            );
            assert_eq!(
                *reviewed.characterize().elements(),
                ['i', 's'],
                "Elements {:?} should be ['i', 's']",
                *reviewed.characterize().elements()
            );
        }

        #[test]
        pub fn space() {
            let string = "    \n\t\n \rThis is a consumable string";
            let mut cursor = Cursor::new(string.as_bytes());

            let predicated = Translation::rules()
                .terminator(rule::glyph())
                .consume(cursor.by_ref())
                .expect("Failed to consume with glyph termination");

            debug!("{:?}", predicated);

            assert_eq!(
                predicated.initial(),
                0,
                "Initial position {:?} should be 0",
                predicated.initial()
            );
            assert_eq!(
                predicated.terminal(),
                9,
                "Terminal position {:?} should be 9",
                predicated.terminal()
            );
            assert_eq!(
                *predicated.characterize().elements(),
                [' ', ' ', ' ', ' ', '\n', '\t', '\n', ' ', '\r'],
                "Elements {:?} should be [' ', ' ', ' ', ' ', '\\n', '\\t', '\\n', ' ', '\\r']",
                *predicated.characterize().elements()
            );

            let viewed = Translation::rules()
                .view(cursor.by_ref())
                .expect("Failed to view elements");

            debug!("{:?}", viewed);

            assert_eq!(
                viewed.initial(),
                9,
                "Reviewed initial position {:?} should be 9",
                viewed.initial()
            );
            assert_eq!(
                viewed.terminal(),
                36,
                "Reviewed terminal position {:?} should be 36",
                viewed.terminal()
            );
            assert_eq!(
                *viewed.characterize().elements(),
                [
                    'T', 'h', 'i', 's', ' ', 'i', 's', ' ', 'a', ' ', 'c', 'o', 'n', 's', 'u', 'm', 'a',
                    'b', 'l', 'e', ' ', 's', 't', 'r', 'i', 'n', 'g'
                ],
                    "Elements {:?} should be ['T', 'h', 'i', 's', ' ', 'i', 's', ' ', 'a', ' ', 'c', 'o', 'n', 's', 'u', 'm', 'a', 'b', 'l', 'e', ' ', 's', 't', 'r', 'i', 'n', 'g']",
                    *viewed.characterize().elements()
                );
        }
    }

    mod translators {
        mod view {
            use std::io::{Cursor, Read};
            use symbolic::translator::{rule, Translation};
            use utility::logging::debug;

            #[test]
            pub fn all() {
                let viewable = "    \n\t\n \rThis is a consumable string";
                let mut viewer = Cursor::new(viewable.as_bytes());

                let alpha = Translation::rules()
                    .view(viewer.by_ref())
                    .expect("Alpha view should have succeeded");

                debug!("{:?}", alpha);

                let beta = Translation::rules()
                    .view(viewer.by_ref())
                    .expect("Beta view should have succeeded");

                debug!("{:?}", beta);

                assert_eq!(
                    alpha,
                    beta,
                    "Alpha {:?} and Beta {:?} should yield the same result on the same strings.",
                    alpha.characterize(),
                    beta.characterize()
                );
            }

            #[test]
            pub fn some() {
                let viewable = "    \n\t\n \rThis is a consumable string";
                let mut viewer = Cursor::new(viewable.as_bytes());

                let alpha = Translation::rules()
                    .terminator(rule::glyph())
                    .view(viewer.by_ref())
                    .expect("Alpha view should have succeeded");

                debug!("{:?}", alpha);

                let beta = Translation::rules()
                    .terminator(rule::glyph())
                    .view(viewer.by_ref())
                    .expect("Beta view should have succeeded");

                debug!("{:?}", beta);

                assert_eq!(
                    alpha,
                    beta,
                    "Alpha {:?} and Beta {:?} should yield the same result on the same strings.",
                    alpha.characterize(),
                    beta.characterize()
                );
            }

            #[test]
            pub fn filter() {
                let viewable = "    \n\t\n \rThis is a consumable string";
                let characters = "Thisisaconsumablestring";
                let mut viewer = Cursor::new(viewable.as_bytes());

                let alpha = Translation::rules()
                    .filter(rule::glyph())
                    .view(viewer.by_ref())
                    .expect("Alpha view should have succeeded");

                debug!("{:?}", alpha);

                assert_eq!(
                    alpha.characterize().elements().iter().collect::<String>(),
                    characters,
                    "Alpha {:?} should be equal to characters {:?}",
                    alpha.characterize(),
                    characters
                );

                let beta = Translation::rules()
                    .filter(rule::glyph())
                    .view(viewer.by_ref())
                    .expect("Beta view should have succeeded");

                debug!("{:?}", beta);

                assert_eq!(
                    alpha,
                    beta,
                    "Alpha {:?} and Beta {:?} should yield the same result on the same strings.",
                    alpha.characterize(),
                    beta.characterize()
                );
            }
        }

        mod consume {
            use std::io::{Cursor, Read};
            use symbolic::translator::{rule, Translation};
            use utility::logging::debug;

            #[test]
            pub fn all() {
                let viewable = "    \n\t\n \rThis is a consumable string";
                let mut viewer = Cursor::new(viewable.as_bytes());

                let alpha = Translation::rules()
                    .consume(viewer.by_ref())
                    .expect("Alpha view should have succeeded")
                    .characterize()
                    .elements()
                    .iter()
                    .collect::<String>();

                debug!("{:?}", alpha);

                assert_eq!(
                    alpha, viewable,
                    "Alpha {:?} and Viewable {:?} should be the same strings.",
                    alpha, viewable
                );

                let beta = Translation::rules()
                    .view(viewer.by_ref())
                    .expect("Beta view should have succeeded");

                debug!("{:?}", beta);

                assert_eq!(
                    beta.length(),
                    0,
                    "Beta should not have any elements but had {:?}",
                    beta.characterize()
                );
            }

            #[test]
            pub fn some() {
                let viewable = "    \n\t\n \rThis is a consumable string";
                let partial = "    \n\t\n \r";
                let remaining = "This is a consumable string";
                let mut viewer = Cursor::new(viewable.as_bytes());

                let alpha = Translation::rules()
                    .terminator(rule::is(b'T'))
                    .consume(viewer.by_ref())
                    .expect("Alpha consume should have succeeded")
                    .characterize()
                    .elements()
                    .iter()
                    .collect::<String>();

                debug!("{:?}", alpha);

                assert_eq!(
                    alpha, partial,
                    "Alpha {:?} and Partial {:?} should be the same strings.",
                    alpha, partial
                );

                let beta = Translation::rules()
                    .view(viewer.by_ref())
                    .expect("Beta view should have succeeded")
                    .characterize()
                    .elements()
                    .iter()
                    .collect::<String>();

                debug!("{:?}", beta);

                assert_eq!(
                    beta, remaining,
                    "Beta {:?} should equal the remaining elements {:?}",
                    beta, remaining
                );
            }

            #[test]
            pub fn filter() {
                let viewable = "    \n\t\n \rThis is a consumable string";
                let filtered = "iii";
                let mut viewer = Cursor::new(viewable.as_bytes());

                let alpha = Translation::rules()
                    .filter(rule::is(b'i'))
                    .view(viewer.by_ref())
                    .expect("Alpha view should have succeeded")
                    .characterize()
                    .elements()
                    .iter()
                    .collect::<String>();

                debug!("{:?}", alpha);

                assert_eq!(
                    alpha, filtered,
                    "Alpha {:?} and Filtered {:?} should be the same strings.",
                    alpha, filtered
                );
            }
        }
    }
}
