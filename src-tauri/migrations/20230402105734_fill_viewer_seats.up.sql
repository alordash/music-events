WITH NAU_ID AS (
    SELECT
        id
    FROM
        concerts
    WHERE
        name = 'НАУ40'
),
GOJIRA_ID AS (
    SELECT
        id
    FROM
        concerts
    WHERE
        name = 'Gojira anniversary'
)
INSERT INTO
    viewer_seats (kind, cost_rubles, real_number, concert_id)
VALUES
    (
        'Seat',
        4000,
        1,
        (SELECT
            *
        FROM
            NAU_ID)
    ),
    (
        'Seat',
        4000,
        2,
        (SELECT
            *
        FROM
            NAU_ID)
    ),
    (
        'Seat',
        4000,
        3,
        (SELECT
            *
        FROM
            NAU_ID)
    ),
    (
        'Dance floor',
        1000,
        1,
        (SELECT
            *
        FROM
            NAU_ID)
    ),
    (
        'Seat',
        7000,
        1,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    ),
    (
        'Seat',
        7000,
        2,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    ),
    (
        'Seat',
        7000,
        3,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    ),
    (
        'Seat',
        7000,
        4,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    ),
    (
        'Seat',
        7000,
        5,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    ),
    (
        'Dance floor',
        2000,
        1,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    ),
    (
        'Dance floor',
        2000,
        2,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    ),
    (
        'Dance floor',
        2000,
        3,
        (SELECT
            *
        FROM
            GOJIRA_ID)
    );