-- Your SQL goes here
CREATE TABLE TASKS (
    TASK_ID SERIAL
    , USER_ID INTEGER NOT NULL
    , CONTENT VARCHAR(100) NOT NULL
    , CREATED TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    , UPDATED TIMESTAMP
    , DEAD_LINE TIMESTAMP NOT NULL
    , FINISHED_FLAG BOOLEAN NOT NULL DEFAULT 'FALSE'
    , PRIMARY KEY(TASK_ID)
)
;
CREATE TABLE USERS (
    USER_ID SERIAL
    , USER_NAME VARCHAR(100) NOT NULL
    , E_MAIL VARCHAR(100) NOT NULL
    , CREATED TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    , UPDATED TIMESTAMP
    , DELETE_FLAG BOOLEAN NOT NULL DEFAULT 'FALSE'
    , PRIMARY KEY(USER_ID)
)
;