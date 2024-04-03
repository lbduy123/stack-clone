use async_trait::async_trait;
use sqlx::types::uuid;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = uuid::Uuid::parse_str(&answer.question_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;
        let record = sqlx::query!(
            r#"
            INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *
            "#,
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if let Some(code) = db_err.code() {
                    if code == postgres_error_codes::FOREIGN_KEY_VIOLATION {
                        return DBError::InvalidUUID(db_err.to_string());
                    }
                }
            }
            return DBError::Other(Box::new(e));
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid =
            uuid::Uuid::parse_str(&answer_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = uuid::Uuid::parse_str(&question_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;
        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|rec| AnswerDetail {
                answer_uuid: rec.answer_uuid.to_string(),
                question_uuid: rec.question_uuid.to_string(),
                content: rec.content.to_string(),
                created_at: rec.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
