import os
import sys
import csv
import datetime


class Country(object):
    def __init__(self, name):
        self.name = name
        self.events = []

    @property
    def total_events(self):
        return len(self.events)

    @property
    def total_fatalities(self):
        return sum([int(e['FATALITIES']) for e in self.events])

    @property
    def avg_fatalities(self):
        return float(self.total_fatalities) / float(self.total_events)

if __name__ == '__main__':
    f_name = 'ACLED_Data_clean.csv'
    data_path = os.path.abspath(os.path.join(
        'data', 'csv', f_name))
    if not os.path.isfile(data_path):
        print('Data file not found ({})'.format(data_path))
        sys.exit(-1)

    countries = []
    with open(data_path) as data_fd:
        rdr = csv.DictReader(data_fd)
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
        print(ctry.name, '-', ctry.total_events, '-', ctry.total_fatalities)
        tot_events += ctry.total_events
    print('')
    print('Total Events: {}'.format(tot_events))
