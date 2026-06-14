use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::dto::{
        create_job_request::CreateJobRequest, create_job_response::CreateJobResponse,
        job_list_response::JobListResponse, job_query_params::JobQueryParams,
        job_response::JobResponse, update_job_request::UpdateJobRequest,
    },
    domain::job::Job,
    errors::app_error::AppError,
    repositories::job_repository::JobRepository,
};

pub struct JobService;

impl JobService {
    pub async fn create_job(
        pool: &PgPool,
        request: CreateJobRequest,
        created_by: Uuid,
    ) -> Result<CreateJobResponse, AppError> {
        let now = Utc::now().naive_utc();
        let job = Job {
            id: Uuid::new_v4(),
            title: request.title,
            description: request.description,
            company_name: request.company_name,
            employment_type: request.employment_type,
            work_mode: request.work_mode,
            location: request.location,
            experience_min: request.experience_min,
            experience_max: request.experience_max,
            salary_min: request.salary_min,
            salary_max: request.salary_max,
            currency: request.currency,
            openings: request.openings,
            status: request.status,
            created_by,
            published_at: None,
            expires_at: request.expires_at.map(|dt| dt.naive_utc()),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        if request.experience_min > request.experience_max {
            return Err(AppError::Validation(
                "experience_min cannot be greater than experience_max".into(),
            ));
        }
        if request.salary_min > request.salary_max {
            return Err(AppError::Validation(
                "salary_min cannot be greater than salary_max".into(),
            ));
        }
        let job = JobRepository::create(pool, &job).await.map_err(|e| {
            println!("Create Job Error: {:?}", e);
            AppError::InternalServerError
        })?;
        Ok(CreateJobResponse {
            id: job.id,
            title: job.title,
            company_name: job.company_name,
            status: job.status,
            created_at: job.created_at,
        })
    }
    pub async fn get_job(pool: &PgPool, id: Uuid) -> Result<JobResponse, AppError> {
        let job = JobRepository::find_by_id(pool, id)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::NotFound)?;
        println!("Job: {:?}", job);
        Ok(JobResponse {
            id: job.id,
            title: job.title,
            description: job.description,
            company_name: job.company_name,
            employment_type: job.employment_type,
            work_mode: job.work_mode,
            location: job.location,
            experience_min: job.experience_min,
            experience_max: job.experience_max,
            salary_min: job.salary_min,
            salary_max: job.salary_max,
            currency: job.currency,
            openings: job.openings,
            status: job.status,
            created_by: job.created_by,
            published_at: job.published_at,
            expires_at: job.expires_at,
            created_at: job.created_at,
            updated_at: job.updated_at,
        })
    }
    pub async fn get_Jobs(
        pool: &PgPool,
        params: JobQueryParams,
    ) -> Result<JobListResponse, AppError> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10);
        let total = JobRepository::count_all(pool, &params)
            .await
            .map_err(|_| AppError::InternalServerError)?;
        let jobs = JobRepository::find_all(pool, &params)
            .await
            .map_err(|_| AppError::InternalServerError)?;
        let total_pages = if total == 0 {
            0
        } else {
            (total as f64 / limit as f64).ceil() as i64
        };
        Ok(JobListResponse {
            items: jobs.into_iter().map(JobResponse::from).collect(),
            page,
            limit,
            total_pages,
            total,
        })
    }
    pub async fn update_job(
        pool: &PgPool,
        id: Uuid,
        request: UpdateJobRequest,
    ) -> Result<JobResponse, AppError> {
        if request.experience_min > request.experience_max {
            return Err(AppError::Validation(
                "experience_min cannot be greater than experience_max".into(),
            ));
        }

        if request.salary_min > request.salary_max {
            return Err(AppError::Validation(
                "salary_min cannot be greater than salary_max".into(),
            ));
        }

        let job = JobRepository::update(pool, id, &request)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::JobNotFound)?;

        Ok(job.into())
    }
    pub async fn delete_job(pool: &PgPool, id: Uuid) -> Result<bool, AppError> {
        let deleted = JobRepository::soft_delete(pool, id)
            .await
            .map_err(|_| AppError::InternalServerError)?;
        if !deleted {
            return Err(AppError::JobNotFound);
        }
        Ok(deleted)
    }
}
