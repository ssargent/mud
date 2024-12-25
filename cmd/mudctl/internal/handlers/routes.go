package handlers

import (
	"context"
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
)

func (h *Handler) Routes() *chi.Mux {
	r := chi.NewRouter()

	Route(r, "GET", "/status", h.getStatus)

	return r
}

type HTTPHandler func(ctx context.Context, w http.ResponseWriter, r *http.Request) error

func Route(r chi.Router, method, pattern string, h HTTPHandler, mw ...func(http.Handler) http.Handler) {
	if len(mw) > 0 {
		r.With(mw...)
	}
	fn := func(w http.ResponseWriter, r *http.Request) {
		ctx := r.Context()
		if err := h(ctx, w, r); err != nil {
			fmt.Println(err)
		}
	}

	r.MethodFunc(method, pattern, fn)
}
