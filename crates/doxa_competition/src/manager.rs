//! Contains methods related to the server-side management of competitions

use std::sync::Arc;

use doxa_core::tokio::{self, join};

use crate::{
    client::{Competition, Context},
    error::CompetitionManagerError,
    manager::game_event::GameEventManager,
    Settings,
};

use self::{activation::AgentActivationManager, executor::ExecutionManager};

mod activation;
pub(crate) mod executor;
mod game_event;
// mod upload;

pub struct CompetitionManager<T: Competition> {
    competition: Arc<T>,
    settings: Arc<Settings>,
}

impl<T: Competition> CompetitionManager<T> {
    /// Spawns tasks required for managing the competition
    /// This exists as soon as the various manager systems have *started* (not finished).
    pub async fn start(
        competition: Arc<T>,
        settings: Arc<Settings>,
    ) -> Result<i32, CompetitionManagerError> {
        let manager = CompetitionManager {
            competition,
            settings,
        };

        let db = tokio::task::spawn_blocking({
            let pool = manager.settings.pg_pool.clone();
            move || pool.get()
        })
        .await??;

        let competition = tokio::task::spawn_blocking(move || {
            doxa_db::action::competition::get_competition_by_name(&db, T::COMPETITION_NAME)
        })
        .await??
        .ok_or(CompetitionManagerError::CompetitionNotFound)?;

        let context = Arc::new(Context::<T>::new(
            manager.settings.mq_pool.clone(),
            manager.settings.pg_pool.clone(),
            competition.id,
        ));

        let activation_manager = AgentActivationManager::new(
            manager.settings.clone(),
            manager.competition.clone(),
            context.clone(),
        );

        let game_event_manager = GameEventManager::new(
            manager.settings.clone(),
            manager.competition.clone(),
            context.clone(),
        );

        let execution_manager =
            ExecutionManager::<T::GameClient>::new(manager.settings, T::COMPETITION_NAME);

        join!(
            activation_manager.start(),
            execution_manager.start(),
            game_event_manager.start(),
        );

        Ok(competition.id)
    }
}
