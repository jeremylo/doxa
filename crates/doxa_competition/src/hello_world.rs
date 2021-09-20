use std::convert::Infallible;

use crate::{
    client::{Competition, Context, GameEvent},
    error::ContextError,
};
use async_trait::async_trait;
use doxa_executor::{client::GameClient, context::GameContext};

use serde::{Deserialize, Serialize};

/// A dummy competition for development/debugging
pub struct HelloWorldCompetiton;

#[async_trait]
impl Competition for HelloWorldCompetiton {
    type GameClient = HelloWorldGameClient;

    const COMPETITION_NAME: &'static str = "helloworld";

    async fn startup(&self, _context: &Context<Self>) -> Result<(), ContextError> {
        println!("[hello_world] starting up");

        Ok(())
    }

    async fn on_agent_activated(
        &self,
        context: &Context<Self>,
        agent_id: String,
    ) -> Result<(), ContextError> {
        context.emit_match_request(vec![agent_id], ()).await?;

        Ok(())
    }

    async fn on_agent_deactivated(
        &self,
        context: &Context<Self>,
        agent_id: String,
    ) -> Result<(), ContextError> {
        context.emit_match_request(vec![agent_id], ()).await?;

        Ok(())
    }

    async fn on_game_event(
        &self,
        context: &Context<Self>,
        event: GameEvent<HelloWorldGameEvent>,
    ) -> Result<(), ContextError> {
        let mut participants = context
            .get_game_participants_unordered(event.game_id)
            .await?;
        let participant = participants.remove(0);

        let score = match event.payload {
            HelloWorldGameEvent::RespondedIncorrectly { .. } => -1,
            HelloWorldGameEvent::RespondedSuccessfully { .. } => 1,
        };

        context.set_new_score(participant.agent, score).await?;

        Ok(())
    }

    fn event_filter(
        game_event: HelloWorldGameEvent,
        _is_admin: bool,
        agent: Option<usize>,
    ) -> Option<serde_json::Value> {
        // Only yield events if the client was part of the event
        if agent.is_some() {
            Some(serde_json::to_value(game_event).unwrap())
        } else {
            None
        }
    }
}

pub struct HelloWorldGameClient;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum HelloWorldGameEvent {
    RespondedSuccessfully {
        output: String,
    },
    RespondedIncorrectly {
        expected_output: String,
        agent_output: String,
    },
}

#[async_trait]
impl GameClient for HelloWorldGameClient {
    type Error = Infallible;
    type MatchRequest = ();
    type GameEvent = HelloWorldGameEvent;

    async fn run<'a>(
        _match_request: Self::MatchRequest,
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), doxa_executor::error::GameError<Self::Error>> {
        context.expect_n_agents(1)?;

        context
            .send_message_to_agent(0, b"PLEASE ECHO THIS MESSAGE\n")
            .await?;

        let expected_output = b"echo PLEASE ECHO THIS MESSAGE";

        let message = context.next_message(0).await?;
        if message == expected_output {
            context
                .emit_game_event(
                    HelloWorldGameEvent::RespondedSuccessfully {
                        output: String::from_utf8_lossy(expected_output).to_string(),
                    },
                    "game",
                )
                .await?;
        } else {
            let agent_output = String::from_utf8_lossy(message).to_string();
            context
                .emit_game_event(
                    HelloWorldGameEvent::RespondedIncorrectly {
                        expected_output: String::from_utf8_lossy(expected_output).to_string(),
                        agent_output,
                    },
                    "game",
                )
                .await?;
        }

        Ok(())
    }
}
