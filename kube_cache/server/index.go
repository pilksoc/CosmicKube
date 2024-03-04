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
		kubesHtml += fmt.Sprintf(`<div style="max-width: 100px;">
      <h3 class="word-wrap: break-word;">%s</h3>
      <a href="./kubeImageByIdNew/%s">Regenerate Image</a>
      <img src="./kubeImageById/%s" style="max-width: 100px; max-height: 100px;" loading="lazy" alt="%s"/>
    </div>`, kube.Name, kube.Id, kube.Id, kube.Name)
	}

	body := fmt.Sprintf(`<!DOCTYPE html>
<head>
  <title>Kube Cache</title>
</head>
<body>
  <h1>Kube Cache</h1>
  <h2>Endpoints</h2>
  <ul>
    <li><a href="./">/.</a></li>
    <li><a href="./kubes">/kubes</a></li>
    <li><a href="./kubeById/:id">/kubeById/:id</a></li>
    <li><a href="./kubeImageById/:id">/kubeImageById/:id</a></li>
    <li><a href="./kubeRecipes">/kubeRecipes</a></li>
    <li><a href="./kubeRecipeByIds/:id1/:id2">/kubeRecipeByIds/:id1/:id2</a></li>
    <li><a href="./cache_metrics">/cache_metrics</a></li>
    <li><a href="./metrics">/metrics</a></li>
  </ul>
  <h2>Kubes</h2>
  <div style="display: flex; flex-direction: row; flex-wrap: wrap; gap: 10px;  align-items: center; align-content: center;">
    %s
  </div>
  <p>End of cache</p>
</body>`, kubesHtml)

	c.Data(200, "text/html", []byte(body))
}
