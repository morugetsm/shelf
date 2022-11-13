--
-- PostgreSQL database dump
--

-- Dumped from database version 15.0
-- Dumped by pg_dump version 15.0

-- Started on 2022-11-14 01:36:07 KST

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

DROP DATABASE shelf;
--
-- TOC entry 3651 (class 1262 OID 16391)
-- Name: shelf; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE shelf WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = icu LOCALE = 'en_US.UTF-8' ICU_LOCALE = 'en-US';


ALTER DATABASE shelf OWNER TO postgres;

\connect shelf

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 6 (class 2615 OID 16475)
-- Name: project; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA project;


ALTER SCHEMA project OWNER TO postgres;

--
-- TOC entry 4 (class 2615 OID 2200)
-- Name: public; Type: SCHEMA; Schema: -; Owner: pg_database_owner
--

CREATE SCHEMA public;


ALTER SCHEMA public OWNER TO pg_database_owner;

--
-- TOC entry 3652 (class 0 OID 0)
-- Dependencies: 4
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: pg_database_owner
--

COMMENT ON SCHEMA public IS 'standard public schema';


--
-- TOC entry 221 (class 1259 OID 16542)
-- Name: comment_seq; Type: SEQUENCE; Schema: project; Owner: postgres
--

CREATE SEQUENCE project.comment_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE project.comment_seq OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 219 (class 1259 OID 16513)
-- Name: comment; Type: TABLE; Schema: project; Owner: postgres
--

CREATE TABLE project.comment (
    idx integer DEFAULT nextval('project.comment_seq'::regclass) NOT NULL,
    post integer NOT NULL,
    author integer NOT NULL,
    content character varying,
    reserved character varying,
    rdate timestamp without time zone DEFAULT now() NOT NULL,
    udate timestamp without time zone,
    removed boolean DEFAULT false NOT NULL
);


ALTER TABLE project.comment OWNER TO postgres;

--
-- TOC entry 218 (class 1259 OID 16488)
-- Name: post_seq; Type: SEQUENCE; Schema: project; Owner: postgres
--

CREATE SEQUENCE project.post_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE project.post_seq OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 16531)
-- Name: post; Type: TABLE; Schema: project; Owner: postgres
--

CREATE TABLE project.post (
    idx integer DEFAULT nextval('project.post_seq'::regclass) NOT NULL,
    project integer NOT NULL,
    author integer NOT NULL,
    manager integer,
    progress integer,
    sdate timestamp without time zone,
    edate timestamp without time zone,
    title character varying NOT NULL,
    content character varying,
    shared boolean DEFAULT false NOT NULL,
    noticed boolean DEFAULT false NOT NULL,
    rdate timestamp without time zone DEFAULT now() NOT NULL,
    udate timestamp without time zone,
    removed boolean DEFAULT false NOT NULL
);


ALTER TABLE project.post OWNER TO postgres;

--
-- TOC entry 217 (class 1259 OID 16486)
-- Name: project_seq; Type: SEQUENCE; Schema: project; Owner: postgres
--

CREATE SEQUENCE project.project_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE project.project_seq OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 16477)
-- Name: project; Type: TABLE; Schema: project; Owner: postgres
--

CREATE TABLE project.project (
    idx integer DEFAULT nextval('project.project_seq'::regclass) NOT NULL,
    manager integer NOT NULL,
    sdate timestamp without time zone NOT NULL,
    edate timestamp without time zone NOT NULL,
    title character varying NOT NULL,
    content character varying,
    reserved1 character varying,
    reserved2 character varying,
    rdate timestamp without time zone DEFAULT now() NOT NULL,
    udate timestamp without time zone,
    removed boolean DEFAULT false NOT NULL
);


ALTER TABLE project.project OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 16544)
-- Name: user_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.user_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.user_seq OWNER TO postgres;

--
-- TOC entry 215 (class 1259 OID 16393)
-- Name: user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."user" (
    idx integer DEFAULT nextval('public.user_seq'::regclass) NOT NULL,
    username character varying NOT NULL,
    password character varying NOT NULL,
    name character varying NOT NULL,
    reserved1 character varying,
    reserved2 character varying,
    admin boolean DEFAULT false NOT NULL,
    rdate timestamp without time zone DEFAULT now() NOT NULL,
    udate timestamp without time zone,
    removed boolean DEFAULT false NOT NULL
);


ALTER TABLE public."user" OWNER TO postgres;

--
-- TOC entry 3642 (class 0 OID 16513)
-- Dependencies: 219
-- Data for Name: comment; Type: TABLE DATA; Schema: project; Owner: postgres
--



--
-- TOC entry 3643 (class 0 OID 16531)
-- Dependencies: 220
-- Data for Name: post; Type: TABLE DATA; Schema: project; Owner: postgres
--



--
-- TOC entry 3639 (class 0 OID 16477)
-- Dependencies: 216
-- Data for Name: project; Type: TABLE DATA; Schema: project; Owner: postgres
--



--
-- TOC entry 3638 (class 0 OID 16393)
-- Dependencies: 215
-- Data for Name: user; Type: TABLE DATA; Schema: public; Owner: postgres
--



--
-- TOC entry 3653 (class 0 OID 0)
-- Dependencies: 221
-- Name: comment_seq; Type: SEQUENCE SET; Schema: project; Owner: postgres
--

SELECT pg_catalog.setval('project.comment_seq', 1, false);


--
-- TOC entry 3654 (class 0 OID 0)
-- Dependencies: 218
-- Name: post_seq; Type: SEQUENCE SET; Schema: project; Owner: postgres
--

SELECT pg_catalog.setval('project.post_seq', 1, false);


--
-- TOC entry 3655 (class 0 OID 0)
-- Dependencies: 217
-- Name: project_seq; Type: SEQUENCE SET; Schema: project; Owner: postgres
--

SELECT pg_catalog.setval('project.project_seq', 1, false);


--
-- TOC entry 3656 (class 0 OID 0)
-- Dependencies: 222
-- Name: user_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.user_seq', 2, true);


--
-- TOC entry 3493 (class 2606 OID 16402)
-- Name: user user_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pk PRIMARY KEY (idx);


--
-- TOC entry 3495 (class 1259 OID 16521)
-- Name: comment_idx; Type: INDEX; Schema: project; Owner: postgres
--

CREATE INDEX comment_idx ON project.comment USING btree (idx);


--
-- TOC entry 3494 (class 1259 OID 16485)
-- Name: project_idx; Type: INDEX; Schema: project; Owner: postgres
--

CREATE INDEX project_idx ON project.project USING btree (idx);


-- Completed on 2022-11-14 01:36:07 KST

--
-- PostgreSQL database dump complete
--

