package main

import (
	"log"
	"os"

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

  log.Println("Starting server...")
  router := gin.Default()

	p := ginpromehteus.NewPrometheus("gin")

  router.Use(cors.New(cors.Config{
    AllowOrigins:     []string{"*"},
    AllowHeaders:     []string{"*"},
    AllowOriginFunc: func(_ string) bool {
      return true
    },
  }))
	p.Use(router)
}
