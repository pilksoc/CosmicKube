package server

import (
	"fmt"
	"log"

	"github.com/CosmicKube/kube_cache/model"
	"github.com/gin-gonic/gin"
)

func (s *Server) allKubesSelect(kubes []model.Kube) string {
	options := ""
	for _, kube := range kubes {
		options += fmt.Sprintf(`<option value="%s">%s</option>`, kube.Id, kube.Name)
	}
	return options
}

const (
	op1 = "op1"
	op2 = "op2"
)

func (s *Server) IndexPost(c *gin.Context) {
	id1 := c.PostForm(op1)
	id2 := c.PostForm(op2)
	recipe, err := s.craft(c, id1, id2)
	if err != nil {
		log.Printf("Cannot craft kube: %s", err)
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}

	kube, err := s.Database.GetKube(recipe.Output.String())
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}

	html := fmt.Sprintf(`<!DOCTYPE HTML>
<head>
  <Title>New Kube | Kube Cache</Title>
</head>
<body>
  <h1>New Kube: %s</h1>
  <p><a href='../'>Back</a></p>
  <img src="../kubeImageById/%s" style="max-width: 100px; max-height: 100px;" loading="lazy" alt="%s"/>
  <p><a href="../private/kubeImageByIdNew/%s">Regenerate Image</a></p>
</body>`,
		kube.Name,
		kube.Id,
		kube.Id,
		kube.Id)

	c.Data(200, "text/html", []byte(html))
}

func (s *Server) Index(c *gin.Context) {
	kubes, err := s.Database.GetAllKubes()
	if err != nil {
		c.JSON(500, gin.H{"error": err.Error()})
		return
	}

	kubesHtml := ""
	for _, kube := range kubes {
		kubesHtml += fmt.Sprintf(`<div style="max-width: 100px;">
      <h3 style="word-wrap: break-word">%s</h3>
      <a href="./private/kubeImageByIdNew/%s">Regenerate Image</a>
      <img src="./kubeImageById/%s" style="max-width: 100px; max-height: 100px" loading="lazy" alt="%s"/>
    </div>`, kube.Name, kube.Id, kube.Id, kube.Name)
	}

	selects := s.allKubesSelect(kubes)
	crafterHtml := fmt.Sprintf(`<form action="./private/craft" method="post">
  <label for="%s">Name</label>
  <select name="%s">
    %s
  </select>

  <label for="%s">Name</label>
  <select name="%s">
    %s
  </select>

  <button type="submit">Craft</button>
</form>
  `,
		op1, op1,
		selects,
		op2, op2,
		selects)

	body := fmt.Sprintf(`<!DOCTYPE html>
<head>
  <title>Kube Cache</title>
</head>
<body style="display: flex; margin: 0px; flex-direction: column; max-width: 100vw">
  <h1>Kube Cache</h1>
  <h2>Endpoints</h2>
  <div style="display: flex; flex-direction: row; gap: 10px; flex-wrap: wrap; justify-content: between">
    <ul style="min-width: 200px">
      <li><a href="./">/.</a></li>
      <li><a href="./kubes">/kubes</a></li>
      <li><a href="./kubeById/:id">/kubeById/:id</a></li>
      <li><a href="./kubeRecipes">/kubeRecipes</a></li>
      <li><a href="./kubeImageById/:id">/kubeImageById/:id</a></li>
      <li><a href="./private/kubeRecipeByIds/:id1/:id2">/private/kubeRecipeByIds/:id1/:id2</a></li>
      <li><a href="./private/kubeImageByIdNew/:id">/private/kubeImageByIdNew/:id</a></li>
      <li><a href="./private/craft">/private/craft</a></li>
      <li><a href="./cache_metrics">/cache_metrics</a></li>
      <li><a href="./metrics">/metrics</a></li>
    </ul>
    <div style="min-width: 200px">
      <p>
        Metrics are collected on Grafana, private endpoints requires the same proxy authentication.
      </p>
    </div>
  </div>
  <h2>Create New Kube</h2>
    %s
  <h2>Kubes</h2>
  <div style="display: flex; flex-direction: row; flex-wrap: wrap; gap: 10px;  align-items: center; align-content: center">
    %s
  </div>
  <p>End of cache</p>
</body>`, crafterHtml, kubesHtml)

	c.Data(200, "text/html", []byte(body))
}
