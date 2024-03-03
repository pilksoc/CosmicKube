package model

import (
	"log"

	"github.com/google/uuid"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type Kube struct {
	Name string    `gorm:"unique" json:"name"`
	Id   uuid.UUID `gorm:"primaryKey" json:"id"`
	// Image []byte    `json:"image"`
}

type KubeRecipe struct {
	Id          uuid.UUID         `gorm:"primaryKey" json:"id"`
	Output      uuid.UUID         `json:"outputId" gorm:"index:idx_output_id"`
	OutputKube  *Kube             `json:"outputKube" gorm:"foreignKey:Output;references:Id"`
	Ingredients []KubeRecipeLines `json:"ingredients"`

	Kube1Id string `json:"kube1_id" gorm:"index:idx_kube1_id"`
	Kube1   *Kube  `json:"kube1" gorm:"foreignKey:Kube1Id;references:Id"`

	Kube2Id string `json:"kube2_id" gorm:"index:idx_kube2_id"`
	Kube2   *Kube  `json:"kube2" gorm:"foreignKey:Kube2Id;references:Id"`
}

type KubeRecipeLines struct {
	KubeRecipeId uuid.UUID `json:"kube_recipe_id" gorm:"index:idx_kube_recipe_id"`
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

	database := &Database{Db: db}
	log.Println("Seeding database...")
	database.seed()

	log.Println("Migrated database successfully")
	return database
}

func (db *Database) GetKube(id string) (Kube, error) {
	var kube Kube
	result := db.Db.First(&kube, "id = ?", id)
	return kube, result.Error
}

func (db *Database) GetKubeRecipe(kube1, kube2 string) (KubeRecipe, error) {
	var recipe KubeRecipe
	result := db.Db.First(&recipe, "kube1 = ? AND kube2 = ?", kube1)
	return recipe, result.Error
}

func (db *Database) SetKubeRecipe(kube1, kube2 Kube, newKube string) error {
	log.Printf("Setting kube recipe: %s + %s = %s", kube1.Name, kube2.Name, newKube)
	err := db.Db.Transaction(func(tx *gorm.DB) error {
		newKubeObject := Kube{Name: newKube, Id: uuid.New()}
		err := tx.Create(&newKubeObject).Error
		if err != nil {
			log.Printf("Cannot create new kube: %s", err)
			return err
		}

		recipe := KubeRecipe{
			Id:     uuid.New(),
			Output: newKubeObject.Id,
			Kube1:  &kube1,
			Kube2:  &kube2,
		}
		err = tx.Create(&recipe).Error
		if err != nil {
			log.Printf("Cannot create new kube recipe: %s", err)
			return err
		}
		tx.Commit()
		return nil
	})

	if err != nil {
		log.Print("Saving the kube recipe failed")
		return err
	}
	return nil
}
