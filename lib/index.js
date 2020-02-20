const Rustling = require('../native/index.node')

// Rustling OutputKind
const Kinds = {
	NUMBER			: 'Number',
	ORDINAL			: 'Ordinal',
	DATETIME		: 'Datetime',
	DATE			: 'Date',
	TIME			: 'Time',
	DATE_PERIOD		: 'DatePeriod',
	TIME_PERIOD		: 'TimePeriod',
	AMOUNT_OF_MONEY	: 'AmountOfMoney',
	TEMPERATURE		: 'Temperature',
	DURATION		: 'Duration',
	PERCENTAGE		: 'Percentage'
}

class RustlingParser {
	constructor(language) {
		this.language = language
		this._parser = new Rustling.Parser(language)
	}
	parse(query, expectedKinds) {
		( expectedKinds || (expectedKinds = []) )
		const outputKinds = Object.values(Kinds)
		const unknownKind = expectedKinds.find(k => outputKinds.indexOf(k) < 0)
		if ( unknownKind ) throw new ReferenceError('Unknown kind "' + unknownKind + '"')
		return this._parser.parse(query, expectedKinds)
	}
	parse_number(query) {
		return this.parse(query, [Kinds.NUMBER])
	}
	parse_ordinal(query) {
		return this.parse(query, [Kinds.ORDINAL])
	}
	parse_percentage(query) {
		return this.parse(query, [Kinds.PERCENTAGE])
	}
	parse_duration(query) {
		return this.parse(query, [Kinds.DURATION])
	}
	parse_amount_of_money(query) {
		return this.parse(query, [Kinds.AMOUNT_OF_MONEY])
	}
	parse_datetime(query) {
		return this.parse(query, [Kinds.DATETIME])
	}
	parse_date(query) {
		return this.parse(query, [Kinds.DATE])
	}
	parse_time(query) {
		return this.parse(query, [Kinds.TIME])
	}
	parse_date_period(query) {
		return this.parse(query, [Kinds.DATE_PERIOD])
	}
	parse_time_period(query) {
		return this.parse(query, [Kinds.TIME_PERIOD])
	}
}

module.exports = {
	Parser: RustlingParser,
	Kinds
}