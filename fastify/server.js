import Fastify from 'fastify';
import { getSong, initS3Client, putSong } from './s3_connector.js';


const fastify = Fastify({
  logger: true,
  bodyLimit: 50 * 1024 * 1024,
})

const s3Client = initS3Client();

fastify.get('/', async (request, reply) => {
  return { hello: 'world' }
})

fastify.addContentTypeParser("audio/mpeg", { parseAs: "buffer" }, (req, body, done) => {
  done(null, body);
});

fastify.put('/song', async (request, reply) => {
  const artist = request.query.artist;
  const song = request.query.song;
  const extension = request.query.ext;
  const file = request.body;

  await putSong(s3Client, file, artist, song, extension);

  return { status: 'ok', artist, song };
});

fastify.get('/song', async (request, reply) => {
  const artist = request.query.artist;
  const song = request.query.song;
  const extension = request.query.ext ? request.query.ext : 'mp3';

  const { Body, ContentLength, ContentType } = await getSong(s3Client, artist, song, extension);

  const buffer = await streamToBuffer(Body);

  reply
    .header('Content-Type', ContentType || 'audio/mpeg')
    .header('Content-Length', ContentLength)
    .header('Content-Disposition', `attachment; filename="${song}.${extension}"`)
    .send(buffer);
});

async function streamToBuffer(stream) {
  const chunks = [];
  for await (const chunk of stream) {
    chunks.push(chunk);
  }
  return Buffer.concat(chunks);
}

try {
  await fastify.listen({ port: 3000, host: '0.0.0.0' })
  console.log('Server running at http://localhost:3000')
} catch (err) {
  fastify.log.error(err)
  process.exit(1)
}