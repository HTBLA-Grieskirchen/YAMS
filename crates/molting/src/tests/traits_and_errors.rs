//! Integration tests for trait impls (Box, Arc), MigrationError, and edge cases.

use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::{MigrationError, MigrationRegistry, UpMigration};
use pollster::FutureExt as _;

use super::common::{FakeMigrationTarget, FakeUpMigration, TestError};

// ---------- MigrationError ----------

#[test]
fn migration_error_runner_error_display() {
    // Arrange
    let err = MigrationError::<TestError>::RunnerError(TestError::GetVersionFailed);

    // Act & Assert
    let s = err.to_string();
    assert!(s.contains("runner") && s.contains("get_version_failed"));
}

#[test]
fn migration_error_migration_failed_display() {
    // Arrange
    let err = MigrationError::MigrationFailed {
        id: 42,
        source: TestError::ApplyFailed,
    };

    // Act & Assert
    let s = err.to_string();
    assert!(s.contains("42") && s.contains("apply_failed"));
}

#[test]
fn migration_error_version_mismatch_display() {
    // Arrange
    let err = MigrationError::<TestError>::VersionMismatch {
        expected: 1,
        actual: 2,
    };

    // Act & Assert
    let s = err.to_string();
    assert!(s.contains("1") && s.contains("2") && s.contains("mismatch"));
}

#[test]
fn migration_error_down_not_supported_display() {
    // Arrange
    let err = MigrationError::<TestError>::DownMigrationNotSupported;

    // Act & Assert
    let s = err.to_string();
    assert!(s.to_lowercase().contains("down") && s.to_lowercase().contains("not supported"));
}

#[test]
fn migration_error_runner_error_source() {
    // Arrange
    let inner = TestError::GetVersionFailed;
    let err = MigrationError::RunnerError(inner.clone());

    // Act & Assert: source() returns the inner error
    let source = err.source().expect("source");
    let msg = source.to_string();
    assert!(msg.contains("get_version_failed"));
}

// ---------- Box<dyn UpMigration> delegation ----------

#[test]
fn box_dyn_up_migration_delegates_version_and_up() {
    // Arrange: wrap concrete migration in Box<dyn UpMigration>
    let log = Arc::new(Mutex::new(Vec::new()));
    let concrete = FakeUpMigration::new(7, Arc::clone(&log));
    let boxed: Box<dyn UpMigration<(), TestError>> = Box::new(concrete);

    // Act & Assert: version and up delegate to inner
    assert_eq!(boxed.version(), 7);
    assert_eq!(boxed.description(), None);

    let mut target = ();
    let result = pollster::block_on(boxed.up(&mut target));
    assert!(result.is_ok());
    assert_eq!(log.lock().unwrap().as_slice(), &[(7, true)]);
}

#[test]
fn box_dyn_up_migration_description_delegation() {
    // Arrange
    let log = Arc::new(Mutex::new(Vec::new()));
    let concrete = FakeUpMigration::with_description(1, "test_desc", Arc::clone(&log));
    let boxed: Box<dyn UpMigration<(), TestError>> = Box::new(concrete);

    // Act & Assert
    assert_eq!(boxed.description(), Some("test_desc"));
}

// ---------- Arc<dyn UpMigration> in registry ----------

#[test]
fn arc_dyn_up_migration_in_registry_applies_correctly() {
    // Arrange: add Arc-wrapped migration to registry
    let log = Arc::new(Mutex::new(Vec::new()));
    let migration = Arc::new(FakeUpMigration::new(1, Arc::clone(&log)));
    let mut registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    registry.add(Arc::clone(&migration) as Arc<dyn UpMigration<(), TestError>>);
    let mut target = FakeMigrationTarget::new(None);

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert
    assert!(result.is_ok());
    assert_eq!(target.current_version(), Some(1));
    assert_eq!(log.lock().unwrap().as_slice(), &[(1, true)]);
}

// ---------- UpMigration::description default ----------

#[test]
fn up_migration_description_default_is_none() {
    // Arrange: FakeUpMigration::new does not set description
    let log = Arc::new(Mutex::new(Vec::new()));
    let m = FakeUpMigration::new(1, log);

    // Act & Assert
    assert_eq!(m.description(), None);
}

// ---------- Edge: empty registry apply ----------

#[test]
fn empty_registry_apply_with_target_none_leaves_version_unchanged() {
    // Arrange: no migrations, target None -> current 0, target 0, no apply_migration calls
    let registry: MigrationRegistry<dyn UpMigration<(), TestError>> = MigrationRegistry::new();
    let mut target = FakeMigrationTarget::new(None);

    // Act
    let result = registry.apply(&mut target, None).block_on();

    // Assert: no migrations run, target version stays None
    assert!(result.is_ok());
    assert_eq!(target.current_version(), None);
}
