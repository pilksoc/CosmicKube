package server

import (
	"log"

	"github.com/CosmicKube/kube_cache/aiStuff"
	"github.com/CosmicKube/kube_cache/model"
	"github.com/gin-gonic/gin"
)

type Server struct {
	Database *model.Database
	Ai       *aiStuff.KubeAi
}

func New(database *model.Database, ai *aiStuff.KubeAi) *Server {
	return &Server{Database: database, Ai: ai}
}

func (s *Server) Use(engine *gin.Engine) {
	engine.GET("/kubes", s.GetAllKubes)
	engine.GET("/kubeRecipes", s.GetAllKubeRecipes)
	engine.GET("/kubeById/:id", s.GetKube)
	engine.GET("/kubeRecipeByIds/:id1/:id2", s.GetKubeRecipe)
}

func (s *Server) GetAllKubeRecipes(c *gin.Context) {
	recipes, err := s.Database.GetAllKubeRecipes()
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, recipes)
}

func (s *Server) GetAllKubes(c *gin.Context) {
	kubes, err := s.Database.GetAllKubes()
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, kubes)
}

func (s *Server) GetKube(c *gin.Context) {
	id := c.Param("id")
	kube, err := s.Database.GetKube(id)
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, kube)
}

// 24 hours
const cacheControlHeader = "max-age=86400"

func (s *Server) GetKubeRecipe(c *gin.Context) {
	id1 := c.Param("id1")
	id2 := c.Param("id2")
	recipe, err := s.Database.GetKubeRecipe(id1, id2)
	if err != nil {
		kube1, err := s.Database.GetKube(id1)
		if err != nil {
			log.Printf("Cannot get kube: %s", err)
			c.JSON(500, gin.H{"error": err.Error()})
			return
		}

		kube2, err := s.Database.GetKube(id2)
		if err != nil {
			log.Printf("Cannot get kube: %s", err)
			c.JSON(500, gin.H{"error": err.Error()})
			return
		}

		newKube, err := s.Ai.GenerateKubeRecipe(kube1.Name, kube2.Name)
		if err != nil {
			log.Printf("Cannot generate kube recipe: %s", err)
			c.JSON(500, gin.H{"error": err.Error()})
			return
		}

    image, err := s.Ai.GenerateDalleForKube(newKube)
    if err != nil {
      log.Printf("Error generating Dalle for kube: %s", err)
      c.JSON(500, gin.H{"error": err.Error()})
      return
    }

		err = s.Database.SetKubeRecipe(kube1, kube2, newKube, image)
		if err != nil {
			log.Printf("Cannot save kube recipe: %s", err)
			c.JSON(500, gin.H{"error": err.Error()})
			return
		}

		recipe, err = s.Database.GetKubeRecipe(id1, id2)
		if err != nil {
			log.Printf("Cannot get kube recipe: %s", err)
			c.JSON(500, gin.H{"error": err.Error()})
			return
		}
	}
	c.Header("Cache-Control", cacheControlHeader)
	c.JSON(200, recipe)
}
