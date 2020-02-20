# node-rustling

Node.js bindings for Snips rustling-ontology parser.

## Supported Output

|   Output  | OutputKind |
| --------- | ------------- |
|  Integer |  Number |
| Float | Number |
| Ordinal | Ordinal |
| Temperature | Temperature |
| Time | Time |
| TimeInterval | Time |
| AmountOfMoney | AmountOfMoney |
| Duration | Duration |

## Get started

### Install rust

```bash
curl https://sh.rustup.rs -sSf | sh
```

### Usage

```javascript
const Rustling = require('@mathquis/node-rustling')

const Parser = new Rustling.Parser()

let result

result = Parser.parse("trente deux", ["Number"])
console.log(result)
// Outputs: [ { kind: 'Number', value: 32 } ]

result = parser.parse("samedi prochain à cinq heures du matin et trente sept minutes")
console.log(result)
/* Outputs: [ { kind: 'InstantTime',
    value: '2020-02-01 05:37:00',
    grain: 'Minute',
    precision: 'Exact' } ]
*/
```