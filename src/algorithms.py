import sqlite3

# For adding new data to the table
def add(tag_name, tag_views, tag_rooms, date_time):
    # Open database
    connec = sqlite3.connect('cb_log.db')
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

# For cleaning data before displaying it
## Make function
### make internal memory of data
### views/room ratio
### remove low traffic tags
### Add notes for high traffic tags/too many rooms etc.

# For sending cleaned data to Rust

if __name__ == "__main__":
        add("#sexy", 100, 200, "12:00 16-02-2020")