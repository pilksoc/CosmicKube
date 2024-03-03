package model

import (
	"github.com/google/uuid"
	"gorm.io/gorm"
	"log"
)

func insertKube(name string, tx *gorm.DB) error {
	log.Printf("Inserting kube: %s", name)
	kube := Kube{Name: name, Id: uuid.New()}
	return tx.Create(&kube).Error
}

func insertKubeRecipe(kube1, kube2, output string, tx *gorm.DB) error {
	log.Printf("Inserting kube recipe: %s + %s = %s", kube1, kube2, output)
	kube1Row := Kube{}
	kube2Row := Kube{}
	outputRow := Kube{}

	tx.First(&kube1Row, "name = ?", kube1)
	tx.First(&kube2Row, "name = ?", kube2)

	kube1Id := kube1Row.Id
	kube2Id := kube2Row.Id
	SortKubesUuid(&kube1Id, &kube2Id)

	err := insertKube(output, tx)
	if err != nil {
		return err
	}
	tx.First(&outputRow, "name = ?", output)

	kubeRecipe := KubeRecipe{
		Id:      uuid.New(),
		Output:  outputRow.Id,
		Kube1Id: kube1Id,
		Kube2Id: kube2Id,
	}

	return tx.Create(&kubeRecipe).Error
}

type recipe struct {
	kube1  string
	kube2  string
	output string
}

func (d *Database) seed() {
	err := d.Db.Transaction(func(tx *gorm.DB) error {
		kubes := []string{
			"hydrogen",
			"oxygen",
			"nitrogen",
			"calcium",
			"iron",
			"aluminium",
			"uranium",
			"sodium",
			"chlorine",
			"light",
			"time",
			"silicon",
		}

		for _, kube := range kubes {
			if err := insertKube(kube, tx); err != nil {
				return err
			}
		}

		recipes := []recipe{
			{"hydrogen", "oxygen", "water"},
			{"water", "chlorine", "tap water"},
			{"sodium", "chlorine", "salt"},
			{"water", "salt", "sea water"},
			{"nitrogen", "oxygen", "air"},
			{"iron", "water", "rust"},
			{"silicon", "aluminium", "feldspar"},
			{"feldspar", "silicon", "sand"},
			{"sand", "water", "dirt"},
			{"sand", "sea water", "beach"},
			{"dirt", "water", "earth"},
			{"earth", "air", "life"},
			{"life", "time", "age"},
			{"uranium", "water", "energy"},
			{"sand", "time", "rock"},
			{"rock", "energy", "fire"},
			{"fire", "sand", "glass"},
		}

		for _, recipe := range recipes {
			if err := insertKubeRecipe(recipe.kube1, recipe.kube2, recipe.output, tx); err != nil {
				return err
			}
		}
		return nil
	}).Error

	if err != nil {
		log.Printf("Seeding database failed: %s", err)
	}

	var kubes, recipes int64
	d.Db.Model(&Kube{}).Count(&kubes)
	d.Db.Model(&KubeRecipe{}).Count(&recipes)

	log.Printf("There are %d kubes and %d recipes in the database", kubes, recipes)
	if kubes == 0 || recipes == 0 {
		log.Fatal("Seeding database failed - no kubes or recipes")
	}

	log.Println("Database successfully seeded")
}
