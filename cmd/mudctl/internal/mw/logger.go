// Inspired by https://github.com/treastech/logger
package mw

import (
	"context"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"go.uber.org/zap"
)

type contextKey struct {
	name string
}

var LogFieldsContextKey = &contextKey{"LogFields"}

type Values struct {
	LogFields []zap.Field
}

// Logger is a middleware that logs the start and end of each request, along
// with some useful data about what was requested, what the response status was,
// and how long it took to return.
func Logger(l *zap.Logger, appName string) func(next http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		fn := func(w http.ResponseWriter, r *http.Request) {
			ww := middleware.NewWrapResponseWriter(w, r.ProtoMajor)

			logFields := make([]zap.Field, 0)
			values := Values{LogFields: logFields}
			ctx := context.WithValue(r.Context(), LogFieldsContextKey, &values)

			r = r.WithContext(ctx)
			t1 := time.Now()
			defer func() {
				ctx := r.Context()
				logFields := make([]zap.Field, 0)

				rctx := chi.RouteContext(r.Context())
				origRoutPattern := strings.Join(rctx.RoutePatterns, "")
				routePattern := strings.Replace(origRoutPattern, "/*/*/", "/", -1)

				values, ok := ctx.Value(LogFieldsContextKey).(*Values)
				if ok {
					for _, l := range values.LogFields {
						logFields = append(logFields, l)
					}
				}

				hostName, _ := os.Hostname()

				logFields = append(logFields, zap.String("service", appName))
				logFields = append(logFields, zap.String("host", hostName))
				logFields = append(logFields, zap.String("proto", r.Proto))
				logFields = append(logFields, zap.String("path", r.URL.Path))
				logFields = append(logFields, zap.Duration("request_time_ms", (time.Since(t1)/time.Millisecond)))
				logFields = append(logFields, zap.Int("status", ww.Status()))
				logFields = append(logFields, zap.Int("size", ww.BytesWritten()))
				logFields = append(logFields, zap.String("route_pattern", routePattern))
				logFields = append(logFields, zap.String("reqId", middleware.GetReqID(ctx)))

				l.Info("Served",
					logFields...,
				)
			}()

			next.ServeHTTP(ww, r)
		}
		return http.HandlerFunc(fn)
	}
}

func add(fields []zap.Field, field zap.Field) []zap.Field {
	return append(fields, field)
}

func AddLogFields(ctx context.Context, fields ...zap.Field) bool {
	values, ok := ctx.Value(LogFieldsContextKey).(*Values)
	if !ok {
		return false
	}

	for _, l := range fields {
		values.LogFields = append(values.LogFields, l)
	}

	return true
}
