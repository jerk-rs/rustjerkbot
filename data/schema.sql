CREATE TABLE public.autoresponse_phrases (
    id integer NOT NULL,
    input text NOT NULL,
    rule_type character varying(255) NOT NULL,
    reply_to boolean NOT NULL,
    output text[] NOT NULL
);

CREATE SEQUENCE public.autoresponse_phrases_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.autoresponse_phrases_id_seq OWNED BY public.autoresponse_phrases.id;

CREATE TABLE public.feeds (
    id integer NOT NULL,
    url character varying(255) NOT NULL,
    kind character varying(100) NOT NULL,
    timeout integer,
    last_update timestamp without time zone,
    last_entry text
);

CREATE SEQUENCE public.feeds_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.feeds_id_seq OWNED BY public.feeds.id;

CREATE TABLE public.greetings (
    id integer NOT NULL,
    text text NOT NULL
);

CREATE SEQUENCE public.greetings_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.greetings_id_seq OWNED BY public.greetings.id;

CREATE TABLE public.schedule (
    id integer NOT NULL,
    day character varying(100) NOT NULL,
    "time" time without time zone NOT NULL,
    messages text[] NOT NULL
);

CREATE SEQUENCE public.schedule_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.schedule_id_seq OWNED BY public.schedule.id;

CREATE TABLE public.shippering_phrases (
    id integer NOT NULL,
    template text
);

ALTER TABLE public.shippering_phrases OWNER TO ross;

CREATE SEQUENCE public.shippering_phrases_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.shippering_phrases_id_seq OWNED BY public.shippering_phrases.id;

CREATE TABLE public.users (
    id bigint NOT NULL,
    first_name character varying(255) NOT NULL,
    last_name character varying(255),
    username character varying(255)
);

ALTER TABLE ONLY public.autoresponse_phrases ALTER COLUMN id SET DEFAULT nextval('public.autoresponse_phrases_id_seq'::regclass);
ALTER TABLE ONLY public.feeds ALTER COLUMN id SET DEFAULT nextval('public.feeds_id_seq'::regclass);
ALTER TABLE ONLY public.greetings ALTER COLUMN id SET DEFAULT nextval('public.greetings_id_seq'::regclass);
ALTER TABLE ONLY public.schedule ALTER COLUMN id SET DEFAULT nextval('public.schedule_id_seq'::regclass);
ALTER TABLE ONLY public.shippering_phrases ALTER COLUMN id SET DEFAULT nextval('public.shippering_phrases_id_seq'::regclass);

SELECT pg_catalog.setval('public.autoresponse_phrases_id_seq', 1, false);
SELECT pg_catalog.setval('public.feeds_id_seq', 1, false);
SELECT pg_catalog.setval('public.greetings_id_seq', 1, false);
SELECT pg_catalog.setval('public.schedule_id_seq', 1, false);
SELECT pg_catalog.setval('public.shippering_phrases_id_seq', 1, false);

ALTER TABLE ONLY public.autoresponse_phrases ADD CONSTRAINT autoresponse_phrases_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.feeds ADD CONSTRAINT feeds_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.greetings ADD CONSTRAINT greetings_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.schedule ADD CONSTRAINT schedule_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.shippering_phrases ADD CONSTRAINT shippering_phrases_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.users ADD CONSTRAINT users_pkey PRIMARY KEY (id);
