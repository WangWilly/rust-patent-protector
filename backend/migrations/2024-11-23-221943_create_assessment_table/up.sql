-- Your SQL goes here
CREATE TABLE assessments (
    id SERIAL PRIMARY KEY,
    patent_id TEXT NOT NULL,
    company_name TEXT NOT NULL,
    analysis_date TIMESTAMP NOT NULL,
    top_infringing_products JSONB,
    overall_risk_assessment TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
