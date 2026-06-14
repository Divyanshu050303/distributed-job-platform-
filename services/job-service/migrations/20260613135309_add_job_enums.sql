-- Create enums
CREATE TYPE job_status AS ENUM (
    'Draft',
    'Published',
    'Closed',
    'Archived'
);

CREATE TYPE employment_type AS ENUM (
    'FullTime',
    'PartTime',
    'Contract',
    'Internship',
    'Freelance'
);

CREATE TYPE work_mode AS ENUM (
    'Remote',
    'Hybrid',
    'Onsite'
);

-- Drop default
ALTER TABLE jobs
ALTER COLUMN status DROP DEFAULT;

-- Convert values if necessary
UPDATE jobs
SET employment_type = 'FullTime'
WHERE employment_type = 'Full Time';

UPDATE jobs
SET work_mode = 'Onsite'
WHERE work_mode = 'WFO';

-- Alter types
ALTER TABLE jobs
ALTER COLUMN status
TYPE job_status
USING status::job_status;

ALTER TABLE jobs
ALTER COLUMN employment_type
TYPE employment_type
USING employment_type::employment_type;

ALTER TABLE jobs
ALTER COLUMN work_mode
TYPE work_mode
USING work_mode::work_mode;

-- Restore default
ALTER TABLE jobs
ALTER COLUMN status
SET DEFAULT 'Draft';