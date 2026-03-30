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
	ctx = context.Background()
)

const cacheDuration = 6 * time.Hour
