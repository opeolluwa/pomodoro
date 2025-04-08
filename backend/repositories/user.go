package repositories

import (
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
