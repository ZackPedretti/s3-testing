package main

import (
	"bytes"
	"context"
	"io"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/gin-gonic/gin"
)

func initS3Client() (*s3.Client, error) {
	ctx := context.Background()
	sdkConfig, err := config.LoadDefaultConfig(ctx)
	if err != nil {
		return nil, err
	}
	s3Client := s3.NewFromConfig(sdkConfig)
	_, err = s3Client.ListBuckets(ctx, &s3.ListBucketsInput{})
	if err != nil {
		return nil, err
	}
	return s3Client, nil
}

func putSongToS3(ctx *gin.Context, file []byte, song Song, s3Client *s3.Client) error {
	_, err := s3Client.PutObject(ctx, &s3.PutObjectInput{
		Bucket: aws.String("jukebox"),
		Key:    aws.String(song.BuildKey()),
		Body:   bytes.NewReader(file),
	})
	if err != nil {
		return err
	}
	return nil
}

func getSongFromS3(ctx *gin.Context, song Song, s3Client *s3.Client) ([]byte, error) {
	result, err := s3Client.GetObject(ctx, &s3.GetObjectInput{
		Bucket: aws.String("jukebox"),
		Key:    aws.String(song.BuildKey()),
	})
	if err != nil {
		return nil, err
	}
	defer result.Body.Close()

	file, err := io.ReadAll(result.Body)
	if err != nil {
		return nil, err
	}

	return file, nil
}
