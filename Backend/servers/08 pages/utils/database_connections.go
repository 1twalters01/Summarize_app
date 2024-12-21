// go get github.com/jackc/pgx/v5
// go get github.com/redis/go-redis/v9

package utils

import (
	"os"
	"strings"
	"githum.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

func create_pg_pool_connection() (*pgxpool.Pool, error) {
	url := os.Getenv("PG_URL")
	pool, err := pgxpool.New(ctx, url)
	if err != nil {
		log.Fatalf("Unable to connect to database: %v", err)
	}
	return pool, nil
}

// CREATE
func createUser(ctx context.Context, pool *pgxpool.Pool, name, email string) {
	query := `INSERT INTO users (name, email) VALUES ($1, $2)`
	_, err := pool.Exec(ctx, query, name, email)
	if err != nil {
		log.Fatalf("Unable to insert user: %v", err)
	}
	fmt.Println("User created successfully")
}

// READ
func readUsers(ctx context.Context, pool *pgxpool.Pool) {
	query := `SELECT id, name, email FROM users`
	rows, err := pool.Query(ctx, query)
	if err != nil {
		log.Fatalf("Unable to read users: %v", err)
	}
	defer rows.Close()

	for rows.Next() {
		var id int
		var name, email string
		if err := rows.Scan(&id, &name, &email); err != nil {
			log.Fatalf("Unable to scan row: %v", err)
		}
		fmt.Printf("ID: %d, Name: %s, Email: %s\n", id, name, email)
	}
}

// UPDATE
func updateUser(ctx context.Context, pool *pgxpool.Pool, id int, newName string) {
	query := `UPDATE users SET name = $1 WHERE id = $2`
	_, err := pool.Exec(ctx, query, newName, id)
	if err != nil {
		log.Fatalf("Unable to update user: %v", err)
	}
	fmt.Println("User updated successfully")
}

// DELETE
func deleteUser(ctx context.Context, pool *pgxpool.Pool, id int) {
	query := `DELETE FROM users WHERE id = $1`
	_, err := pool.Exec(ctx, query, id)
	if err != nil {
		log.Fatalf("Unable to delete user: %v", err)
	}
	fmt.Println("User deleted successfully")
}


// Function to initialize the Redis client
func initRedis(ctx context.Context, addr, password string) (*redis.Client, error) {
	client := redis.NewClient(&redis.Options{
		Addr:     addr,     // Redis server address
		Password: password, // Empty string for no password
		DB:       0,        // Default DB
	})

	// Test connection
	_, err := client.Ping(ctx).Result()
	if err != nil {
		return nil, fmt.Errorf("unable to connect to Redis: %w", err)
	}

	fmt.Println("Redis connection established")
	return client, nil
}

// CREATE (Set a key-value pair)
func setValue(ctx context.Context, client *redis.Client, key, value string) {
	err := client.Set(ctx, key, value, 0).Err() // 0 means no expiration
	if err != nil {
		log.Fatalf("Unable to set key: %v", err)
	}
	fmt.Printf("Key '%s' set successfully\n", key)
}

// READ (Get the value for a key)
func getValue(ctx context.Context, client *redis.Client, key string) {
	value, err := client.Get(ctx, key).Result()
	if err == redis.Nil {
		fmt.Printf("Key '%s' does not exist\n", key)
		return
	} else if err != nil {
		log.Fatalf("Unable to get key: %v", err)
	}

	fmt.Printf("Key '%s' has value: %s\n", key, value)
}

// UPDATE (Update the value for a key)
func updateValue(ctx context.Context, client *redis.Client, key, newValue string) {
	err := client.Set(ctx, key, newValue, 0).Err() // 0 means no expiration
	if err != nil {
		log.Fatalf("Unable to update key: %v", err)
	}
	fmt.Printf("Key '%s' updated successfully\n", key)
}

// DELETE (Delete a key)
func deleteValue(ctx context.Context, client *redis.Client, key string) {
	err := client.Del(ctx, key).Err()
	if err != nil {
		log.Fatalf("Unable to delete key: %v", err)
	}
	fmt.Printf("Key '%s' deleted successfully\n", key)
}