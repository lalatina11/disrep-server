-- Your SQL goes here
CREATE TABLE disaster_report_aid_items
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    disaster_report_aid_id UUID NOT NULL,
    item_name VARCHAR(128) NOT NULL,
    item_price DOUBLE PRECISION NOT NULL,
    quantity BIGINT NOT NULL,

    CONSTRAINT fk_disaster_report_aid FOREIGN KEY (disaster_report_aid_id)
    REFERENCES disaster_report_aids(id) ON DELETE CASCADE ON UPDATE CASCADE
)

