-- Your SQL goes here
CREATE TABLE disaster_report_attachments
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    disaster_report_id UUID NOT NULL,
    media_url TEXT NOT NULL,

    CONSTRAINT fk_disaster_report FOREIGN KEY (disaster_report_id)
    REFERENCES disaster_reports(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX idx_disaster_reports_image ON disaster_report_attachments(id);
CREATE INDEX idx_disaster_reports_image_disaster ON disaster_report_attachments(disaster_report_id);
