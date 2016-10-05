import os
import sys
import csv
import datetime


class Country(object):
    def __init__(self, name):
        self.name = name
        self.events = []


if __name__ == '__main__':
    f_name = 'ACLED_Data_clean.csv'
    data_path = os.path.abspath(f_name)
    #  find the data file

    countries = []
    with open(data_path) as data_fd:
        rdr = csv.DictReader(data_df)
        header = [elem.lower() for elem in rdr.fieldnames]
        last_country = None
        for row in rdr:
            if last_country is None or last_country != row['COUNTRY']:
                countries.append(Country(row['COUNTRY']))
            countries[-1].events.append(row)

            last_country = row['COUNTRY']

    print(len(countries))

    tot_events = 0
    for ctry in countries:
        print(elem.name, '-', len(elem.events))
        tot_events += len(elem.events)
    print('')
    print('Total Events: {}'.format(tot_events))
