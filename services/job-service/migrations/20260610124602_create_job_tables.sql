-- Add migration script here
-- ==========================================
-- JOBS
-- ==========================================

CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,

    company_name VARCHAR(255) NOT NULL,

    employment_type VARCHAR(50) NOT NULL,
    work_mode VARCHAR(50) NOT NULL,

    location VARCHAR(255) NOT NULL,

    experience_min INTEGER NOT NULL DEFAULT 0,
    experience_max INTEGER NOT NULL DEFAULT 0,

    salary_min BIGINT,
    salary_max BIGINT,
    currency VARCHAR(10) NOT NULL DEFAULT 'INR',

    openings INTEGER NOT NULL DEFAULT 1,

    status VARCHAR(50) NOT NULL DEFAULT 'Draft',

    created_by UUID NOT NULL,

    published_at TIMESTAMP,
    expires_at TIMESTAMP,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
);

CREATE INDEX idx_jobs_status ON jobs(status);
CREATE INDEX idx_jobs_created_by ON jobs(created_by);
CREATE INDEX idx_jobs_location ON jobs(location);

-- ==========================================
-- JOB SKILLS
-- ==========================================

CREATE TABLE job_skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,

    skill_name VARCHAR(100) NOT NULL,

    is_mandatory BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE(job_id, skill_name)
);

CREATE INDEX idx_job_skills_job_id
ON job_skills(job_id);

-- ==========================================
-- JOB BENEFITS
-- ==========================================

CREATE TABLE job_benefits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,

    benefit VARCHAR(255) NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE(job_id, benefit)
);

CREATE INDEX idx_job_benefits_job_id
ON job_benefits(job_id);

-- ==========================================
-- JOB STATUS HISTORY
-- ==========================================

CREATE TABLE job_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,

    old_status VARCHAR(50),

    new_status VARCHAR(50) NOT NULL,

    changed_by UUID NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_job_status_history_job_id
ON job_status_history(job_id);

-- ==========================================
-- JOB BOOKMARKS
-- ==========================================

CREATE TABLE job_bookmarks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id UUID NOT NULL,

    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE(user_id, job_id)
);

CREATE INDEX idx_job_bookmarks_user
ON job_bookmarks(user_id);

CREATE INDEX idx_job_bookmarks_job
ON job_bookmarks(job_id);