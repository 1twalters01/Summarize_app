// Test configuration
package config

import (
	"log"
	"os"
	"github.com/joho/godotenv"
)

func LoadConfig() {
	err := godotenv.Load()
	if err != nil {
		log.Println("No .env file found, using system environment variables")
	}
	log.Println("Configuration loaded")
}