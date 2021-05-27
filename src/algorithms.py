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

    # Close database
    connec.close()

    # Make column for viewers/rooms ratio
    df['ratio'] = df['viewers'] / df['rooms']

    # Sort by highest ratio
    df = df.sort_values(by='ratio', ascending=False).head()

    return df

# Returns list of tags
def tag_list(df):
    drop = df['tags'].drop_duplicates
    
    return drop

# Display tags with the highest v/r ratio
def ratios(df):
    sns.displot(df, x='time', y='ratio', hue='tag')
    plt.show()

# Display tags with highest viewers whose v/r ratio > 1
def good_tags(df):
    pass

# Display tags whose v/r ratio < 1
def bad_tags(df):
    pass

# Display tag variation by time of day
def hour(df, tag):
    pass

# Display tag variation over days of the week
def week(df, tag):
    pass

if __name__ == "__main__":
    clean()
