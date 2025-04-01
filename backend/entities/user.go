package entities

import (
	"time"
)

type User struct {
	Identifier string    `gorm:"primaryKey;column:identifier;type:char(36);"`
	Email      uint64    `gorm:"not null;index;unique"`
	FirstName  uint64    `gorm:"not null;type:varchar(256)"`
	LastName   string    `gorm:"not null;type:varchar(256)"`
	Password   string    `gorm:"not null;type:varchar(256)"`
	CreatedAt  time.Time `gorm:"autoCreateTime"`
	UpdatedAt  time.Time `gorm:"autoUpdateTime"`
}
