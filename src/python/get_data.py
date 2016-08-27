import os
import sys
import json


if __name__ == '__main__':
    cfg_file_name = 'config_def.json'
    if len(sys.argv) > 1:  # cmd line input config file
        pass
    if not os.path.isfile(cfg_file_name):
        print('Config file not found ({})'.format(cfg_file_name))
    data_url = None
    with open(cfg_file_name) as cfg_fd:
        cfg_obj = json.load(cfg_fd)
    print(cfg_obj)



