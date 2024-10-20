---------------------------------
-- TABLE: users
---------------------------------
DEFINE TABLE OVERWRITE users TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON users TYPE string;
DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);
DEFINE FIELD username ON users TYPE string;
DEFINE FIELD password ON users TYPE string;
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
DEFINE FIELD primary ON boards TYPE bool DEFAULT false;
DEFINE FIELD space ON boards TYPE record<spaces>;
DEFINE FIELD created_at ON boards TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON boards TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: boards
---------------------------------
DEFINE TABLE OVERWRITE boards TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON boards TYPE string;
DEFINE FIELD description ON boards TYPE option<string>;
DEFINE FIELD icon ON boards TYPE option<string>;
DEFINE FIELD primary ON boards TYPE bool DEFAULT false;
DEFINE FIELD space ON boards TYPE record<spaces>;
DEFINE FIELD created_at ON boards TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON boards TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: task_states
---------------------------------
DEFINE TABLE OVERWRITE task_states TYPE NORMAL SCHEMAFULL;
DEFINE FIELD name ON task_states TYPE string;
DEFINE FIELD color ON task_states TYPE string;
DEFINE FIELD done ON task_states TYPE bool;
DEFINE FIELD task_index ON task_states TYPE int;
DEFINE FIELD board ON task_states TYPE record<boards>;
DEFINE FIELD created_at ON task_states TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON task_states TYPE datetime VALUE time::now();

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
-- TABLE: board_lists
---------------------------------
DEFINE TABLE OVERWRITE board_lists TYPE NORMAL SCHEMAFULL;
DEFINE FIELD board_index ON board_lists TYPE int;
DEFINE FIELD task ON board_lists TYPE record<tasks>;
DEFINE FIELD task_state ON board_lists TYPE record<task_states>;
DEFINE FIELD created_at ON board_lists TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON board_lists TYPE datetime VALUE time::now();

---------------------------------
-- TABLE: users
---------------------------------
DEFINE TABLE OVERWRITE users_spaces TYPE RELATION  FROM users TO spaces SCHEMAFULL;
DEFINE FIELD user_roles ON users_spaces TYPE "OWNER" | "EDITOR" | "GUEST";
DEFINE FIELD created_at ON users_spaces TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users_spaces TYPE datetime VALUE time::now();
