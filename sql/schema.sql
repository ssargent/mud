--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2 (Postgres.app)
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: game; Type: SCHEMA; Schema: -; Owner: mud
--

CREATE SCHEMA game;


ALTER SCHEMA game OWNER TO mud;

--
-- Name: player; Type: SCHEMA; Schema: -; Owner: mud
--

CREATE SCHEMA player;


ALTER SCHEMA player OWNER TO mud;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: postgres
--

-- *not* creating schema, since initdb creates it


ALTER SCHEMA public OWNER TO postgres;

--
-- Name: system; Type: SCHEMA; Schema: -; Owner: mud
--

CREATE SCHEMA system;


ALTER SCHEMA system OWNER TO mud;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: attributes; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.attributes (
    id bigint NOT NULL,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.attributes OWNER TO mud;

--
-- Name: attributes_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.attributes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.attributes_id_seq OWNER TO mud;

--
-- Name: attributes_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.attributes_id_seq OWNED BY game.attributes.id;


--
-- Name: feats; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.feats (
    id bigint NOT NULL,
    world_id bigint NOT NULL,
    code character varying(32) NOT NULL,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.feats OWNER TO mud;

--
-- Name: feats_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.feats_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.feats_id_seq OWNER TO mud;

--
-- Name: feats_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.feats_id_seq OWNED BY game.feats.id;


--
-- Name: item_categories; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.item_categories (
    id bigint NOT NULL,
    parent_id bigint,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.item_categories OWNER TO mud;

--
-- Name: item_categories_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.item_categories_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.item_categories_id_seq OWNER TO mud;

--
-- Name: item_categories_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.item_categories_id_seq OWNED BY game.item_categories.id;


--
-- Name: items; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.items (
    id bigint NOT NULL,
    category_id bigint NOT NULL,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    item_properties jsonb NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    item_type character varying(32) NOT NULL,
    code character varying(32) NOT NULL,
    base_price bigint DEFAULT 0 NOT NULL,
    world_id bigint DEFAULT 1 NOT NULL
);


ALTER TABLE game.items OWNER TO mud;

--
-- Name: items_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.items_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.items_id_seq OWNER TO mud;

--
-- Name: items_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.items_id_seq OWNED BY game.items.id;


--
-- Name: npc_spawn_rules; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.npc_spawn_rules (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    npc_template_id bigint NOT NULL,
    world_node_id bigint NOT NULL,
    spawn_chance integer NOT NULL,
    spawn_quantity_min integer DEFAULT 1 NOT NULL,
    spawn_quantity_max integer DEFAULT 1 NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.npc_spawn_rules OWNER TO mud;

--
-- Name: npc_templates; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.npc_templates (
    id bigint NOT NULL,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    npc_properties jsonb NOT NULL,
    can_spawn_multiple boolean NOT NULL,
    can_respawn boolean NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.npc_templates OWNER TO mud;

--
-- Name: npc_templates_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.npc_templates_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.npc_templates_id_seq OWNER TO mud;

--
-- Name: npc_templates_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.npc_templates_id_seq OWNED BY game.npc_templates.id;


--
-- Name: races; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.races (
    id bigint NOT NULL,
    world_id bigint NOT NULL,
    code character varying(32) NOT NULL,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.races OWNER TO mud;

--
-- Name: races_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.races_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.races_id_seq OWNER TO mud;

--
-- Name: races_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.races_id_seq OWNED BY game.races.id;


--
-- Name: skills; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.skills (
    id bigint NOT NULL,
    world_id bigint NOT NULL,
    code character varying(32) NOT NULL,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.skills OWNER TO mud;

--
-- Name: skills_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.skills_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.skills_id_seq OWNER TO mud;

--
-- Name: skills_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.skills_id_seq OWNED BY game.skills.id;


--
-- Name: world_node_features; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.world_node_features (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    world_node_id bigint NOT NULL,
    feature_name character varying(32) NOT NULL,
    feature_value text NOT NULL,
    feature_properties jsonb NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE game.world_node_features OWNER TO mud;

--
-- Name: world_nodes; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.world_nodes (
    id bigint NOT NULL,
    parent_id bigint,
    name character varying(32) NOT NULL,
    description text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    world_id bigint NOT NULL
);


ALTER TABLE game.world_nodes OWNER TO mud;

--
-- Name: world_nodes_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.world_nodes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.world_nodes_id_seq OWNER TO mud;

--
-- Name: world_nodes_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.world_nodes_id_seq OWNED BY game.world_nodes.id;


--
-- Name: worlds; Type: TABLE; Schema: game; Owner: mud
--

CREATE TABLE game.worlds (
    name character varying(32) NOT NULL,
    description text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    id bigint NOT NULL
);


ALTER TABLE game.worlds OWNER TO mud;

--
-- Name: worlds_id_seq; Type: SEQUENCE; Schema: game; Owner: mud
--

CREATE SEQUENCE game.worlds_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE game.worlds_id_seq OWNER TO mud;

--
-- Name: worlds_id_seq; Type: SEQUENCE OWNED BY; Schema: game; Owner: mud
--

ALTER SEQUENCE game.worlds_id_seq OWNED BY game.worlds.id;


--
-- Name: characters; Type: TABLE; Schema: player; Owner: mud
--

CREATE TABLE player.characters (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    user_id uuid NOT NULL,
    character_name character varying(32) NOT NULL,
    class character varying(32) NOT NULL,
    character_level integer NOT NULL,
    character_definition jsonb NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE player.characters OWNER TO mud;

--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: mud
--

CREATE TABLE public.schema_migrations (
    version bigint NOT NULL,
    dirty boolean NOT NULL
);


ALTER TABLE public.schema_migrations OWNER TO mud;

--
-- Name: settings; Type: TABLE; Schema: system; Owner: mud
--

CREATE TABLE system.settings (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(32) NOT NULL,
    data_type character varying(32) NOT NULL,
    value text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE system.settings OWNER TO mud;

--
-- Name: users; Type: TABLE; Schema: system; Owner: mud
--

CREATE TABLE system.users (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    username character varying(32) NOT NULL,
    password character varying(128) NOT NULL,
    email character varying(128) NOT NULL,
    full_name character varying(128) NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE system.users OWNER TO mud;

--
-- Name: attributes id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.attributes ALTER COLUMN id SET DEFAULT nextval('game.attributes_id_seq'::regclass);


--
-- Name: feats id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.feats ALTER COLUMN id SET DEFAULT nextval('game.feats_id_seq'::regclass);


--
-- Name: item_categories id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.item_categories ALTER COLUMN id SET DEFAULT nextval('game.item_categories_id_seq'::regclass);


--
-- Name: items id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.items ALTER COLUMN id SET DEFAULT nextval('game.items_id_seq'::regclass);


--
-- Name: npc_templates id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.npc_templates ALTER COLUMN id SET DEFAULT nextval('game.npc_templates_id_seq'::regclass);


--
-- Name: races id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.races ALTER COLUMN id SET DEFAULT nextval('game.races_id_seq'::regclass);


--
-- Name: skills id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.skills ALTER COLUMN id SET DEFAULT nextval('game.skills_id_seq'::regclass);


--
-- Name: world_nodes id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.world_nodes ALTER COLUMN id SET DEFAULT nextval('game.world_nodes_id_seq'::regclass);


--
-- Name: worlds id; Type: DEFAULT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.worlds ALTER COLUMN id SET DEFAULT nextval('game.worlds_id_seq'::regclass);


--
-- Name: attributes pk_attributes_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.attributes
    ADD CONSTRAINT pk_attributes_id PRIMARY KEY (id);


--
-- Name: feats pk_feats_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.feats
    ADD CONSTRAINT pk_feats_id PRIMARY KEY (id);


--
-- Name: item_categories pk_item_categories_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.item_categories
    ADD CONSTRAINT pk_item_categories_id PRIMARY KEY (id);


--
-- Name: items pk_items_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.items
    ADD CONSTRAINT pk_items_id PRIMARY KEY (id);


--
-- Name: npc_spawn_rules pk_npc_spawn_rules_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.npc_spawn_rules
    ADD CONSTRAINT pk_npc_spawn_rules_id PRIMARY KEY (id);


--
-- Name: npc_templates pk_npc_templates_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.npc_templates
    ADD CONSTRAINT pk_npc_templates_id PRIMARY KEY (id);


--
-- Name: races pk_races_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.races
    ADD CONSTRAINT pk_races_id PRIMARY KEY (id);


--
-- Name: skills pk_skills_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.skills
    ADD CONSTRAINT pk_skills_id PRIMARY KEY (id);


--
-- Name: world_node_features pk_world_node_features_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.world_node_features
    ADD CONSTRAINT pk_world_node_features_id PRIMARY KEY (id);


--
-- Name: world_nodes pk_world_nodes_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.world_nodes
    ADD CONSTRAINT pk_world_nodes_id PRIMARY KEY (id);


--
-- Name: worlds pk_worlds_id; Type: CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.worlds
    ADD CONSTRAINT pk_worlds_id PRIMARY KEY (id);


--
-- Name: characters pk_characters_id; Type: CONSTRAINT; Schema: player; Owner: mud
--

ALTER TABLE ONLY player.characters
    ADD CONSTRAINT pk_characters_id PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: mud
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: settings pk_settings_id; Type: CONSTRAINT; Schema: system; Owner: mud
--

ALTER TABLE ONLY system.settings
    ADD CONSTRAINT pk_settings_id PRIMARY KEY (id);


--
-- Name: users pk_users_id; Type: CONSTRAINT; Schema: system; Owner: mud
--

ALTER TABLE ONLY system.users
    ADD CONSTRAINT pk_users_id PRIMARY KEY (id);


--
-- Name: idx_feats_world; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_feats_world ON game.feats USING btree (world_id);


--
-- Name: idx_items_category_id; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_items_category_id ON game.items USING btree (category_id);


--
-- Name: idx_items_world; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_items_world ON game.items USING btree (world_id);


--
-- Name: idx_npc_spawn_rules_npc_template_id; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_npc_spawn_rules_npc_template_id ON game.npc_spawn_rules USING btree (npc_template_id);


--
-- Name: idx_npc_spawn_rules_world_node_id; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_npc_spawn_rules_world_node_id ON game.npc_spawn_rules USING btree (world_node_id);


--
-- Name: idx_races_world; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_races_world ON game.races USING btree (world_id);


--
-- Name: idx_skills_world; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_skills_world ON game.skills USING btree (world_id);


--
-- Name: idx_world_node_features_world_node_id; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_world_node_features_world_node_id ON game.world_node_features USING btree (world_node_id);


--
-- Name: idx_world_nodes_world_id; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX idx_world_nodes_world_id ON game.world_nodes USING btree (world_id);


--
-- Name: ix_items_type; Type: INDEX; Schema: game; Owner: mud
--

CREATE INDEX ix_items_type ON game.items USING btree (item_type);


--
-- Name: uq_feats_code; Type: INDEX; Schema: game; Owner: mud
--

CREATE UNIQUE INDEX uq_feats_code ON game.feats USING btree (world_id, code);


--
-- Name: uq_items_code; Type: INDEX; Schema: game; Owner: mud
--

CREATE UNIQUE INDEX uq_items_code ON game.items USING btree (code);


--
-- Name: uq_races_code; Type: INDEX; Schema: game; Owner: mud
--

CREATE UNIQUE INDEX uq_races_code ON game.races USING btree (world_id, code);


--
-- Name: uq_skills_code; Type: INDEX; Schema: game; Owner: mud
--

CREATE UNIQUE INDEX uq_skills_code ON game.skills USING btree (world_id, code);


--
-- Name: idx_characters_user_id; Type: INDEX; Schema: player; Owner: mud
--

CREATE INDEX idx_characters_user_id ON player.characters USING btree (user_id);


--
-- Name: feats fk_feats_world_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.feats
    ADD CONSTRAINT fk_feats_world_id FOREIGN KEY (world_id) REFERENCES game.worlds(id);


--
-- Name: item_categories fk_item_categories_parent_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.item_categories
    ADD CONSTRAINT fk_item_categories_parent_id FOREIGN KEY (parent_id) REFERENCES game.item_categories(id);


--
-- Name: items fk_items_category_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.items
    ADD CONSTRAINT fk_items_category_id FOREIGN KEY (category_id) REFERENCES game.item_categories(id);


--
-- Name: items fk_items_world_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.items
    ADD CONSTRAINT fk_items_world_id FOREIGN KEY (world_id) REFERENCES game.worlds(id);


--
-- Name: npc_spawn_rules fk_npc_spawn_rules_npc_template_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.npc_spawn_rules
    ADD CONSTRAINT fk_npc_spawn_rules_npc_template_id FOREIGN KEY (npc_template_id) REFERENCES game.npc_templates(id);


--
-- Name: npc_spawn_rules fk_npc_spawn_rules_world_node_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.npc_spawn_rules
    ADD CONSTRAINT fk_npc_spawn_rules_world_node_id FOREIGN KEY (world_node_id) REFERENCES game.world_nodes(id);


--
-- Name: races fk_races_world_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.races
    ADD CONSTRAINT fk_races_world_id FOREIGN KEY (world_id) REFERENCES game.worlds(id);


--
-- Name: skills fk_skills_world_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.skills
    ADD CONSTRAINT fk_skills_world_id FOREIGN KEY (world_id) REFERENCES game.worlds(id);


--
-- Name: world_node_features fk_world_node_features_world_node_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.world_node_features
    ADD CONSTRAINT fk_world_node_features_world_node_id FOREIGN KEY (world_node_id) REFERENCES game.world_nodes(id);


--
-- Name: world_nodes fk_world_nodes_worlds_id; Type: FK CONSTRAINT; Schema: game; Owner: mud
--

ALTER TABLE ONLY game.world_nodes
    ADD CONSTRAINT fk_world_nodes_worlds_id FOREIGN KEY (world_id) REFERENCES game.worlds(id);


--
-- Name: characters fk_characters_user_id; Type: FK CONSTRAINT; Schema: player; Owner: mud
--

ALTER TABLE ONLY player.characters
    ADD CONSTRAINT fk_characters_user_id FOREIGN KEY (user_id) REFERENCES system.users(id);


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: postgres
--

REVOKE USAGE ON SCHEMA public FROM PUBLIC;
GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--

