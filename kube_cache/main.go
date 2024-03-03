package main

import (
	"log"
	"os"
	"strings"

	"github.com/CosmicKube/kube_cache/aiStuff"
	"github.com/CosmicKube/kube_cache/metrics"
	"github.com/CosmicKube/kube_cache/model"
	"github.com/CosmicKube/kube_cache/server"
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
		log.Println(err)
	}

  metrics := metrics.New()

	log.Println("Creating AI client...")
	ai := aiStuff.New(metrics, os.Getenv("OPENAI_ENDPOINT"),
		os.Getenv("OPENAI_API_KEY"),
		os.Getenv("OPENAI_MODEL_ID"))

	log.Println("Using configuration for database...")
	database := model.New(metrics, ai, os.Getenv("DATABASE_URL"))

	log.Println("Starting server...")
	router := gin.Default()

	p := ginpromehteus.NewPrometheus("gin")
	p.ReqCntURLLabelMappingFn = func(c *gin.Context) string {
		url := c.Request.URL.Path
		for _, p := range c.Params {
			if p.Key == "id" {
				url = strings.Replace(url, p.Value, ":id", 1)
				break
			} else if p.Key == "id1" {
				url = strings.Replace(url, p.Value, ":id1", 1)
				break
			} else if p.Key == "id2" {
				url = strings.Replace(url, p.Value, ":id2", 1)
				break
			}
		}
		return url
	}
  
	p.Use(router)

	router.Use(cors.New(cors.Config{
		AllowOrigins: []string{"*"},
		AllowHeaders: []string{"*"},
		AllowOriginFunc: func(_ string) bool {
			return true
		},
	}))

	log.Println("Start up the API")
	server := server.New(metrics, database, ai)
	server.Use(router)
	log.Fatal(router.Run("0.0.0.0:8080"))
}
