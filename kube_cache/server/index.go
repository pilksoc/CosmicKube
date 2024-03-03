package server

import (
	"fmt"

	"github.com/gin-gonic/gin"
)

func (s *Server) Index(c *gin.Context) {
	kubes, err := s.Database.GetAllKubes()
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}

	kubesHtml := ""
	for _, kube := range kubes {
		kubesHtml += fmt.Sprintf(`<div style="display: flex; flex-direction: column; align-items: center;">
      <h3>%s</h3>
      <img src="/kubeImageById/%s" style="max-width: 200px; max-height: 200px;"/>
    </div>`, kube.Name, kube.Id)
	}

	body := fmt.Sprint(`<!DOCTYPE html>
<head>
  <title>Kube Cache</title>
</head>
<body>
  <h1>Kube Cache</h1>
  <h2>Endpoints</h2>  
  <ul>
    <li><a href="/kubes">/kubes</a></li>
    <li><a href="/kubeById/:id">/kubeById/:id</a></li>
    <li><a href="/kubeImageById/:id">/kubeImageById/:id</a></li>
    <li><a href="/kubeRecipes">/kubeRecipes</a></li>
    <li><a href="/kubeRecipeByIds/:id1/:id2">/kubeRecipeByIds/:id1/:id2</a></li>
  </ul>
  <h2>Kubes</h2>
  <div style="display: flex; flex-direction: row; flex-wrap: wrap; gap: 10px; width: 100%">
    %s
  </div>
</body>`, kubesHtml)

	c.Data(200, "text/html", []byte(body))
}
