package repositories

import (
	"log"
	"strings"

	"com.pomodoro.app/adapters/dto"
	"com.pomodoro.app/entities"
	"github.com/matthewhartstonge/argon2"
	"github.com/oklog/ulid/v2"
	"gorm.io/gorm"
)

type UserRepository struct {
	Db *gorm.DB
}

func NewUserUserRepository(Db gorm.DB) *UserRepository {
	return &UserRepository{
		Db: &Db,
	}
}

func (r *UserRepository) FindOneByEmail(email string) entities.User {
	var user entities.User
	r.Db.Where("email = ?", strings.TrimSpace(email)).First(&user)

	return user
}

func (r *UserRepository) FindOneByIdentifier(identifier string) (entities.User, error) {
	var user entities.User
	err := r.Db.Where("identifier = ?", strings.TrimSpace(identifier)).First(&user).Error

	if err != nil {
		log.Fatal(err)
		return entities.User{}, err
	}

	return user, nil
}

func (r *UserRepository) Create(payload dto.CreateUserDto) (entities.User, error) {
	argon := argon2.DefaultConfig()
	encoded, err := argon.HashEncoded([]byte(strings.TrimSpace(payload.Password)))

	if err != nil {
		return entities.User{}, err
	}
	user := entities.User{
		Identifier: ulid.Make().String(),
		Email:      payload.Email,
		FirstName:  payload.FirstName,
		LastName:   strings.TrimSpace(payload.LastName),
		Password:   string(encoded),
	}

	result := r.Db.Create(&user)

	if result.Error != nil {
		return entities.User{}, result.Error
	}

	return r.FindOneByEmail(payload.Email), nil
}

func (r *UserRepository) Delete(identifier string) error {
	user, err := r.FindOneByIdentifier(identifier)
	if err != nil {
		return err
	}

	return r.Db.Where("identifier = ?", identifier).Delete(user).Error
}
