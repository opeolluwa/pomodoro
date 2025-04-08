package dto

type CreateUserDto struct {
	FirstName string `json:"firstName"`
	LastName  string `json:"lastName"`
	Email     string `json:"email"`
	Password  string `json:"password"`
}

type LoginUserDto struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}
