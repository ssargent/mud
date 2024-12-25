package internal

import (
	"fmt"
	"log/slog"
	"net/http"
	"strings"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"github.com/go-chi/httplog/v2"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/ssargent/mud/cmd/mudctl/internal/config"
	"github.com/ssargent/mud/internal/accounts/repository"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"
)

type API struct {
	cfg    *config.Config
	Reader repository.DBTX
	Writer repository.DBTX
}

func NewAPI(cfg *config.Config, rdb, wdb *pgxpool.Pool) *API {
	return &API{
		cfg:    cfg,
		Reader: rdb,
		Writer: wdb,
	}
}

func (a *API) ListenAndServe() error {
	// Logger
	logger := httplog.NewLogger("mudctl-logger", httplog.Options{
		// JSON:             true,
		LogLevel:         slog.LevelDebug,
		Concise:          true,
		RequestHeaders:   true,
		MessageFieldName: "message",
		// TimeFieldFormat: time.RFC850,
		Tags: map[string]string{
			"version": "v1.0-81aa4244d9fc8076a",
			"env":     "dev",
		},
		QuietDownRoutes: []string{
			"/",
			"/ping",
		},
		QuietDownPeriod: 10 * time.Second,
		// SourceFieldName: "source",
	})
	r := chi.NewRouter()
	//logger := configureLogger()

	//r.Use(httplog.RequestLogger(logger))
	r.Use(middleware.RequestID)
	//r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(cors.Handler(cors.Options{
		// AllowedOrigins:   []string{"https://foo.com"}, // Use this to allow specific origin hosts
		AllowedOrigins: []string{"https://*", "http://*"},
		// AllowOriginFunc:  func(r *http.Request, origin string) bool { return true },
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"*"},
		ExposedHeaders:   []string{"*"},
		AllowCredentials: false,
		MaxAge:           300, // Maximum value not ignored by any of major browsers
	}))

	r.Use(httplog.RequestLogger(logger))
	/*host, err := os.Hostname()
	if err != nil {
		host = "unknown"
	}

	logManager := logs.NewLogManager(*slog.New(slog.NewJSONHandler(os.Stdout, nil)).With(
		slog.String("service", "entitlements"),
		slog.String("version", "1.0.0"), slog.String("env", "dev"),
		slog.String("host", host),
	))
	*/
	walkFunc := func(method string,
		route string,
		handler http.Handler,
		middlewares ...func(http.Handler) http.Handler) error {
		route = strings.ReplaceAll(route, "/*/", "/")
		fmt.Printf("%s %s\n", method, route)
		return nil
	}

	if err := chi.Walk(r, walkFunc); err != nil {
		fmt.Printf("Logging err: %s\n", err.Error())
	}

	h2s := &http2.Server{}
	srv := &http.Server{
		Addr:    fmt.Sprintf("0.0.0.0:%d", a.cfg.Port),
		Handler: h2c.NewHandler(r, h2s),
	}

	return srv.ListenAndServe()
}

func configureLogger() *httplog.Logger {
	logger := httplog.NewLogger("entitlements", httplog.Options{
		JSON:             true,
		LogLevel:         slog.LevelDebug,
		Concise:          true,
		RequestHeaders:   true,
		MessageFieldName: "message",
		Tags: map[string]string{
			"version": "1.0.0",
			"env":     "dev",
		},
		QuietDownRoutes: []string{
			"/",
			"/ping",
		},
		QuietDownPeriod: 10 * time.Second,
	})

	return logger
}
