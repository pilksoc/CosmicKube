package model

import (
	"log"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type Kube struct {
  Name string 
  Id string `gorm:"primaryKey"` 
}

type KubeRecipe struct {
  Id string `gorm:"primaryKey"`
  Output Kube
  Ingredients []Kube `gorm:"index"`
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
