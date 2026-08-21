//! Integration tests for `MigrationRegistry<dyn UpMigration>`: apply up, no-op, errors, construction.

use std::sync::{Arc, Mutex};

use crate::{MigrationError, MigrationRegistry, UpMigration};
use pollster::FutureExt as _;

use super::common::{FakeMigrationTarget, FakeUpMigration, TestError, arc_up};

// ---------- Registry construction ----------

#[test]
fn registry_default_is_empty() {
    // Arrange: use Default for MigrationRegistry<dyn UpMigration<(), TestError>>
    // Act
    let registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::default();

    // Assert: apply with no migrations; get_versions returns (0, 0), so no apply_migration calls, version stays None
    let mut target = FakeMigrationTarget::new(None);
    let applied = registry.apply(&mut target, None).block_on();
    assert!(applied.is_ok());
    assert_eq!(target.current_version(), None);
}

#[test]
fn registry_new_is_empty() {
    // Arrange & Act
    let registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();

    // Assert: apply to latest with no migrations; no apply_migration calls, version stays None
    let mut target = FakeMigrationTarget::new(None);
    let result = registry.apply(&mut target, None).block_on();
    assert!(result.is_ok());
    assert_eq!(target.current_version(), None);
}

#[test]
fn registry_from_vec_sorts_by_version() {
    // Arrange: create migrations out of order (3, 1, 2)
    let log = Arc::new(Mutex::new(Vec::<(usize, bool)>::new()));
    let m3 = arc_up(FakeUpMigration::new(3, Arc::clone(&log)));
    let m1 = arc_up(FakeUpMigration::new(1, Arc::clone(&log)));
    let m2 = arc_up(FakeUpMigration::new(2, Arc::clone(&log)));
    let registry: MigrationRegistry<dyn UpMigration<(), TestError>> =
        MigrationRegistry::from(vec![m3, m1, m2]);

    // Act: apply to latest (3)
    let mut target = FakeMigrationTarget::new(None);
    let result = registry.apply(&mut target, None).block_on();

    // Assert: applied in order 1, 2, 3
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(3));
    assert_eq!(
        log.lock().unwrap().as_slice(),
        &[(1, true), (2, true), (3, true)]
    );
}

#[test]
fn add_migration_out_of_order_results_in_sorted_apply_order() {
    // Arrange: add migrations in order 2, 1, 3
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry
        .add(arc_up(FakeUpMigration::new(2, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(3, Arc::clone(&log))));

    // Act
    let mut target = FakeMigrationTarget::new(None);
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(
        log.lock().unwrap().as_slice(),
        &[(1, true), (2, true), (3, true)]
    );
}

// ---------- Apply up: current == target (no-op) ----------

#[test]
fn apply_when_current_equals_target_none_is_no_op() {
    // Arrange: target at 0, no migrations, target_version None -> current 0, target 0
    let registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    let mut target = FakeMigrationTarget::new(Some(0));

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(0));
}

#[test]
fn apply_when_current_equals_target_some_is_no_op() {
    // Arrange: one migration v1, target already at 1
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry.add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(1));

    // Act
    let result = registry.apply(&mut target, Some(1)).block_on();

    // Assert
    assert!(result.is_ok());
    assert!(log.lock().unwrap().is_empty());
    assert_eq!(target.current_version(), Some(1));
}

#[test]
fn apply_when_current_equals_latest_target_none_is_no_op() {
    // Arrange: migrations 1, 2, 3; target at 3
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry
        .add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(2, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(3, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(3));

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(result.is_ok());
    assert!(log.lock().unwrap().is_empty());
    assert_eq!(target.current_version(), Some(3));
}

// ---------- Apply up: from None (no migrations applied yet) ----------

#[test]
fn apply_from_none_to_latest_applies_all_migrations() {
    // Arrange
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry
        .add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(2, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(3, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(None);

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(3));
    assert_eq!(
        log.lock().unwrap().as_slice(),
        &[(1, true), (2, true), (3, true)]
    );
}

#[test]
fn apply_from_none_to_specific_target_applies_only_up_to_target() {
    // Arrange: migrations 1..5, target_version Some(3)
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    for v in 1..=5 {
        registry.add(arc_up(FakeUpMigration::new(v, Arc::clone(&log))));
    }
    let mut target = FakeMigrationTarget::new(None);

    // Act
    let result = registry.apply(&mut target, Some(3)).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(3));
    assert_eq!(
        log.lock().unwrap().as_slice(),
        &[(1, true), (2, true), (3, true)]
    );
}

#[test]
fn apply_migration_with_description_runs_successfully() {
    // Arrange: migration with description (output not asserted)
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry.add(arc_up(FakeUpMigration::with_description(
        1,
        "add_users_table",
        Arc::clone(&log),
    )));
    let mut target = FakeMigrationTarget::new(None);

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(1));
    assert_eq!(log.lock().unwrap().as_slice(), &[(1, true)]);
}

#[test]
fn apply_from_none_to_target_zero_leaves_none_and_applies_nothing() {
    // Arrange: get_current_version returns None -> current 0; target Some(0) -> target 0
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry.add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(None);

    // Act: target_version Some(0)
    let result = registry.apply(&mut target, Some(0)).block_on();

    // Assert: current 0, target 0 -> no-op; no apply_migration calls, version stays None
    assert!(result.is_ok());
    assert_eq!(target.current_version(), None);
    assert!(log.lock().unwrap().is_empty());
}

// ---------- Apply up: from some version to higher ----------

#[test]
fn apply_from_mid_to_latest_applies_only_pending() {
    // Arrange: current 1, migrations 1, 2, 3
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry
        .add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(2, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(3, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(1));

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(3));
    assert_eq!(log.lock().unwrap().as_slice(), &[(2, true), (3, true)]);
}

#[test]
fn apply_from_mid_to_mid_applies_only_in_range() {
    // Arrange: current 2, target Some(4), migrations 1..=5
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    for v in 1..=5 {
        registry.add(arc_up(FakeUpMigration::new(v, Arc::clone(&log))));
    }
    let mut target = FakeMigrationTarget::new(Some(2));

    // Act
    let result = registry.apply(&mut target, Some(4)).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(4));
    assert_eq!(log.lock().unwrap().as_slice(), &[(3, true), (4, true)]);
}

// ---------- UpMigration-only registry: target < current -> DownMigrationNotSupported ----------

#[test]
fn apply_target_lower_than_current_returns_down_migration_not_supported() {
    // Arrange: UpMigration-only registry, current 3, target Some(1)
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry
        .add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(2, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(3, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(3));

    // Act
    let result = registry.apply(&mut target, Some(1)).block_on();

    // Assert
    assert!(matches!(
        result,
        Err(MigrationError::DownMigrationNotSupported)
    ));
    assert_eq!(target.current_version(), Some(3));
    assert!(log.lock().unwrap().is_empty());
}

#[test]
fn apply_target_zero_when_current_nonzero_returns_down_migration_not_supported() {
    // Arrange
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry.add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(1));

    // Act
    let result = registry.apply(&mut target, Some(0)).block_on();

    // Assert
    assert!(matches!(
        result,
        Err(MigrationError::DownMigrationNotSupported)
    ));
    assert_eq!(target.current_version(), Some(1));
}

// ---------- Runner errors ----------

#[test]
fn get_current_version_error_propagates_as_runner_error() {
    // Arrange
    let registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    let mut target = FakeMigrationTarget::new(None);
    target
        .fail_get_version
        .set(Some(TestError::GetVersionFailed));

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(matches!(
        result,
        Err(MigrationError::RunnerError(TestError::GetVersionFailed))
    ));
}

#[test]
fn apply_migration_error_propagates_as_migration_failed() {
    // Arrange: one migration, target will fail on first apply
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry.add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(None);
    target.fail_apply.set(Some(TestError::ApplyFailed));

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    match &result {
        Err(MigrationError::MigrationFailed {
            id: 1,
            source: TestError::ApplyFailed,
        }) => {}
        other => panic!("expected MigrationFailed {{ id: 1, .. }}, got {:?}", other),
    }
    assert_eq!(target.current_version(), None);
}

#[test]
fn apply_migration_error_on_second_migration_stops_chain() {
    // Arrange: two migrations; target fails on the second apply_migration call
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry
        .add(arc_up(FakeUpMigration::new(1, Arc::clone(&log))))
        .add(arc_up(FakeUpMigration::new(2, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(None);
    target.fail_on_apply_call.set(Some(2));
    target.fail_apply.set(Some(TestError::ApplyFailed));

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert: first migration applied, second failed; version remains 1
    assert!(matches!(
        result,
        Err(MigrationError::MigrationFailed {
            id: 2,
            source: TestError::ApplyFailed
        })
    ));
    assert_eq!(target.current_version(), Some(1));
    assert_eq!(log.lock().unwrap().as_slice(), &[(1, true)]);
}
