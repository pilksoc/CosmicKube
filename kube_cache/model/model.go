package model

import (
	"log"

	"github.com/google/uuid"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type Kube struct {
	Name string    `json:"name"`
	Id   uuid.UUID `gorm:"primaryKey" json:"id"`
	// Image []byte    `json:"image"`
}

type KubeRecipe struct {
	Id          uuid.UUID         `gorm:"primaryKey" json:"id"`
	Output      uuid.UUID         `json:"outputId" gorm:"index:idx_output_id"`
	OutputKube  *Kube             `json:"outputKube" gorm:"foreignKey:Output;references:Id"`
	Ingredients []KubeRecipeLines `json:"ingredients"`
}

type KubeRecipeLines struct {
	KubeRecipeId uuid.UUID `json:"kube_recipe_id" gorm:"index:idx_kube_recipe_id"`
	KubeId       string    `json:"kube_id" gorm:"index:idx_kube_id"`
	Kube         *Kube     `json:"kube" gorm:"foreignKey:KubeId;references:Id"`
}

type Database struct {
	Db *gorm.DB
}

func New(url string) *Database {
	log.Println("Connecting to database...")
	db, err := gorm.Open(postgres.Open(url), &gorm.Config{
		PrepareStmt: true,
	})
	if err != nil {
		log.Fatal(err)
	}

	log.Println("Database connection established")
	log.Println("Migrating database...")
	err = db.AutoMigrate(&Kube{}, &KubeRecipe{})
	if err != nil {
		log.Fatal(err)
	}

	log.Println("Migrated database successfully")
	return &Database{Db: db}
}

func (db *Database) GetKube(id string) (Kube, error) {
	var kube Kube
	result := db.Db.First(&kube, "id = ?", id)
	return kube, result.Error
}

func (db *Database) SetKube(kube Kube) error {
	result := db.Db.Create(&kube)
	return result.Error
}

func (db *Database) GetKubeRecipe(id string) (KubeRecipe, error) {
	var recipe KubeRecipe
	result := db.Db.Preload("OutputKube").Preload("Ingredients.Kube").First(&recipe, "id = ?", id)
	return recipe, result.Error
}

func (db *Database) SetKubeRecipe(recipe KubeRecipe) error {
	result := db.Db.Create(&recipe)
	return result.Error
}
