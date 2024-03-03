package server

import (
	"github.com/CosmicKube/kube_cache/aiStuff"
	"github.com/CosmicKube/kube_cache/model"
	"github.com/gin-gonic/gin"
)

type Server struct {
	Database *model.Database
  Ai *aiStuff.KubeAi
}

func New(database *model.Database, ai *aiStuff.KubeAi) *Server {
  return &Server{Database: database, Ai: ai}
}

func (s *Server) Use(engine *gin.Engine) {
}
