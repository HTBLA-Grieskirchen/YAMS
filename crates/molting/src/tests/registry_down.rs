//! Integration tests for `MigrationRegistry<dyn DownMigration>`: apply up, apply down, errors.

use std::sync::{Arc, Mutex};

use crate::{DownMigration, MigrationError, MigrationRegistry};
use pollster::FutureExt as _;

use super::common::{FakeDownMigration, FakeMigrationTarget, TestError, arc_down};

// ---------- Apply up (same as UpMigration registry) ----------

#[test]
fn down_registry_apply_up_to_latest_applies_all_migrations() {
    // Arrange: DownMigration registry used for up
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::new();
    registry
        .add_dyn(arc_down(FakeDownMigration::new(1, Arc::clone(&log))))
        .add_dyn(arc_down(FakeDownMigration::new(2, Arc::clone(&log))))
        .add_dyn(arc_down(FakeDownMigration::new(3, Arc::clone(&log))));
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
fn down_registry_apply_when_current_equals_target_is_no_op() {
    // Arrange
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::new();
    registry.add_dyn(arc_down(FakeDownMigration::new(1, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(1));

    // Act
    let result = registry.apply(&mut target, Some(1)).block_on();

    // Assert
    assert!(result.is_ok());
    assert!(log.lock().unwrap().is_empty());
}

// ---------- Apply down ----------

#[test]
fn apply_down_from_current_to_lower_target_applies_down_migrations() {
    // Arrange: current 3, target 0; migrations 1, 2, 3
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::new();
    registry
        .add_dyn(arc_down(FakeDownMigration::new(1, Arc::clone(&log))))
        .add_dyn(arc_down(FakeDownMigration::new(2, Arc::clone(&log))))
        .add_dyn(arc_down(FakeDownMigration::new(3, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(3));

    // Act: target 0
    let result = registry.apply(&mut target, Some(0)).block_on();

    // Assert: down applied in reverse order 3, 2, 1; last apply_migration receives new_version None
    assert!(result.is_ok());
    assert_eq!(target.current_version(), None);
    assert_eq!(
        log.lock().unwrap().as_slice(),
        &[(3, false), (2, false), (1, false)]
    );
}

#[test]
fn apply_down_from_current_to_mid_target_applies_only_relevant_down() {
    // Arrange: current 5, target 2; migrations 1..5
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::new();
    for v in 1..=5 {
        registry.add_dyn(arc_down(FakeDownMigration::new(v, Arc::clone(&log))));
    }
    let mut target = FakeMigrationTarget::new(Some(5));

    // Act: target 2
    let result = registry.apply(&mut target, Some(2)).block_on();

    // Assert: down 5, 4, 3 (so we end at version 2)
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(2));
    assert_eq!(
        log.lock().unwrap().as_slice(),
        &[(5, false), (4, false), (3, false)]
    );
}

#[test]
fn apply_down_one_step_from_2_to_1() {
    // Arrange: current 2, target 1
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::new();
    registry
        .add_dyn(arc_down(FakeDownMigration::new(1, Arc::clone(&log))))
        .add_dyn(arc_down(FakeDownMigration::new(2, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(2));

    // Act
    let result = registry.apply(&mut target, Some(1)).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(1));
    assert_eq!(log.lock().unwrap().as_slice(), &[(2, false)]);
}

#[test]
fn apply_down_from_one_to_none() {
    // Arrange: current 1, target None -> in get_versions target_version.unwrap_or(latest_ver)
    // So target None means "latest" for up. For down, we're going from current to *lower*.
    // So we need target_version Some(0) to go to "no migrations". Let me check the lib:
    // target = target_version.unwrap_or(latest_ver). So if we want to go to 0 we pass Some(0).
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::new();
    registry.add_dyn(arc_down(FakeDownMigration::new(1, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(1));

    // Act: target 0 (no migrations applied)
    let result = registry.apply(&mut target, Some(0)).block_on();

    // Assert: last down receives new_version None, so target stores None
    assert!(result.is_ok());
    assert_eq!(target.current_version(), None);
    assert_eq!(log.lock().unwrap().as_slice(), &[(1, false)]);
}

// ---------- Down registry: runner errors ----------

#[test]
fn down_registry_get_current_version_error_propagates() {
    // Arrange
    let registry: MigrationRegistry<dyn DownMigration<(), TestError>> = MigrationRegistry::new();
    let mut target = FakeMigrationTarget::new(Some(1));
    target
        .fail_get_version
        .set(Some(TestError::GetVersionFailed));

    // Act
    let result = registry.apply(&mut target, Some(0)).block_on();

    // Assert
    assert!(matches!(
        result,
        Err(MigrationError::RunnerError(TestError::GetVersionFailed))
    ));
}

#[test]
fn down_registry_apply_down_failure_propagates_as_migration_failed() {
    // Arrange: current 2, target 0; fail on first down (version 2)
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::new();
    registry
        .add_dyn(arc_down(FakeDownMigration::new(1, Arc::clone(&log))))
        .add_dyn(arc_down(FakeDownMigration::new(2, Arc::clone(&log))));
    let mut target = FakeMigrationTarget::new(Some(2));
    target.fail_apply.set(Some(TestError::ApplyFailed));

    // Act
    let result = registry.apply(&mut target, Some(0)).block_on();

    // Assert: first down (2) fails
    assert!(matches!(
        result,
        Err(MigrationError::MigrationFailed {
            id: 2,
            source: TestError::ApplyFailed
        })
    ));
    assert_eq!(target.current_version(), Some(2));
    assert!(log.lock().unwrap().is_empty());
}

// ---------- From<Vec> for DownMigration registry ----------

#[test]
fn down_registry_from_vec_sorts_and_applies_down_correctly() {
    // Arrange: from vec out of order
    let log = Arc::new(Mutex::new(Vec::new()));
    let m2 = arc_down(FakeDownMigration::new(2, Arc::clone(&log)));
    let m1 = arc_down(FakeDownMigration::new(1, Arc::clone(&log)));
    let m3 = arc_down(FakeDownMigration::new(3, Arc::clone(&log)));
    let registry: MigrationRegistry<dyn DownMigration<(), TestError>> =
        MigrationRegistry::from(vec![m2, m1, m3]);
    let mut target = FakeMigrationTarget::new(Some(3));

    // Act: apply down to 0
    let result = registry.apply(&mut target, Some(0)).block_on();

    // Assert: down order 3, 2, 1
    assert!(result.is_ok());
    assert_eq!(
        log.lock().unwrap().as_slice(),
        &[(3, false), (2, false), (1, false)]
    );
}
