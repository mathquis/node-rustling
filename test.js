const Rustling = require('./lib/');

(async () => {
	const parser = new Rustling.Parser('FR')

	const text = 'quarante deux'

	console.log('Parsing number in "%s"', text)

	const result = await parser.parse_number(text)

	console.log(result)
})()