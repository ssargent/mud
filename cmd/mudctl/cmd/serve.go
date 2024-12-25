/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"context"
	"fmt"
	"os"
	"strings"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/joho/godotenv"
	"github.com/kelseyhightower/envconfig"
	"github.com/spf13/cobra"

	"github.com/ssargent/mud/cmd/mudctl/internal"
	"github.com/ssargent/mud/cmd/mudctl/internal/config"
)

type serveCmd struct {
	serveEnvFile  string
	explainConfig bool
}

func (s *serveCmd) server() (*internal.API, error) {
	if _, err := os.Stat(s.serveEnvFile); err == nil {
		if err := godotenv.Load(s.serveEnvFile); err != nil {
			return nil, fmt.Errorf("godotenv.Load: %w", err)
		}
	}

	var cfg config.Config
	if err := envconfig.Process("mud", &cfg); err != nil {
		return nil, fmt.Errorf("envconfig.Process: %w", err)
	}

	if s.explainConfig {
		s.explain(&cfg)
	}
	db, safeDb, err := s.database(&cfg)
	if err != nil {
		return nil, fmt.Errorf("database: %w", err)
	}

	fmt.Printf("database connection established %s\n", safeDb)

	return internal.NewAPI(&cfg, db, db), nil
}

// database is a helper function that creates the database connection
func (s *serveCmd) database(cfg *config.Config) (*pgxpool.Pool, string, error) {
	dbURISafe := fmt.Sprintf("postgres://%s:xxxxxxxxxxx@%s/%s?sslmode=disable",
		cfg.Database.Username,
		cfg.Database.Server,
		cfg.Database.Name)
	dbURI := fmt.Sprintf("postgres://%s:%s@%s/%s?sslmode=disable",
		cfg.Database.Username,
		strings.TrimSpace(cfg.Database.Password),
		cfg.Database.Server,
		cfg.Database.Name)

	pool, err := pgxpool.New(context.Background(), dbURI)

	return pool, dbURISafe, err
}

// explain is a helper function that prints the configuration
func (s *serveCmd) explain(cfg *config.Config) {
	fmt.Println("-----------------")
	fmt.Println("Entitlement Server Configured Parameters")
	fmt.Println("-----------------")

	fmt.Printf("Config.Port := %d\n", cfg.Port)
	fmt.Printf("Config.Database.Driver := %q\n", cfg.Database.Driver)
	fmt.Printf("Config.Database.Name := %q\n", cfg.Database.Name)
	fmt.Printf("Config.Database.Username := %q\n", cfg.Database.Username)
	fmt.Printf("Config.Database.Server := %q\n", cfg.Database.Server)
	fmt.Println("-----------------")
}

func (s *serveCmd) runE(cmd *cobra.Command, args []string) error {
	api, err := s.server()
	if err != nil {
		return fmt.Errorf("server: %w", err)
	}

	if s.explainConfig {
		return envconfig.Usage("mud", &config.Config{})
	}

	return api.ListenAndServe()
}

func init() {
	s := &serveCmd{}
	cmd := &cobra.Command{
		Use:   "serve",
		Short: "starts the mud service",
		Long:  `starts the mud service`,
		RunE:  s.runE,
	}

	rootCmd.AddCommand(cmd)

	flags := cmd.Flags()
	flags.StringVarP(&s.serveEnvFile, "environment-file", "e", ".env", "Environment file to load")
	flags.BoolVarP(&s.explainConfig, "explain-config", "x", false, "Explain the configuration")
}
