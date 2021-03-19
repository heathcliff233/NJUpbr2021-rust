import cv2
import numpy as np

img = cv2.imread('base.png', 1)
rate = float(img.shape[0])/float(img.shape[1])
if img.shape[1] != 100 :
	height = int(100*rate)
	img = cv2.resize(img, (100, height), interpolation=cv2.INTER_CUBIC)

def custom_blur_demo(image):
    kernel = np.array([[0, -1, 0], [-1, 5, -1], [0, -1, 0]], np.float32)
    dst = cv2.filter2D(image, -1, kernel=kernel)
    return dst

#img = custom_blur_demo(img)
cv2.imwrite('1.png', img)


