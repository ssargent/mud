package config

type Config struct {
	Port     int
	Database DatabaseConfig
}

type DatabaseConfig struct {
	Driver   string `default:"postgres" split_words:"true" json:"driver,omitempty"`
	Username string `default:"wb" split_words:"true"  json:"username,omitempty"`
	Password string `required:"true" split_words:"true" json:"password,omitempty"`
	Server   string `default:"localhost" split_words:"true" json:"server,omitempty"`
	Name     string `default:"world-builder" split_words:"true" json:"name,omitempty"`
}
