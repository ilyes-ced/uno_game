import fastify from 'fastify'
const server = fastify()





interface IQuerystring {
    username: string;
    password: string;
}

interface IHeaders {
    'h-Custom': string;
}

server.get('/fffff', async (request, reply) => {
	return '11fzadazdazdzzadazdf1\n'
})

server.get('/ping', async (request, reply) => {
  	return 'pong\n'
})



server.get<{
	Querystring: IQuerystring,
	Headers: IHeaders
  }>('/auth', {
	preValidation: (request, reply, done) => {
	  const { username, password } = request.query
	  done(username !== 'admin' ? new Error('Must be admin') : undefined) // only validate `admin` account
	}
  }, async (request, reply) => {
	const customerHeader = request.headers['h-Custom']
	// do something with request data
	return `logged in!`
  })




server.listen({ port: 3001 }, (err, address) => {
  	if (err) {
  	  	console.error(err)
  	  	process.exit(1)
  	}
  	console.log(`Server listening at ${address}`)
})