CREATE TABLE applicants (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  resume_url VARCHAR(255),
  cover_letter TEXT,
  created_at TIMESTAMP DEFAULT NOW()
);