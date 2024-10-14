---------------------------------
-- TABLE: users
---------------------------------
DEFINE TABLE OVERWRITE users TYPE NORMAL SCHEMALESS
	PERMISSIONS NONE
;
DEFINE FIELD name ON users TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value)
	PERMISSIONS FULL
;
DEFINE FIELD username ON users TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD password ON users TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now()
	PERMISSIONS FULL
;
DEFINE FIELD updated_at ON users TYPE datetime VALUE time::now()
	PERMISSIONS FULL
;
DEFINE INDEX OVERWRITE usernameIndex ON TABLE users FIELDS username UNIQUE;
DEFINE INDEX OVERWRITE emailIndex ON TABLE users FIELDS email UNIQUE;

---------------------------------
-- TABLE: tasks
---------------------------------
DEFINE TABLE OVERWRITE tasks TYPE NORMAL SCHEMALESS
	PERMISSIONS NONE
;
DEFINE FIELD title ON tasks TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD description ON tasks TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD created_at ON tasks TYPE datetime DEFAULT time::now()
	PERMISSIONS FULL
;
DEFINE FIELD updated_at ON tasks TYPE datetime VALUE time::now()
	PERMISSIONS FULL
;
