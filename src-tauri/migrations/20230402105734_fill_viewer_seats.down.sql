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
DELETE FROM
    viewer_seats
WHERE
    concert_id IN (
        (
            SELECT
                *
            FROM
                NAU_ID
        ),
        (
            SELECT
                *
            FROM
                GOJIRA_ID
        )
    );