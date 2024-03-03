package aiStuff

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

func (ai *KubeAi) generateDallePrompt(kubeName string) string {
	return fmt.Sprint("A flat tillable square texture that represents %s in pixel art for a video game, covers the full image", kubeName)
}

const (
	dalleN    = 1
	dalleSize = "1024x1024"
)

type data struct {
	Url string `json:"url"`
}

type DalleResp struct {
	Data []data `json:"data"`
}

type dalleRequest struct {
	Prompt     string `json:"prompt"`
	NumSamples int    `json:"n"`
	Size       string `json:"size"`
}

func (ai *KubeAi) GenerateDalleForKube(kubeName string) ([]byte, error) {
	log.Printf("Generating Dalle for kube: %s", kubeName)
	prompt := ai.generateDallePrompt(kubeName)
	dalleReq := dalleRequest{
		Prompt:     prompt,
		NumSamples: dalleN,
		Size:       dalleSize,
	}

	reqBytes, err := json.Marshal(dalleReq)
	if err != nil {
		log.Printf("Error marshalling dalle request: %s", err)
		return nil, err
	}

	url := fmt.Sprintf("%s/openai/deployments/Dalle3/images/generations?api-version=2024-02-15-preview", ai.Endpoint)
	req, err := http.NewRequest("POST", url, bytes.NewReader(reqBytes))
	if err != nil {
		log.Printf("Cannot get a message or something like that mate: %s", err)
		return nil, err
	}
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("api-key", ai.Apikey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Printf("Cannot get a message or something like that mate: %s", err)
		return nil, err
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Printf("A nightmare has occurred, I cannot get any data :( %s", err)
		return nil, err
	}

	var dalleResp DalleResp
	err = json.Unmarshal(body, &dalleResp)
	if err != nil {
		log.Printf("Error unmarshalling dalle response: %s", err)
		return nil, err
	}

	log.Println("Downloading dalle response")
	resp, err = http.Get(dalleResp.Data[0].Url)
	if err != nil {
		log.Printf("Error getting image from url: %s", err)
		return nil, err
	}

	body, err = io.ReadAll(resp.Body)
	if err != nil {
		log.Printf("Error reading image from response: %s", err)
		return nil, err
	}

	return body, nil
}