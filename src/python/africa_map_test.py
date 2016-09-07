import os
import sys

from PIL import Image


if __name__ == '__main__':
    af_map = os.path.realpath('images/africa_map.jpg')
    print(af_map)
    if not os.path.isfile(af_map):
        print('Not a file!')
        sys.exit(-1)
    else:
        print('Is a file!')

    af_im = Image.open(af_map)
    print(af_im.size)

    sm_size = [int(i * 0.25) for i in af_im.size]
    md_size = [int(i * 0.5) for i in af_im.size]

    sm_file = os.path.realpath('images/africa_map_sm.png')
    sm_im = af_im.resize(sm_size)

    md_file = os.path.realpath('images/africa_map_md.png')
    md_im = af_im.resize(md_size)

    lg_file = os.path.realpath('images/africa_map_lg.png')

    with open(sm_file, 'wb') as out_file:
        sm_im.save(out_file, 'PNG')

    with open(md_file, 'wb') as out_file:
        md_im.save(out_file, 'PNG')

    with open(lg_file, 'wb') as out_file:
        af_im.save(out_file, 'PNG')
