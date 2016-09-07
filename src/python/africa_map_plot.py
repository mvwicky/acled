import os
import csv
import sys

import matplotlib.pyplot as plt


if __name__ == '__main__':
    af_map_path = os.path.realpath('images/africa_map_lg.png')
    if os.path.isfile(af_map_path):
        print('{} exists'.format(af_map_path))
    else:
        print('{} does not exist'.format(af_map_path))
        sys.exit(-1)

    af_data_path = os.path.realpath('data/csv/ACLED_Data_clean.csv')
    if os.path.isfile(af_data_path):
        print('{} exists'.format(af_data_path))
    else:
        print('{} does not exist'.format(af_data_path))
        sys.exit(-1)

    country_name = 'Egypt'

    event_coords = []
    with open(af_data_path) as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
        #    if row['COUNTRY'] == country_name:
        #        lat = row['LATITUDE']
        #        lon = row['LONGITUDE']
        #        coord = (float(lat), float(lon))
        #        event_coords.append(coord)
            lat = row['LATITUDE']
            lon = row['LONGITUDE']
            coord = (float(lon), float(lat))
            event_coords.append(coord)

    print(len(event_coords))
    lats = [i[0] for i in event_coords]
    lons = [i[1] for i in event_coords]

    plt.plot(lats, lons, '.')
    plt.show()

