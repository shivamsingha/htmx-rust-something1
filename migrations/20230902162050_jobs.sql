CREATE TABLE jobs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  company_id INTEGER REFERENCES companies(id),
  location VARCHAR(255),
  salary NUMERIC(10, 2),
  created_at TIMESTAMP DEFAULT NOW(),
  expires_at TIMESTAMP
);