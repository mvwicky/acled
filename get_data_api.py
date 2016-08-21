import os
import sys
import json

import requests

if __name__ == '__main__':
    page = int(1)
    if len(sys.argv) > 1:
        if not sys.argv[1] == 'all':
            try:
                int(sys.argv[1])
            except ValueError:
                print('Arg must be a number')
                sys.exit(-1)
            else:
                page = int(sys.argv[1])
        elif sys.argv[1] == 'all':
            page = True

    base_url = 'http://acleddata.com/api/acled/read'

    if isinstance(page, int):
    # get specific page
        print('Getting page {}'.format(page))
        res = requests.get(base_url, params={'page': page})
    elif isinstance(page, bool) and page == True:
    # get all pages
        pass
