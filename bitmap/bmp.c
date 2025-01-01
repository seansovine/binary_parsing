#include <locale.h>
#include <stdio.h> // For calloc.
#include <stdlib.h>
#include <string.h> // For strncpy.

#include "bmp.h"

/**
 *  Functions for working with
 *  BMP header data structures.
 */

BMPHeader *readHeader(FILE *fp) {
  BMPHeader *bmpHeader = (BMPHeader *)calloc(1, sizeof(struct BMPHeader));

  fread(&bmpHeader->type, 1, 2, fp);
  bmpHeader->type[2] = '\0';
  fread(&bmpHeader->size, 4, 1, fp);
  // Ignore reserved fields.
  fseek(fp, 4, SEEK_CUR);
  fread(&bmpHeader->offset, 4, 1, fp);

  return bmpHeader;
}

BMPInfoHeader *readInfoHeader(FILE *fp) {
  BMPInfoHeader *infoHeader =
      (BMPInfoHeader *)calloc(1, sizeof(struct BMPInfoHeader));

  fread(&infoHeader->size, 4, 1, fp);
  fread(&infoHeader->width, 4, 1, fp);
  fread(&infoHeader->height, 4, 1, fp);
  fread(&infoHeader->planes, 2, 1, fp);
  fread(&infoHeader->bitcount, 2, 1, fp);
  fread(&infoHeader->compression, 4, 1, fp);
  fread(&infoHeader->sizeImage, 4, 1, fp);
  fread(&infoHeader->xPelsPerMeter, 4, 1, fp);
  fread(&infoHeader->yPelsPerMeter, 4, 1, fp);
  fread(&infoHeader->colorsUsed, 4, 1, fp);
  fread(&infoHeader->colorsImportant, 4, 1, fp);
  // We ignore remaining v5 header fields (for now).

  return infoHeader;
}

void printHeaderInfo(BMPHeader *bmpHeader, BMPInfoHeader *infoHeader) {
  // For comma thousands separators.
  setlocale(LC_NUMERIC, "");

  // File header info.
  printf("Type bytes: %s\n", bmpHeader->type);
  printf("Size of file: %'d bytes\n", bmpHeader->size);
  printf("Offset of pixel data: %'d\n", bmpHeader->offset);

  // Info header info.
  printf("\nInfo header size: %d (124 means BITMAPV5HEADER)\n",
         infoHeader->size);
  printf("Image width: %'d\n", infoHeader->width);
  printf("Image height: %'d\n", infoHeader->height);
  printf("Color planes: %d\n", infoHeader->planes);
  printf("Bit count: %d\n", infoHeader->bitcount);
  printf("Compression: %'d\n", infoHeader->compression);
  printf("Size of raw bitmap: %'d\n", infoHeader->sizeImage);
  printf("X pixels per meter: %'d\n", infoHeader->xPelsPerMeter);
  printf("Y pixels per meter: %'d\n", infoHeader->yPelsPerMeter);
  printf("Colors used: %'d (0 = max for bit count)\n", infoHeader->colorsUsed);
  printf("Important colors: %'d\n", infoHeader->colorsImportant);
}

/**
 *  Functinos for managing structures.
 */

void free_PixelData(PixelData *pixData) {
  free(pixData->pixels);
  free(pixData);
}

/**
 *  Functions for reading, converting,
 *  and writing pixel data.
 */

PixelData *readPixels(BMPHeader *bmpHeader, BMPInfoHeader *infoHeader,
                      FILE *fp) {
  // Rows are padded to a multiple of 4 bytes.
  size_t rowSize = (infoHeader->bitcount / 8) * infoHeader->width;
  size_t padding = (4 - rowSize % 4) % 4;

  size_t numPixels = infoHeader->width * infoHeader->height;
  Pixel *pixels = (Pixel *)calloc(numPixels, sizeof(struct Pixel));

  size_t width = infoHeader->width;
  size_t height = infoHeader->height;

  fseek(fp, bmpHeader->offset, SEEK_SET);
  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      fread(&pixels[i * width + j].blue, 1, 1, fp);
      fread(&pixels[i * width + j].green, 1, 1, fp);
      fread(&pixels[i * width + j].red, 1, 1, fp);
    }
    fseek(fp, padding, SEEK_CUR);
  }

  PixelData *pixData = (PixelData *)calloc(1, sizeof(struct PixelData));
  pixData->pixels = pixels;
  pixData->width = infoHeader->width;
  pixData->height = infoHeader->height;
  return pixData;
}

PixelData *convertToGrayscale(PixelData *pixData) {
  Pixel *pix = pixData->pixels;
  size_t height = pixData->height;
  size_t width = pixData->width;

  size_t numPix = pixData->width * pixData->height;
  Pixel *gsPix = (Pixel *)calloc(numPix, sizeof(struct Pixel));
  PixelData *gsPixData = (PixelData *)calloc(1, sizeof(struct PixelData));
  gsPixData->pixels = gsPix;
  gsPixData->width = width;
  gsPixData->height = height;

  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      float n_fl = 0.299 * pix[i * width + j].red +
                   0.587 * pix[i * width + j].green +
                   0.114 * pix[i * width + j].blue;

      uint8_t n = (uint8_t)n_fl;
      gsPix[i * width + j].blue = n;
      gsPix[i * width + j].green = n;
      gsPix[i * width + j].red = n;
    }
  }

  return gsPixData;
}

void writeImageFile(FILE *inFile, BMPHeader *bmpHeader,
                    BMPInfoHeader *infoHeader, PixelData *gsPixels,
                    const char *fileName, size_t fNameLen) {
  char outFileName[fNameLen + 15];
  strncpy(outFileName, fileName, fNameLen);
  strncpy(outFileName + (fNameLen - 4), "_grayscale.bmp", 15);
  printf("\nWriting grayscale image to file: %s\n", outFileName);

  FILE *outFile = fopen(outFileName, "w");

  // Copy header directly from old file.
  size_t headerLen = bmpHeader->offset;
  uint8_t *someHeaderBytes = (uint8_t *)calloc(headerLen, sizeof(uint8_t));
  fseek(inFile, 0, SEEK_SET);
  fread(someHeaderBytes, sizeof(uint8_t), headerLen, inFile);
  fwrite(someHeaderBytes, sizeof(uint8_t), headerLen, outFile);

  // Prepare to write pixels.
  size_t height = gsPixels->height;
  size_t width = gsPixels->width;
  Pixel *pix = gsPixels->pixels;

  // Rows are padded to a multiple of 4 bytes.
  size_t rowSize = (infoHeader->bitcount / 8) * width;
  size_t padding = (4 - rowSize % 4) % 4;

  // Write grayscale pixels into file.
  for (size_t i = 0; i < height; i++) {
    for (size_t j = 0; j < width; j++) {
      Pixel *curPix = pix + (i * width + j);
      fwrite(&curPix->blue, 1, 1, outFile);
      fwrite(&curPix->green, 1, 1, outFile);
      fwrite(&curPix->red, 1, 1, outFile);
    }

    // Pad row as necessary.
    for (size_t n = 0; n < padding; n++) {
      uint8_t pad = 0;
      fwrite(&pad, 1, 1, outFile);
    }
  }

  // Cleanup.
  free(someHeaderBytes);
  fclose(outFile);
}
