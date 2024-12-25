package handlers

import (
	"context"
	"net/http"
	"os"
	"strings"

	"go.uber.org/zap"

	"github.com/go-chi/jwtauth/v5"
	"github.com/go-chi/render"
	"github.com/jmoiron/sqlx"
)

type Handler struct {
	DB *sqlx.DB

	Logger    *zap.Logger
	TokenAuth *jwtauth.JWTAuth
}

// ErrResponse renderer type for handling all sorts of errors.
//
// In the best case scenario, the excellent github.com/pkg/errors package
// helps reveal information on the error, setting it on Err, and in the Render()
// method, using it to set the application-specific error code in AppCode.
type ErrResponse struct {
	Err            error  `json:"-"` // low-level runtime error
	HTTPStatusCode int    `json:"http_status_code,omitempty"`
	StackTrace     string `json:"stack_trace,omitempty"`
	StatusText     string `json:"status,omitempty"` // user-level status message
	AppCode        int64  `json:"code,omitempty"`   // application-specific error code
	ErrorText      string `json:"error,omitempty"`  // application-level error message, for debugging
}

func (e *ErrResponse) Error() string {
	return e.Err.Error()
}

// Render  renders rendery things to render wtse-1
func (e *ErrResponse) Render(w http.ResponseWriter, r *http.Request) error {
	w.Header().Set("Content-Type", "application/json")
	render.Status(r, e.HTTPStatusCode)
	render.JSON(w, r, e)
	return nil
}

func (h *Handler) text(w http.ResponseWriter, r *http.Request, data []byte) error {
	w.Header().Set("Content-Type", "text/plain")
	render.Status(r, 200)
	w.Write(data)
	return nil
}

func (h *Handler) success(w http.ResponseWriter, r *http.Request, v interface{}) error {
	render.JSON(w, r, v)
	return nil
}

func (h *Handler) status(w http.ResponseWriter, r *http.Request, status int, err error) error {
	response := ErrResponse{
		Err:            err,
		HTTPStatusCode: status,
		StatusText:     http.StatusText(status),
		ErrorText:      err.Error(),
	}

	response.Render(w, r)
	return &response
}

type Environment struct {
	EnvironmentVariables map[string]string `json:"environment_variables,omitempty"`
}

func (h *Handler) getEnvironment(ctx context.Context, w http.ResponseWriter, r *http.Request) error {

	envMap := make(map[string]string)

	for _, e := range os.Environ() {
		pair := strings.SplitN(e, "=", 2)
		envMap[pair[0]] = pair[1]
	}

	env := Environment{
		EnvironmentVariables: envMap,
	}

	return h.success(w, r, env)
}

type ApiResponse struct {
	Data   interface{} `json:"data,omitempty"`
	Status string      `json:"status,omitempty"`
	Code   int         `json:"code,omitempty"`
}
