-- Your SQL goes here
--
-- PostgreSQL database dump
--

-- Dumped from database version 12.6 (Ubuntu 12.6-1.pgdg18.04+1)
-- Dumped by pg_dump version 12.6 (Ubuntu 12.6-1.pgdg18.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', 'public', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;
SET default_tablespace = '';
SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: cyprien
--

-- CREATE TABLE public.__diesel_schema_migrations (
--     version character varying(50) NOT NULL PRIMARY KEY,
--     run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
-- );


-- ALTER TABLE public.__diesel_schema_migrations OWNER TO cyprien;

--
-- Name: comments; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.comments (
    id SERIAL PRIMARY KEY,
    content text NOT NULL,
    user_id integer,
    project_id integer NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    deleted_at timestamp(0) without time zone
);


ALTER TABLE public.comments OWNER TO cyprien;

--
-- Name: comments_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.comments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.comments_id_seq OWNER TO cyprien;

--
-- Name: comments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.comments_id_seq OWNED BY public.comments.id;


--
-- Name: migrations; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.migrations (
    id SERIAL PRIMARY KEY,
    migration character varying(255) NOT NULL,
    batch integer NOT NULL
);


ALTER TABLE public.migrations OWNER TO cyprien;

--
-- Name: migrations_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.migrations_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.migrations_id_seq OWNER TO cyprien;

--
-- Name: migrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.migrations_id_seq OWNED BY public.migrations.id;


--
-- Name: profile_cover_images; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.profile_cover_images (
    id SERIAL PRIMARY KEY,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    user_id integer NOT NULL,
    w3200_keyname character varying(255) NOT NULL UNIQUE,
    w3200_object_url character varying(255) NOT NULL,
    w300_keyname character varying(255) NOT NULL UNIQUE,
    w300_object_url character varying(255) NOT NULL
);


ALTER TABLE public.profile_cover_images OWNER TO cyprien;

--
-- Name: profile_cover_images_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.profile_cover_images_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.profile_cover_images_id_seq OWNER TO cyprien;

--
-- Name: profile_cover_images_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.profile_cover_images_id_seq OWNED BY public.profile_cover_images.id;


--
-- Name: profile_user_images; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.profile_user_images (
    id SERIAL PRIMARY KEY,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    user_id integer NOT NULL,
    w1500_keyname character varying(255) NOT NULL UNIQUE,
    w200_keyname character varying(255) NOT NULL UNIQUE,
    w40_keyname character varying(255) NOT NULL UNIQUE,
    w1500_object_url character varying(255) NOT NULL,
    w200_object_url character varying(255) NOT NULL,
    w40_object_url character varying(255) NOT NULL,
    deleted_at timestamp without time zone
);


ALTER TABLE public.profile_user_images OWNER TO cyprien;

--
-- Name: profile_user_images_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.profile_user_images_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.profile_user_images_id_seq OWNER TO cyprien;

--
-- Name: profile_user_images_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.profile_user_images_id_seq OWNED BY public.profile_user_images.id;


--
-- Name: project_categories; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.project_categories (
    id SERIAL PRIMARY KEY,
    name character varying(255) NOT NULL UNIQUE,
    picture_url character varying(255) NOT NULL,
    slug character varying(255) UNIQUE,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at timestamp without time zone,
    color_hex character varying(7) DEFAULT '#000000'::character varying NOT NULL,
    parent_category_id integer
);


ALTER TABLE public.project_categories OWNER TO cyprien;

--
-- Name: project_categories_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.project_categories_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.project_categories_id_seq OWNER TO cyprien;

--
-- Name: project_categories_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.project_categories_id_seq OWNED BY public.project_categories.id;


--
-- Name: project_image_categories; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.project_image_categories (
    id SERIAL PRIMARY KEY,
    name character varying(255) NOT NULL UNIQUE,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    deleted_at timestamp without time zone,
    color_hex character varying(7) DEFAULT '#FFFFFF'::character varying NOT NULL
);


ALTER TABLE public.project_image_categories OWNER TO cyprien;

--
-- Name: project_image_categories_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.project_image_categories_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.project_image_categories_id_seq OWNER TO cyprien;

--
-- Name: project_image_categories_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.project_image_categories_id_seq OWNED BY public.project_image_categories.id;


--
-- Name: project_images; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.project_images (
    id SERIAL PRIMARY KEY,
    w1500_keyname character varying(255) NOT NULL UNIQUE,
    w350_keyname character varying(255) NOT NULL UNIQUE,
    project_image_category_id integer NOT NULL,
    w1500_object_url character varying(255) NOT NULL,
    w350_object_url character varying(255) NOT NULL,
    "primary" boolean DEFAULT false NOT NULL,
    project_id integer NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    original_object_url character varying(255),
    deleted_at timestamp without time zone,
    views_count integer DEFAULT 0 NOT NULL
);


ALTER TABLE public.project_images OWNER TO cyprien;

--
-- Name: project_images_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.project_images_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.project_images_id_seq OWNER TO cyprien;

--
-- Name: project_images_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.project_images_id_seq OWNED BY public.project_images.id;


--
-- Name: projects; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.projects (
    id SERIAL PRIMARY KEY,
    category_id integer NOT NULL,
    user_id integer NOT NULL,
    title character varying(100) NOT NULL UNIQUE,
    slug character varying(255) NOT NULL UNIQUE,
    content text NOT NULL,
    published boolean DEFAULT false NOT NULL,
    views_count integer DEFAULT 0 NOT NULL,
    likes_count integer DEFAULT 0 NOT NULL,
    deleted_at timestamp(0) without time zone,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    sketchfab_model_number character varying(255),
    is_pro boolean DEFAULT false NOT NULL,
    bitbucket_project_key character varying(20),
    tags character varying(255) DEFAULT ''::character varying NOT NULL
);


ALTER TABLE public.projects OWNER TO cyprien;

--
-- Name: projects_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.projects_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.projects_id_seq OWNER TO cyprien;

--
-- Name: projects_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.projects_id_seq OWNED BY public.projects.id;


--
-- Name: subscriptions; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.subscriptions (
    id SERIAL PRIMARY KEY,
    subscriber_id integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.subscriptions OWNER TO cyprien;

--
-- Name: subscriptions_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.subscriptions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.subscriptions_id_seq OWNER TO cyprien;

--
-- Name: subscriptions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.subscriptions_id_seq OWNED BY public.subscriptions.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: cyprien
--

CREATE TABLE IF NOT EXISTS public.users (
    id SERIAL PRIMARY KEY,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL UNIQUE,
    email character varying(255) NOT NULL UNIQUE,
    password character varying(255) NOT NULL,
    punchline character varying(50),
    website_url text,
    admin boolean DEFAULT false NOT NULL,
    active boolean DEFAULT true NOT NULL,
    "isNewProjectEmailSubscriber" boolean DEFAULT true NOT NULL,
    deleted_at timestamp(0) without time zone,
    remember_token character varying(100),
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    api_token character varying(60) UNIQUE
);


ALTER TABLE public.users OWNER TO cyprien;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: cyprien
--

CREATE SEQUENCE IF NOT EXISTS public.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO cyprien;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: cyprien
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: comments id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.comments ALTER COLUMN id SET DEFAULT nextval('public.comments_id_seq'::regclass);


--
-- Name: migrations id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.migrations ALTER COLUMN id SET DEFAULT nextval('public.migrations_id_seq'::regclass);


--
-- Name: profile_cover_images id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.profile_cover_images ALTER COLUMN id SET DEFAULT nextval('public.profile_cover_images_id_seq'::regclass);


--
-- Name: profile_user_images id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.profile_user_images ALTER COLUMN id SET DEFAULT nextval('public.profile_user_images_id_seq'::regclass);


--
-- Name: project_categories id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.project_categories ALTER COLUMN id SET DEFAULT nextval('public.project_categories_id_seq'::regclass);


--
-- Name: project_image_categories id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.project_image_categories ALTER COLUMN id SET DEFAULT nextval('public.project_image_categories_id_seq'::regclass);


--
-- Name: project_images id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.project_images ALTER COLUMN id SET DEFAULT nextval('public.project_images_id_seq'::regclass);


--
-- Name: projects id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.projects ALTER COLUMN id SET DEFAULT nextval('public.projects_id_seq'::regclass);


--
-- Name: subscriptions id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.subscriptions ALTER COLUMN id SET DEFAULT nextval('public.subscriptions_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: cyprien
--

-- ALTER TABLE ONLY public.__diesel_schema_migrations
--     ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);

--
-- Name: project_images project_images_w1500_keyname_unique; Type: CONSTRAINT; Schema: public; Owner: cyprien
--

--
-- Name: subscriptions subscriptions_subscriber_id_user_id_unique; Type: CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.subscriptions
    ADD CONSTRAINT subscriptions_subscriber_id_user_id_unique UNIQUE (subscriber_id, user_id);


--
-- Name: comments comments_project_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_project_id_foreign FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;


--
-- Name: profile_user_images profile_user_images_user_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.profile_user_images
    ADD CONSTRAINT profile_user_images_user_id_foreign FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: project_categories project_categories_parent_category_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.project_categories
    ADD CONSTRAINT project_categories_parent_category_id_fkey FOREIGN KEY (parent_category_id) REFERENCES public.project_categories(id) ON DELETE RESTRICT;


--
-- Name: project_images project_images_project_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.project_images
    ADD CONSTRAINT project_images_project_id_foreign FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;


--
-- Name: project_images project_images_project_image_category_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.project_images
    ADD CONSTRAINT project_images_project_image_category_id_foreign FOREIGN KEY (project_image_category_id) REFERENCES public.project_image_categories(id) ON DELETE RESTRICT;


--
-- Name: projects projects_category_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_category_id_foreign FOREIGN KEY (category_id) REFERENCES public.project_categories(id) ON DELETE RESTRICT;


--
-- Name: projects projects_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: projects projects_user_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_user_id_foreign FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: subscriptions subscriptions_subscriber_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.subscriptions
    ADD CONSTRAINT subscriptions_subscriber_id_foreign FOREIGN KEY (subscriber_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: subscriptions subscriptions_user_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: cyprien
--

ALTER TABLE ONLY public.subscriptions
    ADD CONSTRAINT subscriptions_user_id_foreign FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

