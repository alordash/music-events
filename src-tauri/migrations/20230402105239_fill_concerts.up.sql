BEGIN TRAN ADD_CONCERTS;
INSERT INTO
    concerts (date, duration_minutes, address, name)
VALUES
    (
        CURRENT_TIMESTAMP + '7 days',
        120,
        'Moscow, Crocus City Hall',
        'НАУ40'
    ),
    (
        CURRENT_TIMESTAMP + '100 days',
        180,
        'Mars, Sirius',
        'Gojira anniversary'
    );

SELECT * FROM concerts WHERE name IN ('НАУ40', 'Gojira anniversary', 'Chris Rea concert');
-- будут возвращены записи двух концертов: "НАУ40" и "Gojira anniversary"

SAVEPOINT my_savepoint;
DELETE FROM concerts WHERE name IN ('Gojira anniversary');
SELECT * FROM concerts WHERE name IN ('НАУ40', 'Gojira anniversary', 'Chris Rea concert');
-- будет возвращена запись только одного концерта: "НАУ40"

ROLLBACK TO SAVEPOINT my_savepoint;
SELECT * FROM concerts WHERE name IN ('НАУ40', 'Gojira anniversary', 'Chris Rea concert');
-- будут возвращены записи двух концертов: "НАУ40" и "Gojira anniversary"

INSERT INTO
    concerts (date, duration_minutes, address, name)
VALUES
    (
        CURRENT_TIMESTAMP + '10 days',
        555,
        'Moscow, Crocus City Hall',
        'Chris Rea concert'
    );
COMMIT TRAN ADD_CONCERTS;
SELECT * FROM concerts WHERE name IN ('НАУ40', 'Gojira anniversary', 'Chris Rea concert');
-- будут возвращены записи трёх концертов: "НАУ40", "Gojira anniversary" и "Chris Rea concert"