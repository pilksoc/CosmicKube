package aiStuff

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"time"

	"github.com/Azure/azure-sdk-for-go/sdk/ai/azopenai"
	"github.com/Azure/azure-sdk-for-go/sdk/azcore"
	"github.com/Azure/azure-sdk-for-go/sdk/azcore/policy"
	"github.com/Azure/azure-sdk-for-go/sdk/azcore/to"
)

type KubeAi struct {
	Client  *azopenai.Client
	ModelId string
}

func New(endpoint, apiKey, modelId string) *KubeAi {
	clientOptions := azopenai.ClientOptions{
		ClientOptions: policy.ClientOptions{
			Retry: policy.RetryOptions{TryTimeout: time.Second * 10,
				MaxRetries: 3,
			},
		},
	}

	cred := azcore.NewKeyCredential(apiKey)
	client, err := azopenai.NewClientWithKeyCredential(endpoint, cred, &clientOptions)
	if err != nil {
		log.Fatalf("Error creating openai client: %s", err)
	}
	return &KubeAi{
		Client: client,
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

func (ai *KubeAi) GenerateKubeRecipe(kubeName1, kubeName2 string) (string, error) {
	log.Printf("Generating kube recipe for %s and %s", kubeName1, kubeName2)

	messages := []azopenai.ChatRequestMessageClassification{
		// You set the tone and rules of the conversation with a prompt as the system role.
		&azopenai.ChatRequestSystemMessage{Content: to.Ptr(baseRequest)},
		&azopenai.ChatRequestUserMessage{Content: azopenai.NewChatRequestUserMessageContent(ai.generateAiPrompt(kubeName1, kubeName2))},
	}

	ctx := context.Background()
	resp, err := ai.Client.GetChatCompletions(ctx, azopenai.ChatCompletionsOptions{
		Messages:       messages,
		DeploymentName: &ai.ModelId,
	}, nil)
	if err != nil {
		log.Printf("Error generating kube recipe: %s", err)
		return "", err
	}

	responseMessage := resp.Choices[0].Message.Content
	var aiResp aiResp
	err = json.Unmarshal([]byte(*responseMessage), &aiResp)
	if err != nil {
		log.Printf("Error unmarshalling AI response: %s, returning it raw", err)
		return *responseMessage, nil
	}
	return aiResp.Name, nil
}
