#ifndef BMP_H
#define BMP_H

#include <stdint.h> // For uintX_t.

/**
 *  Data structures for BMP header data, and
 *  functions for working with BMP files.
 */

/* Stuctures definitions. */

typedef struct BMPHeader {
  char type[3]; /* 2 bytes + null char */
  uint32_t size;
  uint16_t reserved1;
  uint16_t reserved2;
  uint32_t offset;
  // End Windows BITMAPFILEHEADER.
  // Next offset is 14 bytes.
} BMPHeader;

typedef struct BMPInfoHeader {
  uint32_t size;
  uint16_t width;
  uint16_t height;
  uint8_t planes;
  uint8_t bitcount;
  uint16_t compression;
  uint16_t sizeImage;
  uint16_t xPelsPerMeter;
  uint16_t yPelsPerMeter;
  uint16_t colorsUsed;
  uint16_t colorsImportant;
  // End Windows BITMAPINFOHEADER[.
  // Next offset is 54 bytes.
} BMPInfoHeader;

typedef struct Pixel {
  uint8_t blue;
  uint8_t green;
  uint8_t red;
  // Because we assume 24-bit pixels.
} Pixel;

typedef struct PixelData {
  Pixel *pixels;
  size_t width;
  size_t height;
} PixelData;

void free_PixelData(PixelData *);

/* Function declarations. */

BMPHeader *readHeader(FILE *);

BMPInfoHeader *readInfoHeader(FILE *);

void printHeaderInfo(BMPHeader *, BMPInfoHeader *);

PixelData *readPixels(BMPHeader *, BMPInfoHeader *, FILE *);

PixelData *convertToGrayscale(PixelData *);

void writeImageFile(FILE *, BMPHeader *, BMPInfoHeader *, PixelData *,
                    const char *, size_t);

#endif
