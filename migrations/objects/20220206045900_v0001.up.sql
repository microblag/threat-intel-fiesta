CREATE TABLE object
(
    id        VARCHAR(128) NOT NULL PRIMARY KEY,
    document  TEXT NOT NULL
) WITHOUT ROWID;
                                                                                                                                                                                                                                                                                                     
CREATE TABLE object_relation
(                                                                                                                                                                                                                                                                                                    
    id              INTEGER NOT NULL PRIMARY KEY,                                                                                                                                                                                                                                                    
    source_ref	    VARCHAR(128) NOT NULL REFERENCES object (id),
    target_ref	    VARCHAR(128) NOT NULL REFERENCES object (id),
    kind INTEGER NOT NULL
)
