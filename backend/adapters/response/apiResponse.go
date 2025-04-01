package response

type ApiResponse struct {
	Data    any    `json:"data"`
	Message string `json:"message"`
}

func NewApiResponse(data any, message string) ApiResponse {
	return ApiResponse{Data: data, Message: message}
}
