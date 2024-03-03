package main

import (
	"log"
	"os"

	"github.com/CosmicKube/kube_cache/aiStuff"
	"github.com/CosmicKube/kube_cache/model"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
	ginpromehteus "github.com/zsais/go-gin-prometheus"
)

func main() {
	log.SetFlags(log.Lshortfile | log.Ldate | log.Ltime)

	log.Println("Reading config...")
	err := godotenv.Load()
	if err != nil {
		log.Fatal(err)
	}

	log.Println("Using configuration for database...")
	model.New(os.Getenv("DATABASE_URL"))

	log.Println("Creating AI client...")
	ai := aiStuff.New(os.Getenv("OPENAI_ENDPOINT"),
		os.Getenv("OPENAI_API_KEY"),
		os.Getenv("OPENAI_MODEL_ID"))

	data, err := ai.GenerateKubeRecipe("water", "laptop")
	if err != nil {
		log.Fatal(err)
	}
  log.Printf("Data: %s", data)

	log.Println("Starting server...")
	router := gin.Default()

	p := ginpromehteus.NewPrometheus("gin")
	p.Use(router)

	router.Use(cors.New(cors.Config{
		AllowOrigins: []string{"*"},
		AllowHeaders: []string{"*"},
		AllowOriginFunc: func(_ string) bool {
			return true
		},
	}))

	log.Fatal(router.Run("0.0.0.0:8080"))
}
