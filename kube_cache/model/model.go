package model

import (
	"log"

	"github.com/CosmicKube/kube_cache/aiStuff"
	"github.com/google/uuid"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func SortKubesStr(kube1, kube2 *string) {
	if *kube1 > *kube2 {
		*kube1, *kube2 = *kube2, *kube1
	}
}

func SortKubesUuid(kube1, kube2 *uuid.UUID) {
	if kube1.String() > kube2.String() {
		*kube1, *kube2 = *kube2, *kube1
	}
}

type Kube struct {
	Name  string    `gorm:"unique;not null" json:"name"`
	Id    uuid.UUID `gorm:"primaryKey" json:"id"`
	Image []byte    `gorm:"not null" json:"image"`
}

type KubeRecipe struct {
	Id         uuid.UUID `gorm:"primaryKey" json:"id"`
	Output     uuid.UUID `json:"outputId" gorm:"index:idx_output_id;not null"`
	OutputKube *Kube     `json:"outputKube" gorm:"foreignKey:Output;references:Id"`

	Kube1Id uuid.UUID `json:"kube1_id" gorm:"index:idx_kube1_id;not null"`
	Kube1   *Kube     `json:"-" gorm:"foreignKey:Kube1Id;references:Id"`

	Kube2Id uuid.UUID `json:"kube2_id" gorm:"index:idx_kube2_id;not null"`
	Kube2   *Kube     `json:"-" gorm:"foreignKey:Kube2Id;references:Id"`
}

type Database struct {
	Db *gorm.DB
}

func New(ai *aiStuff.KubeAi, url string) *Database {
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
	database.seed(ai)

	log.Println("Migrated database successfully")
	return database
}

func (db *Database) GetKube(id string) (Kube, error) {
	var kube Kube
	result := db.Db.First(&kube, "id = ?", id)
	return kube, result.Error
}

func (db *Database) GetAllKubeRecipes() ([]KubeRecipe, error) {
	var recipes []KubeRecipe
	result := db.Db.Find(&recipes)
	return recipes, result.Error
}

func (db *Database) GetAllKubes() ([]Kube, error) {
	var kubes []Kube
	result := db.Db.Find(&kubes)
	return kubes, result.Error
}

func (db *Database) GetKubeRecipe(kube1, kube2 string) (KubeRecipe, error) {
	SortKubesStr(&kube1, &kube2)

	var recipe KubeRecipe
	result := db.Db.First(&recipe, "kube1_id = ? AND kube2_id = ?", kube1, kube2)
	return recipe, result.Error
}

func (db *Database) SetKubeRecipe(kube1, kube2 Kube, newKube string, image []byte) error {
	kube1Id := kube1.Id
	kube2Id := kube2.Id
	SortKubesUuid(&kube1Id, &kube2Id)

	log.Printf("Setting kube recipe: %s + %s = %s", kube1.Name, kube2.Name, newKube)
	err := db.Db.Transaction(func(tx *gorm.DB) error {
		newKubeObject := Kube{Name: newKube,
			Id:    uuid.New(),
			Image: image}
		err := tx.Create(&newKubeObject).Error
		if err != nil {
			log.Printf("Cannot create new kube: %s", err)
			return err
		}

		recipe := KubeRecipe{
			Id:      uuid.New(),
			Output:  newKubeObject.Id,
			Kube1Id: kube1Id,
			Kube2Id: kube2Id,
		}
		err = tx.Create(&recipe).Error
		if err != nil {
			log.Printf("Cannot create new kube recipe: %s", err)
			return err
		}
		return nil
	})

	if err != nil {
		log.Print("Saving the kube recipe failed")
		return err
	}
	return nil
}
