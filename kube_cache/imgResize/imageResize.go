package imgresize

import (
	"bytes"
	"image/jpeg"
	"image/png"
	"io"
	"log"
	"os"

	"github.com/nfnt/resize"
)

const (
	dimX = 100
	dimY = 100
)

func ResizeImage(img []byte) ([]byte, error) {
	log.Println("Creating temp file for image resize")
	outputFile, err := os.CreateTemp(".", "resized-*.jpg")
	if err != nil {
		log.Printf("Error creating temp file: %s", err)
		return nil, err
	}
	defer outputFile.Close()

	log.Printf("Resizing image, original size: %d", len(img))
	image, err := png.Decode(bytes.NewReader(img))
	if err != nil {
		log.Printf("Error decoding image: %s", err)
		return nil, err
	}

	newImg := resize.Resize(dimX, dimY, image, resize.Lanczos3)
	if err != nil {
		log.Printf("Error encoding image: %s", err)
		return nil, err
	}

	log.Println("Encoding resized image")
	err = jpeg.Encode(outputFile, newImg, nil)
	if err != nil {
		log.Printf("Error encoding image: %s", err)
		return nil, err
	}

	outputFile.Seek(0, 0)
  newImageByes, err := io.ReadAll(outputFile)
  if err != nil {
    log.Printf("Error reading resized image: %s", err)
    return nil, err
  }

  log.Printf("Resized image size: %d", len(newImageByes))
  return newImageByes, nil
}
