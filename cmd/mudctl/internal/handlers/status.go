package handlers

import (
	"context"
	"net/http"
)

func (h *Handler) getStatus(ctx context.Context, w http.ResponseWriter, r *http.Request) error {
	status := struct {
		service       string `json:"service,omitempty"`
		serviceStatus string `json:"service_status,omitempty"`
	}{
		service:       "content service",
		serviceStatus: "running",
	}

	return h.success(w, r, status)
}
