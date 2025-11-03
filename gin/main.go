package main

import (
	"fmt"
	"net/http"

	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()

	s3Client, err := initS3Client()
	if err != nil {
		panic(err)
	}

	router.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	router.PUT("/song", func(ctx *gin.Context) {
		putSong(ctx, s3Client)
	})
	router.GET("/song", func(ctx *gin.Context) {
		getSong(ctx, s3Client)
	})
	router.Run(":3002")
}

func putSong(ctx *gin.Context, s3Client *s3.Client) {
	var song Song

	if err := ctx.ShouldBindQuery(&song); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if song.Extension == nil {
		ext := "mp3"
		song.Extension = &ext
	}

	file, err := ctx.GetRawData()
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	err = putSongToS3(ctx, file, song, s3Client)
	if err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	ctx.Status(http.StatusOK)
}

func getSong(ctx *gin.Context, s3Client *s3.Client) {
	var song Song

	if err := ctx.ShouldBindQuery(&song); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if song.Extension == nil {
		ext := "mp3"
		song.Extension = &ext
	}

	file, err := getSongFromS3(ctx, song, s3Client)
	if err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	ctx.Header("Content-Disposition", fmt.Sprintf("attachment; filename=\"%s.%s\"", song.Name, *song.Extension))
	ctx.Data(http.StatusOK, "audio/mpeg", file)
}
