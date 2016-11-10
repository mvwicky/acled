import os
import sys
import time

from bs4 import UnicodeDammit

if __name__ == '__main__':
    start = time.process_time()

    file_name = 'ACLED_Data'
    if len(sys.argv) > 1:
        file_name = sys.argv[1]
    clean_name = '{}_clean'.format(file_name)
    data_file = os.path.abspath(
        os.path.join('data', 'csv', '{}.csv'.format(file_name)))
    clean_file = os.path.abspath(
        os.path.join('data', 'csv', '{}.csv'.format(clean_name)))

    if not os.path.isfile(data_file):
        print('File does not exist ({})'.format(data_file))
        sys.exit(-1)

    with open(data_file, 'rb') as csvfile, open(clean_file, 'w') as cleanfile:
        cts = csvfile.read(1024)
        while cts:
            ucode = UnicodeDammit(cts)
            cleanfile.write(ucode.unicode_markup)
            cts = csvfile.read(1024)

    print('Elapsed Time: {}'.format(time.process_time() - start))
