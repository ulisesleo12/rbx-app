// usage: npm run aker-add-robot -- robot_add --name K3_L01_1 --path K3_L01_1

// receive arguments and turn them into json variables
const argv = require('minimist')(process.argv.slice(2))
const grapqql_operation = argv._[0]
console.log("grapqql_operation", grapqql_operation)

var variables = argv
delete variables._
Object.keys(variables).forEach((key) => {
  variables[key] = String(variables[key])
})
console.log("variables", variables)

// load config file: .graphqlrc
const { loadConfigSync } = require('graphql-config')
const config = loadConfigSync({})
const api = config.getDefault().extension("endpoints")["dev"]
console.log("api", api)

// find graphql operation inside config
const jp = require('jsonpath')
let docs = config.getDefault().getDocumentsSync()
let operation = jp.query(docs, '$[*].document.definitions[?(@.name.value == "' + grapqql_operation + '")]')[0]
// console.log("operation", operation);

// process graphql request
const { request, gql, GraphQLClient } = require('graphql-request')
async function main() {
  const endpoint = api.url
  const graphQLClient = new GraphQLClient(endpoint, { headers: api.headers })
  const data = await graphQLClient.request(operation, variables)
  console.log('Response', JSON.stringify(data, undefined, 2))
}
main().catch((error) => console.error(error))

