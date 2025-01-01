# C BMP Color-to-Grayscale Converter

This project has a C program that reads the data from a 24-bit
color BMP file, converts the image it contains to a grayscale
image, and then writes the grayscale image to a new file. Most of
the work is in reading and writing the image bytes according
to the BMP specification.

**Note:** 24-bit BMP represents pixels using the RGB model with
one byte per color for red, green, and blue. There are
established formulas for converting RGB values to grayscale,
and we can represent grayscale values as

$$t \cdot (255, 255, 255) ~~~\text{with}~~ t \in [0, 1].$$

Here we use the formula mapping

$$(R, G, B) ~~\Rightarrow~~ (n, n, n)$$

where the greyscale value $n$ is given by

$$n = \lfloor 0.299 * R + 0.587 * G + 0.114 * B \rfloor.$$
