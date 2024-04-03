use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::*, AppState};

use self::handlers_inner::HandlerError;

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> Result<Json<QuestionDetail>, HandlerError> {
    Ok(Json(
        handlers_inner::create_question(question, &(*questions_dao)).await?,
    ))
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> Result<Json<Vec<QuestionDetail>>, HandlerError> {
    Ok(Json(
        handlers_inner::read_questions(&(*questions_dao)).await?,
    ))
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<Json<()>, HandlerError> {
    Ok(Json(
        handlers_inner::delete_question(question_uuid, &(*questions_dao)).await?,
    ))
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> Result<Json<AnswerDetail>, HandlerError> {
    Ok(Json(
        handlers_inner::create_answer(answer, &(*answers_dao)).await?,
    ))
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<Json<Vec<AnswerDetail>>, HandlerError> {
    Ok(Json(
        handlers_inner::read_answers(question_uuid, &(*answers_dao)).await?,
    ))
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> Result<Json<()>, HandlerError> {
    Ok(Json(
        handlers_inner::delete_answer(answer_uuid, &(*answers_dao)).await?,
    ))
}
