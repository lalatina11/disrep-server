-- Your SQL goes here
CREATE TABLE disaster_report_aid_attachments
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    disaster_report_aid_id UUID NOT NULL,
    image_url TEXT NOT NULL,

    CONSTRAINT fk_disaster_report_aid FOREIGN KEY (disaster_report_aid_id)
    REFERENCES disaster_report_aids(id) ON DELETE CASCADE ON UPDATE CASCADE

)