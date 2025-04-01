package config

import (
	"fmt"
	"log"

	"com.pomodoro.app/entities"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func LoadDatabase() *gorm.DB {
	env := ExtractEnv()

	dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s",
		env.Database.Host,
		env.Database.User,
		env.Database.Password,
		env.Database.Name,
		env.Database.Port,
	)

	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if err != nil {
		log.Fatal(err.Error())
	} else {
		log.Println("Successfully connected to database")
	}

	runDatabaseMigrations(db)
	return db
}

func runDatabaseMigrations(db *gorm.DB) {
	err := db.AutoMigrate(&entities.User{})
	if err != nil {
		log.Fatalln(err.Error())
	}
}
