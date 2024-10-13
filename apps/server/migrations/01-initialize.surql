 ------------------------------
-- TABLE: tasks
-- ------------------------------
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
