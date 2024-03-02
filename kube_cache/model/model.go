package model

import (
	"log"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type Kube struct {
	Name  string `json:"name"`
	Id    string `gorm:"primaryKey" json:"id"`
	Image []byte `json:"image"`
}

type KubeRecipe struct {
	Id          string `gorm:"primaryKey" json:"id"`
	Output      Kube   `gorm:"foreignKey:Id;references:Id" json:"output"`
	Ingredients []Kube `gorm:"index" json:"ingredients"`
}

type Database struct {
	Db *gorm.DB
}

func New(url string) *Database {
	log.Println("Connecting to database...")
	db, err := gorm.Open(postgres.Open(url), &gorm.Config{})
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
