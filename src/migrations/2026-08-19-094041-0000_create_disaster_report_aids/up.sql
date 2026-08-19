-- Your SQL goes here
CREATE TABLE disaster_report_aids
(

    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    disaster_report_id UUID NOT NULL,

    CONSTRAINT fk_disaster_report FOREIGN KEY (disaster_report_id)
    REFERENCES disaster_reports(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX idx_disaster_reports_aid_disaster ON disaster_reports(id);