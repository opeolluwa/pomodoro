package entities

import (
	"time"
)

type User struct {
	Identifier string    `gorm:"primaryKey;column:identifier;type:char(36);"`
	Email      string    `gorm:"not null;index;unique"`
	FirstName  string    `gorm:"not null;type:varchar(256)"`
	LastName   string    `gorm:"not null;type:varchar(256)"`
	Password   string    `gorm:"not null;type:varchar(256)"`
	CreatedAt  time.Time `gorm:"autoCreateTime"`
	UpdatedAt  time.Time `gorm:"autoUpdateTime"`
}
