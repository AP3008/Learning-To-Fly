package store

import (
	"context"
	"fmt"
	"time"

	"github.com/go-redis/redis"
	"github.com/redis/go-redis"
)

type StoreService struct {
	redisClient *redis.Client
}

var (
	storeService = &StoreService{}
	ctx          = context.Background()
)

const cacheDuration = 6 * time.Hour

func InitializeStore() *StoreService {
	redisClient := redis.NewClient(&redis.Options{
		Addr:     "localhost:6379",
		Password: "",
		DB:       0,
	})

	pong, err := redisClient.Ping(ctx).Result()
	if err != nil {
		panic(fmt.Sprintf("Error init Redis: %v", err )) 
	}

	fmt.Printf("\nRedis Started successfully: pong message = {%s}", pong)
	storeService.redisClient = redisClient
	return storeService
}
