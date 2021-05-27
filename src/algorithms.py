import sqlite3
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# For adding new data to the table
def add(tag_name, tag_views, tag_rooms, date, time):
    # Open database
    connec = sqlite3.connect('src/cb_log.db')
    cursor = connec.cursor()

    # Add tag data to database [exceptions??]
    cursor.execute(''' INSERT INTO cb_log
                    VALUES (?,?,?,?,?)''',
                    (tag_name, tag_views, tag_rooms, date, time,))
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

    # Make column for viewers/rooms ratio
    ratio = df['viewers'] / df['rooms']
    df.insert(loc=len(df.columns), column='ratio', value = ratio)

    # Sort by highest ratio
    print(df.sort_values(by='ratio', ascending=False).head())

    # Close database
    connec.close()

# For sending cleaned data to Rust
# def send():

if __name__ == "__main__":
    clean()
