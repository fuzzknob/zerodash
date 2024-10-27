---------------------------------
-- TABLE: users
---------------------------------
DEFINE TABLE OVERWRITE users TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON users TYPE string;
DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);
DEFINE FIELD username ON users TYPE string;
DEFINE FIELD password ON users TYPE string;
DEFINE FIELD email_verified_at ON users TYPE option<datetime>;
DEFINE FIELD profile_pic ON users TYPE option<string>;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime VALUE time::now();
DEFINE INDEX OVERWRITE usernameIndex ON TABLE users FIELDS username UNIQUE;
DEFINE INDEX OVERWRITE emailIndex ON TABLE users FIELDS email UNIQUE;

-- TODO: Move session to embeded db
---------------------------------
-- TABLE: sessions
---------------------------------
DEFINE TABLE OVERWRITE sessions TYPE NORMAL SCHEMAFULL;
DEFINE FIELD token ON sessions TYPE string;
DEFINE FIELD user ON sessions TYPE record<users>;
DEFINE FIELD expiration ON sessions TYPE datetime DEFAULT time::now();
DEFINE FIELD created_at ON sessions TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON sessions TYPE datetime VALUE time::now();
DEFINE INDEX OVERWRITE tokenIndex ON TABLE sessions FIELDS token UNIQUE;

---------------------------------
-- TABLE: spaces
---------------------------------
DEFINE TABLE OVERWRITE spaces TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON spaces TYPE string;
DEFINE FIELD description ON spaces TYPE option<string>;
DEFINE FIELD icon ON spaces TYPE option<string>;
DEFINE FIELD primary ON spaces TYPE bool DEFAULT false;
DEFINE FIELD created_at ON spaces TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON spaces TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: boards
---------------------------------
DEFINE TABLE OVERWRITE boards TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON boards TYPE string;
DEFINE FIELD description ON boards TYPE option<string>;
DEFINE FIELD icon ON boards TYPE option<string>;
DEFINE FIELD space ON boards TYPE record<spaces>;
DEFINE FIELD created_at ON boards TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON boards TYPE datetime VALUE time::now();

-- ---------------------------------
-- -- TABLE: boards
-- ---------------------------------
-- DEFINE TABLE OVERWRITE boards TYPE NORMAL SCHEMAFULL;
-- DEFINE FIELD name ON boards TYPE string;
-- DEFINE FIELD description ON boards TYPE option<string>;
-- DEFINE FIELD icon ON boards TYPE option<string>;
-- DEFINE FIELD primary ON boards TYPE bool DEFAULT false;
-- DEFINE FIELD space ON boards TYPE record<spaces>;
-- DEFINE FIELD created_at ON boards TYPE datetime DEFAULT time::now();
-- DEFINE FIELD updated_at ON boards TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: task_states
---------------------------------
DEFINE TABLE OVERWRITE task_states TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON task_states TYPE string;
DEFINE FIELD color ON task_states TYPE string;
DEFINE FIELD index ON task_list_index TYPE int;
DEFINE FIELD state_type ON task_states TYPE "TODO" | "COMPLETED";
DEFINE FIELD board ON task_states TYPE record<boards>;
DEFINE FIELD created_at ON task_states TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON task_states TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: task_list_index
---------------------------------
DEFINE TABLE OVERWRITE task_list_index TYPE NORMAL SCHEMAFULL;
DEFINE FIELD task_state ON task_list_index TYPE record<task_states>;
DEFINE FIELD task ON task_list_index TYPE record<tasks>;
DEFINE FIELD index ON task_list_index TYPE int;
DEFINE FIELD created_at ON task_list_index TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON task_list_index TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: task_tags
---------------------------------
DEFINE TABLE OVERWRITE task_tags TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON task_tags TYPE string;
DEFINE FIELD color ON task_tags TYPE string;
DEFINE FIELD board ON task_tags TYPE record<boards>;
DEFINE FIELD created_at ON task_tags TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON task_tags TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: tasks
---------------------------------
DEFINE TABLE OVERWRITE tasks TYPE NORMAL SCHEMAFULL;
DEFINE FIELD title ON tasks TYPE string;
DEFINE FIELD description ON tasks TYPE option<string>;
-- overdue
DEFINE FIELD overdue ON tasks TYPE option<array<object>>;
DEFINE FIELD overdue.*.date ON tasks TYPE datetime;
DEFINE FIELD overdue.*.recurring ON tasks TYPE option<object>;

-- subtasks
DEFINE FIELD subtasks ON tasks TYPE option<array<object>>;
DEFINE FIELD subtasks.*.title ON tasks TYPE string;
DEFINE FIELD subtasks.*.completed ON tasks TYPE string;

DEFINE FIELD board ON tasks TYPE record<boards>;
DEFINE FIELD tags ON tasks TYPE option<array<record<task_tags>>>;
DEFINE FIELD state ON tasks TYPE record<task_states>;
DEFINE FIELD assigned_to ON tasks TYPE option<record<users>>;
DEFINE FIELD created_at ON tasks TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON tasks TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: users_spaces
---------------------------------
DEFINE TABLE OVERWRITE users_spaces TYPE RELATION  FROM users TO spaces SCHEMAFULL;
DEFINE FIELD user_role ON users_spaces TYPE "OWNER" | "EDITOR" | "GUEST";
DEFINE FIELD created_at ON users_spaces TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users_spaces TYPE datetime VALUE time::now();
