import math
from PIL import Image

img_width = 2048
img_height = 2048



center_x = img_width / 2
center_y = img_height / 2

values = [
    [ [0, 0, 0] for _ in range(img_width) ]
    for _ in range(img_height)
]


def spiro(x, y, frame_radius, shape_radius, hole_height, color):
    prefix = []
    last_n = []

    i = -1
    while True:
        i += 1
        outer_angle = i * math.pi/180.0 / 10
        inner_angle = frame_radius / shape_radius * i * math.pi/180 / 10

        hole_range_min = hole_height / 100.0 * shape_radius
        hole_range_max = shape_radius - hole_range_min
        hole_range = hole_range_max - hole_range_min

        outer_x = frame_radius * math.cos(outer_angle)
        outer_y = frame_radius * math.sin(outer_angle)

        inner_x = hole_range * math.cos(inner_angle)
        inner_y = hole_range * math.sin(inner_angle)

        nx = int(x + outer_x + inner_x)
        ny = int(y + outer_y + inner_y)
        coord = (nx, ny)

        last_n.append(coord)
        if i < 20:
            prefix.append(coord)
        else:
            last_n = last_n[1:]

        if i > 20 and prefix == last_n[-len(prefix):]:
            break

        values[ny][nx][0] = color[0]
        values[ny][nx][1] = color[1]
        values[ny][nx][2] = color[2]



spiro(center_x, center_y, 600, 360, 31, (0xFF, 0xFF, 0x00) )
spiro(center_x, center_y, 600, 235, 1, (0x80, 0xFF, 0x80) )
spiro(center_x, center_y, 600, 105, 17, (0x00, 0xFF, 0xFF) )

spiro(center_x, center_y, 400, 250, 31, (0xFF, 0xFF, 0x80) )
spiro(center_x, center_y, 400, 150, 1, (0xFF, 0x8F, 0x80) )
spiro(center_x, center_y, 400, 35,  15, (0xFF, 0xFF, 0xFF) )

img = Image.new('RGB', (img_width, img_height), 0x000000)

for y in range(img_height):
    for x in range(img_width):
        img.putpixel((x,y), tuple(values[y][x]))

img.save('output.jpg')
