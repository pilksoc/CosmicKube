package aiStuff

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

type KubeAi struct {
	Endpoint, Apikey, ModelId string
}

func New(endpoint, apiKey, modelId string) *KubeAi {
	return &KubeAi{
		Endpoint: endpoint,
		Apikey:   apiKey,
		ModelId:  modelId,
	}
}

const baseRequest = `You are a game master for crafting game.
You will be given an input in the form of multiple elements like; ["object1", "object2", ...]
Your response will be limited to a single element that is formed from the preceding element in the form of a JSON string like; {"name":"object3"}
You can be creative with your responses but it has to somewhat grounded.
If you can't figure one output, say {"name": "I dunno lmao"}.
Only generate the output for the input provided.
Separate the input from the output with a space.`

func (ai *KubeAi) generateAiPrompt(kubeName1, kubeName2 string) string {
	arr := []string{kubeName1, kubeName2}
	req, err := json.Marshal(arr)
	if err != nil {
		log.Printf("Error marshalling kube names: %s", err)
		return fmt.Sprintf("[\"%s\", \"%s\"]", kubeName1, kubeName2)
	}

	return string(req)
}

type aiResp struct {
	Name string `json:"name"`
}

type message struct {
	Content string `json:"content"`
}

type choice struct {
	Message message `json:"message"`
}

type openaiResp struct {
	Choices []choice `json:"choices"`
}

type aiMessage struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

const (
	topP             = 0.95
	maxTokens        = 800
	temperature      = 0.7
	presencePenalty  = 0
	frequencyPenalty = 0
)

type aiReq struct {
	Messages         []aiMessage `json:"messages"`
	MaxTokens        float32     `json:"max_tokens"`
	Temperature      float32     `json:"temperature"`
	FrequencyPenalty float32     `json:"frequency_penalty"`
	PresencePenalty  float32     `json:"presence_penalty"`
	TopP             float32     `json:"top_p"`
}

func (ai *KubeAi) GenerateKubeRecipe(kubeName1, kubeName2 string) (string, error) {
	url := fmt.Sprintf("%s/openai/deployments/%s/chat/completions?api-version=2024-02-15-preview", ai.Endpoint, ai.ModelId)

	postReq := aiReq{
		Messages: []aiMessage{
			{
				Role:    "system",
				Content: baseRequest,
			},
			{
				Role:    "user",
				Content: ai.generateAiPrompt(kubeName1, kubeName2),
			},
		},
		MaxTokens:        maxTokens,
		Temperature:      temperature,
		FrequencyPenalty: frequencyPenalty,
		TopP:             topP,
		PresencePenalty:  presencePenalty,
	}
	postBody, err := json.Marshal(postReq)
	if err != nil {
		log.Printf("THIS IS BAD %s", err)
		return "", err
	}

	req, err := http.NewRequest("POST", url, bytes.NewReader(postBody))
	if err != nil {
		log.Printf("Cannot get a message or something like that mate: %s", err)
		return "", err
	}
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("api-key", ai.Apikey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Printf("Cannot get a message or something like that mate: %s", err)
		return "", err
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Printf("A nightmare has occurred, I cannot get any data :( %s", err)
		return "", err
	}

	var aiResponse openaiResp
	err = json.Unmarshal([]byte(body), &aiResponse)
	if err != nil {
		log.Printf("The silly server sent %s, this is very bad %s", body, err)
		return string(body), nil
	}

	actualLegitMessage := aiResponse.Choices[0].Message.Content

	var aiResp2 aiResp
	err = json.Unmarshal([]byte(actualLegitMessage), &aiResp2)
	if err != nil {
		log.Printf("The silly ai sent %s, this is very bad %s", actualLegitMessage, err)
		return actualLegitMessage, nil
	}

	if aiResp2.Name == "" {
		log.Printf("The silly ai sent %s, this is very bad", actualLegitMessage)
		return actualLegitMessage, nil
	}

	return aiResp2.Name, nil
}
