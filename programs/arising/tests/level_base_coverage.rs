//! Coverage for level curve constructors and civilization helpers.

use arising::codex::{level_from_experience, Civilization, LEVEL_CURVE};

#[test]
fn level_curve_entries_construct_and_resolve() {
    // Touch the const fn at runtime and ensure fields are set.
    let entry = arising::codex::level::LevelCurveEntry::new(1, 2, 3);
    assert_eq!(entry.level, 1);
    assert_eq!(entry.xp_to_next, 2);
    assert_eq!(entry.cumulative_xp, 3);

    // Ensure level_from_experience covers first and last entries.
    assert_eq!(level_from_experience(0), 1);
    let max_entry = LEVEL_CURVE.last().unwrap();
    assert!(level_from_experience(max_entry.cumulative_xp) >= max_entry.level);
}

#[test]
fn civilization_helpers_cover_all_variants() {
    for civ in [
        Civilization::Ard,
        Civilization::Hartenn,
        Civilization::Ikarans,
        Civilization::Zhand,
        Civilization::Shinkari,
        Civilization::Tarki,
    ] {
        assert!(civ.as_index() < Civilization::COUNT);
        assert!(!civ.slug().is_empty());
        assert!(!civ.display_name().is_empty());
        let uri = civ.metadata_uri(42);
        assert!(uri.contains(civ.slug()));
        assert_eq!(civ.seed_bytes()[0], civ as u8);
        // Touch base stats/attributes/passive.
        let _ = civ.base_stats();
        let _ = civ.base_attributes();
        let _ = civ.passive();
    }
}
