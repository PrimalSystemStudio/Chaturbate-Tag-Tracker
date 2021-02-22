import sqlite3
import pandas as pd

# For adding new data to the table
def add(tag_name, tag_views, tag_rooms, date_time):
    # Open database
    connec = sqlite3.connect('src/cb_log.db')
    cursor = connec.cursor()

    # Add tag data to database [exceptions??]
    cursor.execute(''' INSERT INTO cb_log
                    VALUES (?,?,?,?)''',
                    (tag_name, tag_views, tag_rooms, date_time,))
    print("Tag data successfully entered into database")

    # Save and close database
    connec.commit()
    connec.close()
    return 1

# For loading and cleaning data before displaying it
def clean():
    # Open database
    connec = sqlite3.connect('src/cb_log.db')
    cursor = connec.cursor()

    # Make dataframe of all tags with viewers greater than 10
    query = "SELECT * FROM cb_log WHERE viewers > 10"
    df = pd.read_sql_query(query, connec)

    # Close database
    connec.close()

### views/room ratio
### Add notes for high traffic tags/too many rooms etc.

# For sending cleaned data to Rust

if __name__ == "__main__":
    clean()
