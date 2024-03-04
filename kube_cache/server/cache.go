package server

import (
	"errors"
	"log"

	"github.com/CosmicKube/kube_cache/aiStuff"
	"github.com/CosmicKube/kube_cache/metrics"
	"github.com/CosmicKube/kube_cache/model"
	"github.com/gin-gonic/gin"
)

type Server struct {
	Database *model.Database
	Ai       *aiStuff.KubeAi
	Metrics  *metrics.Metrics
}

// 24 hours
const cacheControlHeader = "max-age=86400, public"

func New(metrics *metrics.Metrics, database *model.Database, ai *aiStuff.KubeAi) *Server {
	return &Server{Database: database, Ai: ai, Metrics: metrics}
}

func (s *Server) Use(engine *gin.Engine) {
	engine.GET("/cache_metrics", s.CacheMetrics)
	engine.GET("/kubes", s.GetAllKubes)
	engine.GET("/kubeById/:id", s.GetKube)
	engine.GET("/kubeRecipes", s.GetAllKubeRecipes)
	engine.GET("/kubeImageById/:id", s.GetKubeImage)
	engine.GET("/private/kubeImageByIdNew/:id", s.RegenerateImage)
	engine.GET("/private/kubeRecipeByIds/:id1/:id2", s.GetKubeRecipe)

	// index
	engine.GET("/", s.Index)
	engine.POST("/", s.IndexPost)
}

func (s *Server) CacheMetrics(c *gin.Context) {
	c.Data(200, "text/plain", []byte(s.Metrics.String()))
}

func (s *Server) RegenerateImage(c *gin.Context) {
	id := c.Param("id")
	kube, err := s.Database.GetKube(id)
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}

	image, err := s.Ai.GenerateDalleForKube(kube.Name)
	if err != nil {
		log.Printf("Error generating Dalle for kube: %s", err)
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}

	err = s.Database.SetKubeImage(kube, image)
	if err != nil {
		log.Printf("Cannot save kube image: %s", err)
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, gin.H{"message": "Image regenerated"})
}

func (s *Server) GetAllKubeRecipes(c *gin.Context) {
	recipes, err := s.Database.GetAllKubeRecipes()
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, recipes)
}

func (s *Server) GetKubeImage(c *gin.Context) {
	id := c.Param("id")
	image, err := s.Database.GetKubeImage(id)
	if err != nil {
		log.Printf("Cannot get kube image: %s", err)
		c.JSON(500, gin.H{"error": err.Error()})
		s.Metrics.IncrementCacheMisses()
		return
	}
	c.Data(200, "image/png", image)
	c.Header("Cache-Control", cacheControlHeader)
	s.Metrics.IncrementCacheHits()
	s.Metrics.IncrementImagesRetrieved()
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
		s.Metrics.IncrementCacheMisses()
		return
	}
	c.JSON(200, kube)
	c.Header("Cache-Control", cacheControlHeader)
	s.Metrics.IncrementCacheHits()
}

func (s *Server) GetKubeRecipe(c *gin.Context) {
	id1 := c.Param("id1")
	id2 := c.Param("id2")
	outputId, err := s.craft(c, id1, id2)
	if err != nil {
		log.Printf("Error crafting kube: %s", err)
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}
	c.JSON(200, outputId)
}

func (s *Server) craft(c *gin.Context, id1, id2 string) (model.KubeRecipe, error) {
	recipe, err := s.Database.GetKubeRecipe(id1, id2)
	if err != nil {
		s.Metrics.IncrementCacheMisses()
		kube1, err := s.Database.GetKube(id1)
		if err != nil {
			return model.KubeRecipe{}, errors.New("Cannot get kube 1")
		}

		kube2, err := s.Database.GetKube(id2)
		if err != nil {
			return model.KubeRecipe{}, errors.New("Cannot get kube 2")
		}

		newKubeId, err := s.Ai.GenerateKubeRecipe(kube1.Name, kube2.Name)
		if err != nil {
			log.Printf("Cannot generate kube recipe: %s", err)
			return model.KubeRecipe{}, errors.New("Cannot generate kube recipe")
		}

		image, err := s.Ai.GenerateDalleForKube(newKubeId)
		if err != nil {
			log.Printf("Error generating Dalle for kube: %s", err)
			return model.KubeRecipe{}, errors.New("Error generating Dalle for kube")
		}

		err = s.Database.SetKubeRecipe(kube1, kube2, newKubeId, image)
		if err != nil {
			log.Printf("Cannot save kube recipe: %s", err)
			return model.KubeRecipe{}, errors.New("Cannot save kube recipe")
		}

		recipe, err = s.Database.GetKubeRecipe(id1, id2)
		if err != nil {
			log.Printf("Cannot get kube recipe: %s", err)
			return model.KubeRecipe{}, errors.New("Cannot get kube recipe")
		}

	} else {
		s.Metrics.IncrementCacheHits()
	}
	c.Header("Cache-Control", cacheControlHeader)
	return recipe, nil
}
