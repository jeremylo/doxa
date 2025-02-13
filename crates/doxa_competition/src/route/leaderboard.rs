use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use serde_json::json;

use crate::client::{Competition, Context};

/// The default route for `_leaderboard/active`.
pub async fn active_leaderboard<C: Competition + ?Sized>(
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let leaderboard = context.get_leaderboard().await?;

    let mut output = Vec::with_capacity(leaderboard.len());

    for (user, entry) in leaderboard {
        output
            .push(json!({ "username": user.username, "agent": entry.agent, "score": entry.score }));
    }

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "leaderboard": output })))
}
