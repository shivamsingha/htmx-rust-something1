CREATE TABLE applications (
  id SERIAL PRIMARY KEY,
  job_id INTEGER REFERENCES jobs(id),
  applicant_id INTEGER REFERENCES applicants(id),
  created_at TIMESTAMP DEFAULT NOW()
);