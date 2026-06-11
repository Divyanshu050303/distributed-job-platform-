use crate::domain::job::Job;
use sqlx::PgPool;
use uuid::Uuid;

pub struct JobRepository;
impl JobRepository {
    pub async fn create(pool: &PgPool, job: &Job) -> Result<Job, sqlx::Error> {
        sqlx::query_as::<_, Job>(
            r#"
            INSERT INTO jobs(
            id,
            title,
            description,
            employment_type,
            work_mode,
            location,
            experience_min,
            experience_max,
            salary_min,
            salary_max,
            currency,
            openings,
            status,
            created_by,
            published_at,
            expires_at,
            created_at,
            updated_at,
            deleted_at
            )
            VALUES(
            $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20
            )
            RETURNING *
            "#,
        )
        .bind(job.id)
        .bind(&job.title)
        .bind(&job.description)
        .bind(&job.employment_type)
        .bind(&job.work_mode)
        .bind(&job.location)
        .bind(&job.experience_min)
        .bind(&job.experience_max)
        .bind(&job.salary_min)
        .bind(&job.salary_max)
        .bind(&job.currency)
        .bind(&job.openings)
        .bind(&job.status)
        .bind(&job.created_by)
        .bind(&job.published_at)
        .bind(&job.expires_at)
        .bind(&job.created_at)
        .bind(&job.updated_at)
        .bind(&job.deleted_at)
        .fetch_one(pool)
        .await
    }
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Job>, sqlx::Error> {
        sqlx::query_as::<_, Job>(
            r#"
            SELECT * FROM jobs WHERE id = $q AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Job>, sqlx::Error> {
        sqlx::query_as::<_, Job>(
            r#"SELECT * FROM jobs WHERE deleted_at IS NULL ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }
    pub async fn soft_delete(Pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE jobs SET deleted_at =NOW() WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(Pool)
        .await?;
        Ok(())
    }
}
