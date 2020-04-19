CREATE TABLE autoresponse_phrases (
    id integer NOT NULL,
    input text NOT NULL,
    rule_type character varying(255) NOT NULL,
    reply_to boolean NOT NULL,
    output text[] NOT NULL
);

CREATE SEQUENCE autoresponse_phrases_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE autoresponse_phrases_id_seq OWNED BY autoresponse_phrases.id;

CREATE TABLE feeds (
    id integer NOT NULL,
    url character varying(255) NOT NULL,
    kind character varying(100) NOT NULL,
    timeout integer,
    last_update timestamp without time zone,
    last_entry text
);

CREATE SEQUENCE feeds_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE feeds_id_seq OWNED BY feeds.id;

CREATE TABLE greetings (
    id integer NOT NULL,
    text text NOT NULL
);

CREATE SEQUENCE greetings_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE greetings_id_seq OWNED BY greetings.id;

CREATE TABLE schedule (
    id integer NOT NULL,
    day character varying(100) NOT NULL,
    "time" time without time zone NOT NULL,
    messages text[] NOT NULL
);

CREATE SEQUENCE schedule_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE schedule_id_seq OWNED BY schedule.id;

ALTER TABLE ONLY autoresponse_phrases ALTER COLUMN id SET DEFAULT nextval('autoresponse_phrases_id_seq'::regclass);

ALTER TABLE ONLY feeds ALTER COLUMN id SET DEFAULT nextval('feeds_id_seq'::regclass);

ALTER TABLE ONLY greetings ALTER COLUMN id SET DEFAULT nextval('greetings_id_seq'::regclass);

ALTER TABLE ONLY schedule ALTER COLUMN id SET DEFAULT nextval('schedule_id_seq'::regclass);

ALTER TABLE ONLY autoresponse_phrases
    ADD CONSTRAINT autoresponse_phrases_pkey PRIMARY KEY (id);

ALTER TABLE ONLY feeds
    ADD CONSTRAINT feeds_pkey PRIMARY KEY (id);

ALTER TABLE ONLY greetings
    ADD CONSTRAINT greetings_pkey PRIMARY KEY (id);

ALTER TABLE ONLY schedule
    ADD CONSTRAINT schedule_pkey PRIMARY KEY (id);
