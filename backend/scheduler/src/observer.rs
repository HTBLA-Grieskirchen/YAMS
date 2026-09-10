use scheduler::{SchedulerEvent, SchedulerObserver};

#[derive(Debug, Clone, Copy, Default)]
pub struct TracingObserver;

impl SchedulerObserver for TracingObserver {
    fn on_event(&self, event: &SchedulerEvent) {
        match event {
            SchedulerEvent::StateLoaded {
                job_id,
                trigger_count,
                next_run_at,
                source,
            } => tracing::info!(
                job_id,
                trigger_count,
                ?next_run_at,
                ?source,
                "scheduler state loaded"
            ),
            SchedulerEvent::StateRepaired {
                job_id,
                trigger_count,
                previous_next_run_at,
                repaired_next_run_at,
            } => tracing::warn!(
                job_id,
                trigger_count,
                ?previous_next_run_at,
                ?repaired_next_run_at,
                "scheduler state repaired"
            ),
            SchedulerEvent::TriggerEmitted {
                job_id,
                scheduled_at,
                catch_up,
                trigger_count,
            } => tracing::debug!(
                job_id,
                %scheduled_at,
                catch_up,
                trigger_count,
                "scheduler trigger emitted"
            ),
            SchedulerEvent::RunSkipped {
                job_id,
                scheduled_at,
                catch_up,
                trigger_count,
                reason,
            } => tracing::debug!(
                job_id,
                %scheduled_at,
                catch_up,
                trigger_count,
                %reason,
                "scheduler run skipped"
            ),
            SchedulerEvent::RunCompleted {
                job_id,
                scheduled_at,
                catch_up,
                trigger_count,
                status,
                error,
            } => tracing::debug!(
                job_id,
                %scheduled_at,
                catch_up,
                trigger_count,
                ?status,
                ?error,
                "scheduler run completed"
            ),
            SchedulerEvent::ExecutionGuardAcquired {
                job_id,
                resource_id,
                scope,
                lease_key,
                scheduled_at,
                catch_up,
                trigger_count,
            } => tracing::debug!(
                job_id,
                resource_id,
                ?scope,
                lease_key,
                ?scheduled_at,
                catch_up,
                trigger_count,
                "scheduler execution guard acquired"
            ),
            SchedulerEvent::ExecutionGuardContended {
                job_id,
                resource_id,
                scope,
                scheduled_at,
                catch_up,
                trigger_count,
            } => tracing::debug!(
                job_id,
                resource_id,
                ?scope,
                ?scheduled_at,
                catch_up,
                trigger_count,
                "scheduler execution guard contended"
            ),
            SchedulerEvent::ExecutionGuardRenewed {
                job_id,
                resource_id,
                scope,
                lease_key,
                scheduled_at,
                catch_up,
                trigger_count,
                renewal_count,
            } => tracing::debug!(
                job_id,
                resource_id,
                ?scope,
                lease_key,
                ?scheduled_at,
                catch_up,
                trigger_count,
                renewal_count,
                "scheduler execution guard renewed"
            ),
            SchedulerEvent::ExecutionGuardRenewFailed {
                job_id,
                resource_id,
                scope,
                lease_key,
                scheduled_at,
                catch_up,
                trigger_count,
                renewal_count,
                failed_renewal_count,
                error,
            } => tracing::warn!(
                job_id,
                resource_id,
                ?scope,
                lease_key,
                ?scheduled_at,
                catch_up,
                trigger_count,
                renewal_count,
                failed_renewal_count,
                error,
                "scheduler execution guard renew failed"
            ),
            SchedulerEvent::ExecutionGuardLost {
                job_id,
                resource_id,
                scope,
                lease_key,
                scheduled_at,
                catch_up,
                trigger_count,
                renewal_count,
                failed_renewal_count,
            } => tracing::warn!(
                job_id,
                resource_id,
                ?scope,
                lease_key,
                ?scheduled_at,
                catch_up,
                trigger_count,
                renewal_count,
                failed_renewal_count,
                "scheduler execution guard lost"
            ),
            SchedulerEvent::ExecutionGuardReleased {
                job_id,
                resource_id,
                scope,
                lease_key,
                scheduled_at,
                catch_up,
                trigger_count,
            } => tracing::debug!(
                job_id,
                resource_id,
                ?scope,
                lease_key,
                ?scheduled_at,
                catch_up,
                trigger_count,
                "scheduler execution guard released"
            ),
            SchedulerEvent::ExecutionGuardReleaseFailed {
                job_id,
                resource_id,
                scope,
                lease_key,
                scheduled_at,
                catch_up,
                trigger_count,
                error,
            } => tracing::warn!(
                job_id,
                resource_id,
                ?scope,
                lease_key,
                ?scheduled_at,
                catch_up,
                trigger_count,
                error,
                "scheduler execution guard release failed"
            ),
            SchedulerEvent::StoreDegraded {
                job_id,
                operation,
                error,
            } => tracing::warn!(job_id, ?operation, error, "scheduler store degraded"),
            SchedulerEvent::StoreRecovering { job_id, operation } => {
                tracing::info!(job_id, ?operation, "scheduler store recovering")
            }
            SchedulerEvent::StoreRecovered { job_id, operation } => {
                tracing::info!(job_id, ?operation, "scheduler store recovered")
            }
            SchedulerEvent::StoreRecoveryFailed {
                job_id,
                operation,
                error,
            } => tracing::warn!(job_id, ?operation, error, "scheduler store recovery failed"),
            SchedulerEvent::StoreRecoveryConflict {
                job_id,
                operation,
                error,
            } => tracing::warn!(
                job_id,
                ?operation,
                error,
                "scheduler store recovery conflict"
            ),
            SchedulerEvent::ExecutionGuardDegraded {
                job_id,
                resource_id,
                scope,
                scheduled_at,
                catch_up,
                trigger_count,
                error,
            } => tracing::warn!(
                job_id,
                resource_id,
                ?scope,
                ?scheduled_at,
                catch_up,
                trigger_count,
                error,
                "scheduler execution guard degraded"
            ),
            SchedulerEvent::ExecutionGuardRecovered {
                job_id,
                resource_id,
                scope,
                scheduled_at,
                catch_up,
                trigger_count,
            } => tracing::info!(
                job_id,
                resource_id,
                ?scope,
                ?scheduled_at,
                catch_up,
                trigger_count,
                "scheduler execution guard recovered"
            ),
            SchedulerEvent::TerminalStateDeleted {
                job_id,
                trigger_count,
            } => tracing::info!(job_id, trigger_count, "scheduler deleted terminal state"),
            SchedulerEvent::SchedulerPaused {
                job_id,
                trigger_count,
                scope,
            } => tracing::info!(job_id, trigger_count, ?scope, "scheduler paused"),
            SchedulerEvent::SchedulerResumed {
                job_id,
                trigger_count,
                scope,
            } => tracing::info!(job_id, trigger_count, ?scope, "scheduler resumed"),
            SchedulerEvent::SchedulerStopped {
                job_id,
                trigger_count,
                reason,
            } => tracing::info!(job_id, trigger_count, ?reason, "scheduler stopped"),
        }
    }
}
