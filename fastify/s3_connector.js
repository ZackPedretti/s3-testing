import {
  S3Client,
  PutObjectCommand,
  DeleteObjectCommand,
  paginateListObjectsV2,
  GetObjectCommand,
} from "@aws-sdk/client-s3";

export function initS3Client(){
    return new S3Client({
        region: "us-east-1",
        endpoint: process.env.MINIO_ENDPOINT,
        credentials: {
            accessKeyId: process.env.MINIO_FASTIFY_USER,
            secretAccessKey: process.env.MINIO_FASTIFY_PASSWORD,
        },
        forcePathStyle: true,
    });
}

export async function putSong(s3Client, file, artist, song, extension) {
   const ext = extension ? extension : 'mp3' 
   await s3Client.send(
    new PutObjectCommand({
        Bucket: "jukebox",
        Key: `${artist}/${song}.${ext}`,
        Body: file
    })
   );
}

export async function getSong(s3Client, artist, song, extension) {
   const ext = extension ? extension : 'mp3' 
   return await s3Client.send(
    new GetObjectCommand({
        Bucket: "jukebox",
        Key: `${artist}/${song}.${ext}`,
    })
   ); 
}