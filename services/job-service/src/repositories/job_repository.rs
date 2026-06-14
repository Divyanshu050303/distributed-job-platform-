use crate::api::dto::{job_query_params::JobQueryParams, update_job_request::UpdateJobRequest};
use crate::domain::job::Job;
use sqlx::{PgPool, Postgres, QueryBuilder};
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
            company_name,  
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
        .bind(&job.company_name)
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
            SELECT * FROM jobs WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }
    pub async fn count_all(pool: &PgPool, params: &JobQueryParams) -> Result<i64, sqlx::Error> {
        let mut qb =
            QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM jobs WHERE deleted_at IS NULL");
        Self::apply_filters(&mut qb, params);
        qb.build_query_scalar().fetch_one(pool).await
    }

    pub async fn find_all(pool: &PgPool, params: &JobQueryParams) -> Result<Vec<Job>, sqlx::Error> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM jobs WHERE deleted_at IS NULL");
        Self::apply_filters(&mut qb, params);

        qb.push(" ORDER BY created_at DESC");

        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        qb.push(" LIMIT ");
        qb.push_bind(limit);
        qb.push(" OFFSET ");
        qb.push_bind(offset);

        qb.build_query_as::<Job>().fetch_all(pool).await
    }

    fn apply_filters(qb: &mut QueryBuilder<Postgres>, params: &JobQueryParams) {
        if let Some(status) = &params.status {
            qb.push(" AND status = ");
            qb.push_bind(status.clone());
        }

        if let Some(employment_type) = &params.employment_type {
            qb.push(" AND employment_type = ");
            qb.push_bind(employment_type.clone());
        }

        if let Some(work_mode) = &params.work_mode {
            qb.push(" AND work_mode = ");
            qb.push_bind(work_mode.clone());
        }

        if let Some(location) = &params.location {
            qb.push(" AND location ILIKE ");
            qb.push_bind(format!("%{}%", location));
        }

        if let Some(search) = &params.search {
            let pattern = format!("%{}%", search);

            qb.push(" AND (");

            qb.push("title ILIKE ");
            qb.push_bind(pattern.clone());

            qb.push(" OR description ILIKE ");
            qb.push_bind(pattern.clone());

            qb.push(" OR company_name ILIKE ");
            qb.push_bind(pattern);

            qb.push(")");
        }
    }

    // pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    //     sqlx::query(
    //         r#"
    //     UPDATE jobs SET deleted_at =NOW() WHERE id = $1
    //     "#,
    //     )
    //     .bind(id)
    //     .execute(pool)
    //     .await?;
    //     Ok(())
    // }
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        job: &UpdateJobRequest,
    ) -> Result<Option<Job>, sqlx::Error> {
        sqlx::query_as::<_, Job>(
            r#"
        UPDATE jobs
        SET
            title = $2,
            description = $3,
            company_name = $4,
            employment_type = $5,
            work_mode = $6,
            location = $7,
            experience_min = $8,
            experience_max = $9,
            salary_min = $10,
            salary_max = $11,
            currency = $12,
            openings = $13,
            expires_at = $14,
            updated_at = NOW()
        WHERE id = $1
          AND deleted_at IS NULL
        RETURNING *
        "#,
        )
        .bind(id)
        .bind(&job.title)
        .bind(&job.description)
        .bind(&job.company_name)
        .bind(job.employment_type.clone())
        .bind(job.work_mode.clone())
        .bind(&job.location)
        .bind(job.experience_min)
        .bind(job.experience_max)
        .bind(job.salary_min)
        .bind(job.salary_max)
        .bind(&job.currency)
        .bind(job.openings)
        .bind(job.expires_at)
        .fetch_optional(pool)
        .await
    }
    pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result =sqlx::query(
            r#"UPDATE jobs set deleted_at = NOW() updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL"#
        ).bind(id).execute(pool).await?;

        Ok(result.rows_affected() > 0)
    }
}
