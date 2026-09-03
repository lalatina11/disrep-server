-- Your SQL goes here
CREATE TABLE disaster_reports (
    id          UUID          DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id     UUID          NOT NULL,
    title       TEXT          NOT NULL,
    description TEXT         ,
    street      TEXT         ,
    city        VARCHAR (100) NOT NULL,
    lat         FLOAT         NOT NULL,
    lng         FLOAT         NOT NULL,
    is_anon     BOOLEAN       DEFAULT false NOT NULL,
    status      TEXT          DEFAULT 'pending' NOT NULL,
    created_at  TIMESTAMPTZ   DEFAULT NOW() NOT NULL,
    updated_at  TIMESTAMPTZ   DEFAULT now() NOT NULL,
    CONSTRAINT fk_disaster_reports_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX idx_disaster_reports_id
    ON disaster_reports(id);

CREATE INDEX idx_disaster_reports_user_id
    ON disaster_reports(user_id);