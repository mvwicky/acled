import os
import sys
import time

from bs4 import UnicodeDammit

if __name__ == '__main__':
    start = time.process_time()

    f_name = 'ACLED_Data'
    uf_name = '{}_clean'.format(f_name)
    data_file = os.path.abspath(os.path.join('data', '{}.csv'.format(f_name)))
    clean_file = os.path.abspath(os.path.join('data', '{}.csv'.format(uf_name)))

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
