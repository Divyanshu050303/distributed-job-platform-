CREATE INDEX IF NOT EXISTS idx_jobs_status
ON jobs(status);

CREATE INDEX IF NOT EXISTS idx_jobs_work_mode
ON jobs(work_mode);

CREATE INDEX IF NOT EXISTS idx_jobs_employment_type
ON jobs(employment_type);

CREATE INDEX IF NOT EXISTS idx_jobs_location
ON jobs(location);

CREATE INDEX IF NOT EXISTS idx_jobs_created_at
ON jobs(created_at DESC);