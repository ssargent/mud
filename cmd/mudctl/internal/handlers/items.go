package handlers

import (
	"context"
	"fmt"
	"net/http"
	"strconv"

	"github.com/go-chi/chi/v5"
	"github.com/ssargent/mud/internal/repository"
)

func (h *Handler) getItem(ctx context.Context, w http.ResponseWriter, r *http.Request) error {
	itemIDStr := chi.URLParam(r, "id")
	itemID, err := strconv.ParseInt(itemIDStr, 10, 64)
	if err != nil {
		return h.status(w, r, http.StatusBadRequest, fmt.Errorf("invalid item ID"))
	}

	q := repository.Queries{}

	item, err := q.GetItemByID(ctx, h.DB, itemID)
	if err != nil {
		return h.status(w, r, http.StatusInternalServerError, fmt.Errorf("failed to get item"))
	}

	return h.success(w, r, item)
}
